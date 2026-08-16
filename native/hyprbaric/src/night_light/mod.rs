//! Night-light runtime backed by hyprsunset.
//!
//! [`NightLight`] owns the live state used by the controls popup and settings
//! modal. The domain module defines the public vocabulary, while [`backend`]
//! owns the hyprsunset, hyprctl, and systemd boundaries.

mod backend;
mod domain;
pub mod settings;
mod signal;

use std::{sync::Arc, time::Duration};

use serde::Deserialize;
use tokio::{
    sync::{Mutex, broadcast},
    time,
};
use tracing::instrument;

use self::backend::Backend;

pub use domain::{Command, Report, Snapshot, Temperature};

/// Shared night-light runtime handle.
pub type Handle = Arc<NightLight>;

const BACKEND_APPLY_TIMEOUT: Duration = Duration::from_secs(6);

/// Night-light configuration.
///
/// ```toml
/// [night_light]
/// enabled = false
/// temperature = 3500
/// ```
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Configuration {
    enabled: bool,
    temperature: Temperature,
}

/// Live hyprsunset controller.
pub struct NightLight {
    events: broadcast::Sender<Snapshot>,
    results: broadcast::Sender<Report>,
    backend: Backend,
    state: Mutex<Running>,
}

#[derive(Clone, Copy, Debug)]
struct Running {
    config: Configuration,
    available: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            enabled: false,
            temperature: Temperature::default(),
        }
    }
}

impl Configuration {
    /// Returns whether night light should be restored at launch.
    pub const fn enabled(self) -> bool {
        self.enabled
    }

    /// Returns the configured hyprsunset temperature.
    pub const fn temperature(self) -> Temperature {
        self.temperature
    }

    fn apply(self, command: &Command) -> Self {
        match command {
            Command::SetEnabled { enabled } => Self {
                enabled: *enabled,
                ..self
            },
            Command::SetTemperature { temperature } => Self {
                temperature: *temperature,
                ..self
            },
        }
    }
}

impl NightLight {
    /// Bootstraps night-light state and restores enabled configuration.
    #[instrument(skip_all)]
    pub async fn bootstrap(config: &Configuration) -> (Handle, Snapshot) {
        let backend = Backend;
        let available = backend.is_available();
        let initial_snapshot = project_snapshot(*config, available, None);

        let (events, _) = broadcast::channel(16);
        let (results, _) = broadcast::channel(8);
        let night_light = Arc::new(Self {
            events,
            results,
            backend,
            state: Mutex::new(Running {
                config: *config,
                available,
            }),
        });

        (night_light, initial_snapshot)
    }

    /// Subscribes to night-light [`Snapshot`] changes.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Subscribes to command [`Report`] updates.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    /// Enables or disables the filter.
    #[instrument(skip(self))]
    pub async fn set_enabled(self: &Arc<Self>, enabled: bool) {
        self.apply(Command::SetEnabled { enabled }).await;
    }

    /// Updates the configured temperature.
    #[instrument(skip(self))]
    pub async fn set_temperature(self: &Arc<Self>, temperature: Temperature) {
        self.apply(Command::SetTemperature { temperature }).await;
    }

    /// Returns whether the runtime should re-apply this desired enabled state.
    #[instrument(skip(self))]
    pub async fn needs_reconcile(&self, enabled: bool) -> bool {
        let state = self.state.lock().await;

        state.config.enabled() != enabled || (enabled && !state.available)
    }

    /// Returns the latest UI-facing night-light state.
    #[instrument(skip(self))]
    pub async fn snapshot(&self) -> Snapshot {
        let state = self.state.lock().await;

        project_snapshot(state.config, state.available, None)
    }

    #[instrument(skip(self, command))]
    async fn apply(self: &Arc<Self>, command: Command) {
        drop(self.results.send(Report::Started(command.clone())));
        let current = { self.state.lock().await.config };
        let next = match settings::save(&command, current) {
            Ok(next) => next,
            Err(error) => {
                drop(self.results.send(Report::Failed {
                    command,
                    message: error.to_string(),
                }));
                return;
            }
        };

        let available = {
            let mut state = self.state.lock().await;
            state.config = next;
            state.available
        };
        drop(self.results.send(Report::Saved(command.clone())));
        drop(self.events.send(project_snapshot(next, available, None)));

        let night_light = Arc::clone(self);
        tokio::spawn(async move {
            night_light
                .finish_backend_apply(command, current, next)
                .await;
        });
    }

    #[instrument(skip(self, command))]
    async fn finish_backend_apply(
        self: Arc<Self>,
        command: Command,
        current: Configuration,
        next: Configuration,
    ) {
        let result = match time::timeout(
            BACKEND_APPLY_TIMEOUT,
            self.apply_backend(&command, current, next),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(Error::BackendTimeout),
        };

        match result {
            Ok(available) => {
                let mut state = self.state.lock().await;
                state.available = available;
                drop(
                    self.events
                        .send(project_snapshot(state.config, available, None)),
                );
            }
            Err(error) => {
                let mut state = self.state.lock().await;
                state.available = false;
                let snapshot = project_snapshot(state.config, false, Some(error.to_string()));
                drop(self.results.send(Report::Failed {
                    command,
                    message: error.to_string(),
                }));
                drop(self.events.send(snapshot));
            }
        }
    }

