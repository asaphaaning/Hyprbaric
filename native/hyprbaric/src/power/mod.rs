//! Battery telemetry and power-profile control.
//!
//! [`Power`] reads battery data from UPower and selects profiles through
//! power-profiles-daemon. Both are D-Bus boundaries accessed directly with
//! [`zbus`]; no helper CLI is spawned for normal operation.

mod domain;
mod signal;

use std::{collections::HashMap, sync::Arc, time::Duration};

use serde::Deserialize;
use tokio::{
    sync::{Mutex, broadcast},
    time::{MissedTickBehavior, interval},
};
use tracing::instrument;
use zbus::{Connection, Proxy, zvariant::OwnedValue};

use crate::config::Cadence;

use self::domain::{Command as PublicCommand, Percent};

pub use domain::{Battery, BatteryState, Command, Profile, Profiles, Report, Snapshot};

/// Shared power runtime handle.
pub type Handle = Arc<Power>;

const UPOWER_DESTINATION: &str = "org.freedesktop.UPower";
const UPOWER_DISPLAY_DEVICE_PATH: &str = "/org/freedesktop/UPower/devices/DisplayDevice";
const UPOWER_DEVICE_INTERFACE: &str = "org.freedesktop.UPower.Device";
const PROFILES_DESTINATION: &str = "net.hadess.PowerProfiles";
const PROFILES_PATH: &str = "/net/hadess/PowerProfiles";
const PROFILES_INTERFACE: &str = "net.hadess.PowerProfiles";

/// Battery and power-profile polling policy.
///
/// ```toml
/// [power]
/// refresh_interval = "15s"
/// ```
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    refresh_interval: Cadence,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            refresh_interval: Cadence::seconds(15),
        }
    }
}

/// Live battery and power-profile state.
pub struct Power {
    events: broadcast::Sender<Snapshot>,
    results: broadcast::Sender<Report>,
    client: Option<Client>,
    latest: Mutex<Snapshot>,
}

#[derive(Clone)]
struct Client {
    connection: Connection,
}

impl Power {
    /// Bootstraps power state and starts background refreshes.
    #[instrument(skip_all)]
    pub async fn bootstrap(config: &Configuration) -> (Handle, Snapshot) {
        let client = match Client::new().await {
            Ok(client) => Some(client),
            Err(error) => {
                tracing::warn!("Power bootstrap failed: {error}");
                None
            }
        };
        let initial_snapshot = match &client {
            Some(client) => client.read_snapshot().await,
            None => Snapshot::unavailable("Power services are unavailable"),
        };

        let (events, _) = broadcast::channel(16);
        let (results, _) = broadcast::channel(8);
        let power = Arc::new(Self {
            events,
            results,
            client,
            latest: Mutex::new(initial_snapshot.clone()),
        });
        spawn_poll(Arc::clone(&power), config.refresh_interval.duration());

        (power, initial_snapshot)
    }

    /// Subscribes to power [`Snapshot`] changes.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Subscribes to profile command [`Report`] values.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    /// Selects a power profile.
    #[instrument(skip(self))]
    pub async fn set_profile(&self, profile: Profile) {
        let command = PublicCommand::SetProfile { profile };
        let result = self.set_profile_inner(profile).await;
        self.send_report(command, result);
        self.refresh().await;
    }

    async fn set_profile_inner(&self, profile: Profile) -> Result<(), Error> {
        let Some(client) = &self.client else {
            return Err(Error::Unavailable);
        };

        let snapshot = self.latest.lock().await.clone();
        if !snapshot.profiles.contains(profile) {
            return Err(Error::ProfileUnavailable { profile });
        }

        client.set_profile(profile).await
    }

    #[instrument(skip(self))]
    async fn refresh(&self) {
        let snapshot = match &self.client {
            Some(client) => client.read_snapshot().await,
            None => Snapshot::unavailable("Power services are unavailable"),
        };
        let mut latest = self.latest.lock().await;
        if *latest == snapshot {
            return;
        }
        *latest = snapshot.clone();
        drop(self.events.send(snapshot));
    }

