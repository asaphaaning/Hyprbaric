//! NetworkManager Wi-Fi reads and effects.

use nmrs::{DeviceState, NetworkManager, WifiSecurity};
use tokio::sync::Mutex;
use tracing::instrument;

use super::{
    Error,
    domain::{Entry, EntryState, Interface, Snapshot},
    traffic::{self, Sample},
};

/// Whether a full snapshot should advertise an active scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Scan {
    /// Read normal network state.
    Idle,
    /// Read network state around an explicit scan request.
    Requested,
}

/// The long-lived NetworkManager boundary owned by [`super::Wifi`].
///
/// [`NetworkManager`] clones keep using the same underlying D-Bus connection,
/// so [`Client`] caches one successful construction and lends cheap clones to
/// individual async reads or commands. A failed initial construction leaves the
/// cache empty for a later poll or UI command to retry.
#[derive(Debug)]
pub(super) struct Client {
    manager: Mutex<Option<NetworkManager>>,
}

/// A connect request at the NetworkManager boundary.
///
/// Secrets stay inside this type after the RINF handler reaches [`super::Wifi`]
/// so tracing and result reporting do not accidentally print them.
pub(super) struct Connect {
    ssid: String,
    bssid: Option<String>,
    password: Option<String>,
}

impl Scan {
    /// Returns whether the UI should render the snapshot as scanning.
    const fn is_requested(self) -> bool {
        matches!(self, Self::Requested)
    }
}

impl Client {
    /// Creates an empty NetworkManager client cache.
    pub(super) fn new() -> Self {
        Self {
            manager: Mutex::new(None),
        }
    }

    /// Reads a full NetworkManager snapshot and traffic baseline.
    #[instrument(skip(self, previous))]
    pub(super) async fn read_snapshot(
        &self,
        scan: Scan,
        previous: Option<&Sample>,
    ) -> Result<(Snapshot, Sample), Error> {
        let manager = self.manager().await?;
        let wifi_enabled = manager
            .airplane_mode_state()
            .await
            .map_err(Error::NetworkManager)?
            .wifi
            .enabled;
        let wireless_devices = manager
            .list_wireless_devices()
            .await
            .map_err(Error::NetworkManager)?;
        let device_present = !wireless_devices.is_empty();
        let active_ssid = manager.current_ssid().await;
        let connecting = manager.is_connecting().await.unwrap_or(false);
        let interfaces = interfaces(&manager).await?;
        let networks = if wifi_enabled && device_present {
            visible_networks(&manager, active_ssid.as_deref(), connecting).await?
        } else {
            Vec::new()
        };
        let (traffic, sample) = traffic::read_with_ping(previous).await?;

        Ok((
            Snapshot {
                wifi_enabled,
                device_present,
                scanning: scan.is_requested(),
                active_ssid,
                traffic,
                networks,
                interfaces,
                message: None,
            },
            sample,
        ))
    }

    /// Requests a NetworkManager access-point scan.
    #[instrument(skip(self))]
    pub(super) async fn scan(&self) -> Result<(), Error> {
        self.manager()
            .await?
            .scan_networks(None)
            .await
            .map_err(Error::NetworkManager)
    }

    /// Enables or disables NetworkManager Wi-Fi support.
    #[instrument(skip(self))]
    pub(super) async fn set_enabled(&self, enabled: bool) -> Result<(), Error> {
        self.manager()
            .await?
            .set_wireless_enabled(enabled)
            .await
            .map_err(Error::NetworkManager)
    }

    /// Connects to a visible Wi-Fi network.
    #[instrument(skip(self, request), fields(ssid = %request.ssid()))]
    pub(super) async fn connect(&self, request: Connect) -> Result<(), Error> {
        let manager = self.manager().await?;
        let network = manager
            .list_networks(None)
            .await
            .map_err(Error::NetworkManager)?
            .into_iter()
            .find(|network| {
                network.ssid == request.ssid
                    && request
                        .bssid
                        .as_ref()
                        .is_none_or(|value| network.bssid.as_ref() == Some(value))
            })
            .ok_or_else(|| Error::NetworkNotFound {
                ssid: request.ssid.clone(),
            })?;

        if network.is_eap {
            return Err(Error::UnsupportedEnterprise { ssid: request.ssid });
        }

        let security = if network.secured {
            let password = request
                .password
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or(Error::MissingPassword)?;
            WifiSecurity::WpaPsk {
                psk: password.to_string(),
            }
        } else {
            WifiSecurity::Open
        };

        manager
            .connect(&request.ssid, None, security)
            .await
            .map_err(Error::NetworkManager)
    }

    /// Returns a shared NetworkManager handle, creating it after cache misses.
    async fn manager(&self) -> Result<NetworkManager, Error> {
        if let Some(manager) = self.manager.lock().await.clone() {
            return Ok(manager);
        }

        let manager = NetworkManager::new().await.map_err(Error::NetworkManager)?;
        let mut cached = self.manager.lock().await;
        if let Some(cached) = cached.as_ref() {
            return Ok(cached.clone());
        }

        *cached = Some(manager.clone());
        Ok(manager)
    }
}

impl Connect {
    /// Creates a Wi-Fi connection request.
    pub(super) fn new(ssid: String, bssid: Option<String>, password: Option<String>) -> Self {
        Self {
            ssid,
            bssid,
            password,
        }
    }

    /// Returns the target SSID without exposing other connection input.
    pub(super) fn ssid(&self) -> &str {
        &self.ssid
    }
}

/// Reads and normalizes NetworkManager interface rows.
async fn interfaces(manager: &NetworkManager) -> Result<Vec<Interface>, Error> {
    Ok(manager
        .list_devices()
        .await
        .map_err(Error::NetworkManager)?
        .into_iter()
        .filter(|device| !device.interface.trim().is_empty())
        .map(|device| Interface {
            name: device.interface,
            address: device
                .ip4_address
                .or(device.ip6_address)
                .map(|address| address.split('/').next().unwrap_or(&address).to_string()),
            active: matches!(device.state, DeviceState::Activated),
        })
        .collect())
}

/// Reads visible access points as display entries.
async fn visible_networks(
    manager: &NetworkManager,
    active_ssid: Option<&str>,
    connecting: bool,
) -> Result<Vec<Entry>, Error> {
    let entries = manager
        .list_networks(None)
        .await
        .map_err(Error::NetworkManager)?
        .into_iter()
        .filter(|network| !network.ssid.trim().is_empty())
        .map(|network| {
            let active = active_ssid.is_some_and(|ssid| ssid == network.ssid);

            Entry {
                ssid: network.ssid,
                bssid: network.bssid,
                strength: network.strength.unwrap_or(0),
                secure: network.secured,
                state: EntryState::from_connection(active, connecting),
            }
        })
        .collect();

    Ok(Entry::visible(entries))
}