    async fn apply_backend(
        &self,
        command: &Command,
        current: Configuration,
        next: Configuration,
    ) -> Result<bool, Error> {
        match command {
            Command::SetEnabled { enabled: true } => {
                self.backend
                    .set_temperature_or_start(next.temperature())
                    .await?;
                Ok(true)
            }
            Command::SetEnabled { enabled: false } => {
                if current.enabled() && self.backend.is_available() {
                    if let Err(error) = self.backend.disable().await {
                        tracing::warn!(
                            "Failed to disable hyprsunset before saving off state: {error}"
                        );
                        return Err(error);
                    }
                }
                Ok(self.backend.is_available())
            }
            Command::SetTemperature { temperature } => {
                if next.enabled() {
                    self.backend.set_temperature_or_start(*temperature).await?;
                }
                Ok(self.backend.is_available())
            }
        }
    }
}

fn project_snapshot(config: Configuration, available: bool, message: Option<String>) -> Snapshot {
    if available {
        Snapshot::Available {
            enabled: config.enabled(),
            temperature: config.temperature(),
        }
    } else {
        Snapshot::Unavailable {
            enabled: config.enabled(),
            temperature: config.temperature(),
            message: message.unwrap_or_else(|| "hyprsunset is unavailable".to_owned()),
        }
    }
}

/// A night-light runtime or boundary error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// hyprsunset is not installed or not discoverable on PATH.
    #[error("hyprsunset is unavailable")]
    HyprsunsetUnavailable,
    /// Applying a backend command exceeded the startup-safe boundary.
    #[error("timed out applying night-light command to hyprsunset")]
    BackendTimeout,
    /// A temperature was outside the accepted domain.
    #[error(transparent)]
    Temperature(#[from] domain::TemperatureError),
    /// A process could not be spawned.
    #[error("failed to launch `{program}`")]
    Spawn {
        /// Program that failed to spawn.
        program: &'static str,
        /// Process spawn error.
        #[source]
        source: std::io::Error,
    },
    /// A process exited unsuccessfully.
    #[error("`{program}` failed with status {status}: {stderr}")]
    CommandFailed {
        /// Program that failed.
        program: &'static str,
        /// Exit status as reported by the process boundary.
        status: String,
        /// Captured stderr.
        stderr: String,
    },
    /// Could not connect to the user bus.
    #[error("failed to connect to the session bus")]
    ConnectSessionBus(#[source] zbus::Error),
    /// systemd rejected a unit lifecycle call.
    #[error("failed to call systemd {action} for hyprsunset.service")]
    UnitAction {
        /// Method called on the systemd manager.
        action: &'static str,
        /// D-Bus source error.
        #[source]
        source: zbus::Error,
    },
    /// systemd returned an unexpected unit lifecycle reply.
    #[error("failed to decode systemd {action} job for hyprsunset.service")]
    UnitActionReply {
        /// Method called on the systemd manager.
        action: &'static str,
        /// D-Bus source error.
        #[source]
        source: zbus::Error,
    },
    /// systemd rejected the ResetFailedUnit call.
    #[error("failed to reset hyprsunset.service failure state")]
    ResetFailedUnit(#[source] zbus::Error),
    /// Configuration path could not be found.
    #[error(transparent)]
    Config(#[from] crate::config::Error),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::{Mutex, broadcast};

    use super::{Backend, Configuration, NightLight, Running, Snapshot, Temperature};

    #[test]
    fn config_defaults_to_disabled_warm_temperature() {
        let config = Configuration::default();

        assert!(!config.enabled());
        assert_eq!(config.temperature().as_u32(), 3500);
    }

    #[test]
    fn config_accepts_temperature_override() {
        let config = toml::from_str::<Configuration>("temperature = 3000")
            .expect("night-light config should parse");

        assert!(!config.enabled());
        assert_eq!(config.temperature().as_u32(), 3000);
    }

    #[test]
    fn config_rejects_zero_temperature() {
        let error = toml::from_str::<Configuration>("temperature = 0")
            .expect_err("zero temperature should fail");

        assert!(error.to_string().contains("temperature cannot be zero"));
    }

    #[tokio::test]
    async fn snapshot_projects_live_state_after_reconcile() {
        let (events, _) = broadcast::channel(1);
        let (results, _) = broadcast::channel(1);
        let night_light = Arc::new(NightLight {
            events,
            results,
            backend: Backend,
            state: Mutex::new(Running {
                config: Configuration {
                    enabled: true,
                    temperature: Temperature::new(3500).expect("temperature should be valid"),
                },
                available: true,
            }),
        });

        {
            let mut state = night_light.state.lock().await;
            state.config = Configuration {
                enabled: false,
                temperature: state.config.temperature(),
            };
        }

        assert_eq!(
            night_light.snapshot().await,
            Snapshot::Available {
                enabled: false,
                temperature: Temperature::new(3500).expect("temperature should be valid"),
            }
        );
    }
}