    fn send_report(&self, command: PublicCommand, result: Result<(), Error>) {
        let report = match result {
            Ok(()) => Report::Started(command),
            Err(error) => Report::Failed {
                command,
                message: error.to_string(),
            },
        };
        drop(self.results.send(report));
    }
}

impl Client {
    #[instrument(err)]
    async fn new() -> Result<Self, Error> {
        Ok(Self {
            connection: Connection::system()
                .await
                .map_err(Error::ConnectSystemBus)?,
        })
    }

    #[instrument(skip(self))]
    async fn read_snapshot(&self) -> Snapshot {
        let battery = self.read_battery().await.unwrap_or_else(|error| {
            tracing::warn!("Battery read failed: {error}");
            Battery::Missing {
                message: Some(error.to_string()),
            }
        });
        let profiles = self.read_profiles().await.unwrap_or_else(|error| {
            tracing::warn!("Power profile read failed: {error}");
            Profiles::Unavailable {
                message: error.to_string(),
            }
        });

        Snapshot { battery, profiles }
    }

    async fn upower_proxy(&self) -> Result<Proxy<'_>, Error> {
        Proxy::new(
            &self.connection,
            UPOWER_DESTINATION,
            UPOWER_DISPLAY_DEVICE_PATH,
            UPOWER_DEVICE_INTERFACE,
        )
        .await
        .map_err(Error::CreateUpowerProxy)
    }

    async fn profiles_proxy(&self) -> Result<Proxy<'_>, Error> {
        Proxy::new(
            &self.connection,
            PROFILES_DESTINATION,
            PROFILES_PATH,
            PROFILES_INTERFACE,
        )
        .await
        .map_err(Error::CreateProfilesProxy)
    }

    #[instrument(skip(self), err)]
    async fn read_battery(&self) -> Result<Battery, Error> {
        let proxy = self.upower_proxy().await?;
        let is_present = proxy
            .get_property::<bool>("IsPresent")
            .await
            .map_err(Error::ReadBattery)?;

        if !is_present {
            return Ok(Battery::Missing { message: None });
        }

        let percent = proxy
            .get_property::<f64>("Percentage")
            .await
            .map(Percent::new)
            .map_err(Error::ReadBattery)?;
        let state = proxy
            .get_property::<u32>("State")
            .await
            .map(BatteryState::from_upower)
            .map_err(Error::ReadBattery)?;
        let time_to_empty = proxy
            .get_property::<i64>("TimeToEmpty")
            .await
            .map_err(Error::ReadBattery)?;
        let time_to_full = proxy
            .get_property::<i64>("TimeToFull")
            .await
            .map_err(Error::ReadBattery)?;
        let energy_rate = proxy
            .get_property::<f64>("EnergyRate")
            .await
            .map_err(Error::ReadBattery)?;
        let voltage = proxy
            .get_property::<f64>("Voltage")
            .await
            .map_err(Error::ReadBattery)?;
        let temperature = proxy
            .get_property::<f64>("Temperature")
            .await
            .map_err(Error::ReadBattery)?;

        Ok(Battery::Present {
            percent,
            state,
            remaining: remaining_duration(state, time_to_empty, time_to_full),
            power_rate_watts: telemetry_value(energy_rate)
                .map(|rate| if state.is_discharging() { -rate } else { rate }),
            voltage: telemetry_value(voltage),
            temperature_celsius: telemetry_value(temperature),
        })
    }

    #[instrument(skip(self), err)]
    async fn read_profiles(&self) -> Result<Profiles, Error> {
        let proxy = self.profiles_proxy().await?;
        let active = proxy
            .get_property::<String>("ActiveProfile")
            .await
            .map_err(Error::ReadProfiles)
            .map(|value| Profile::from_backend(&value))?;
        let raw_profiles = proxy
            .get_property::<Vec<HashMap<String, OwnedValue>>>("Profiles")
            .await
            .map_err(Error::ReadProfiles)?;
        let available = raw_profiles
            .iter()
            .filter_map(profile_from_map)
            .fold(Vec::new(), push_unique_profile);
        let degraded = proxy
            .get_property::<String>("PerformanceDegraded")
            .await
            .map_err(Error::ReadProfiles)
            .map(empty_to_none)?;
        let inhibited = proxy
            .get_property::<String>("PerformanceInhibited")
            .await
            .map_err(Error::ReadProfiles)
            .map(empty_to_none)?;

        Ok(Profiles::Available {
            active,
            available,
            degraded,
            inhibited,
        })
    }

    #[instrument(skip(self), err)]
    async fn set_profile(&self, profile: Profile) -> Result<(), Error> {
        let proxy = self.profiles_proxy().await?;
        proxy
            .set_property("ActiveProfile", profile.as_backend())
            .await
            .map_err(Error::SetProfile)
    }
}

