use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

/// A network command reported back to Flutter.
///
/// Variant data remains attached to the command that owns it so a Wi-Fi SSID
/// cannot accidentally accompany a scan or settings launch report.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NetworkCommand {
    Scan,
    SetWifiEnabled { enabled: bool },
    Connect { ssid: String },
    OpenSettings,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkEntry {
    pub ssid: String,
    pub bssid: Option<String>,
    pub strength: u8,
    pub secure: bool,
    pub state: NetworkEntryState,
}

/// The live display state of a visible Wi-Fi entry.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum NetworkEntryState {
    #[default]
    Available,
    Active,
    Connecting,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkTransfer {
    pub bytes_per_second: u64,
    pub total_bytes: u64,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkTraffic {
    pub upload: NetworkTransfer,
    pub download: NetworkTransfer,
    pub ping_ms: Option<u16>,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkInterface {
    pub name: String,
    pub address: Option<String>,
    pub active: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct NetworkScanRequest {}

#[derive(Deserialize, DartSignal)]
pub struct NetworkSetWifiEnabled {
    pub enabled: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct NetworkConnectRequest {
    pub ssid: String,
    pub bssid: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, DartSignal)]
pub struct NetworkSettingsRequest {}

#[derive(Serialize, RustSignal)]
pub struct NetworkStatus {
    pub wifi_enabled: bool,
    pub device_present: bool,
    pub scanning: bool,
    pub active_ssid: Option<String>,
    pub traffic: NetworkTraffic,
    pub networks: Vec<NetworkEntry>,
    pub interfaces: Vec<NetworkInterface>,
    pub message: Option<String>,
}

/// The result of pushing a network command toward its system boundary.
///
/// A failed command always owns user-facing failure copy. Started commands do
/// not carry a nullable message slot.
#[derive(Serialize, RustSignal)]
pub enum NetworkCommandResult {
    Started {
        command: NetworkCommand,
    },
    Failed {
        command: NetworkCommand,
        message: String,
    },
}