#[instrument(skip_all)]
fn spawn_poll(power: Handle, refresh_interval: Duration) {
    tokio::spawn(async move {
        let mut ticker = interval(refresh_interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            power.refresh().await;
        }
    });
}

fn profile_from_map(profile: &HashMap<String, OwnedValue>) -> Option<Profile> {
    let value = profile.get("Profile")?;
    let raw = <&str>::try_from(value).ok()?;
    Profile::from_backend(raw)
}

fn push_unique_profile(mut profiles: Vec<Profile>, profile: Profile) -> Vec<Profile> {
    if !profiles.contains(&profile) {
        profiles.push(profile);
    }
    profiles
}

fn remaining_duration(state: BatteryState, empty: i64, full: i64) -> Option<Duration> {
    let seconds = match state {
        BatteryState::Charging | BatteryState::PendingCharge => full,
        BatteryState::Discharging | BatteryState::PendingDischarge => empty,
        BatteryState::Full | BatteryState::Empty | BatteryState::Unknown => 0,
    };
    if seconds <= 0 {
        None
    } else {
        Some(Duration::from_secs(seconds as u64))
    }
}

fn telemetry_value(value: f64) -> Option<f64> {
    if value.is_finite() && value > 0.0 {
        Some(value)
    } else {
        None
    }
}

fn empty_to_none(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

/// A power runtime or D-Bus boundary error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The system bus could not be opened.
    #[error("failed to connect to the system bus")]
    ConnectSystemBus(#[source] zbus::Error),
    /// No system bus client is available.
    #[error("power services are unavailable")]
    Unavailable,
    /// The UPower display-device proxy could not be created.
    #[error("UPower display device is unavailable")]
    CreateUpowerProxy(#[source] zbus::Error),
    /// Battery properties could not be read.
    #[error("battery telemetry is unavailable")]
    ReadBattery(#[source] zbus::Error),
    /// The power-profiles-daemon proxy could not be created.
    #[error("power profile control is unavailable")]
    CreateProfilesProxy(#[source] zbus::Error),
    /// Power profile properties could not be read.
    #[error("power profile state is unavailable")]
    ReadProfiles(#[source] zbus::Error),
    /// A requested profile is not exposed by the backend.
    #[error("power profile `{}` is unavailable", profile.as_backend())]
    ProfileUnavailable {
        /// Requested profile.
        profile: Profile,
    },
    /// The active power profile could not be changed.
    #[error("failed to set power profile")]
    SetProfile(#[source] zbus::fdo::Error),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{BatteryState, Configuration, remaining_duration};

    #[test]
    fn config_accepts_human_refresh_cadence() {
        let config = toml::from_str::<Configuration>(
            r#"
            refresh_interval = "2s"
            "#,
        )
        .expect("power config should parse human refresh cadence");

        assert_eq!(config.refresh_interval.duration(), Duration::from_secs(2));
    }

    #[test]
    fn config_rejects_zero_refresh_cadence() {
        let error = toml::from_str::<Configuration>(
            r#"
            refresh_interval = "0s"
            "#,
        )
        .expect_err("zero power refresh cadence should not deserialize");

        assert!(error.to_string().contains("cadence cannot be zero"));
    }

    #[test]
    fn remaining_time_follows_battery_state() {
        assert_eq!(
            remaining_duration(BatteryState::Discharging, 120, 900),
            Some(Duration::from_secs(120))
        );
        assert_eq!(
            remaining_duration(BatteryState::Charging, 120, 900),
            Some(Duration::from_secs(900))
        );
        assert_eq!(remaining_duration(BatteryState::Full, 120, 900), None);
    }
}
