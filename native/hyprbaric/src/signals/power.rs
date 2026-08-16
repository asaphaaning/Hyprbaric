use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PowerProfile {
    Saver,
    Balanced,
    Performance,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PowerBatteryState {
    #[default]
    Unknown,
    Charging,
    Discharging,
    Empty,
    Full,
    PendingCharge,
    PendingDischarge,
}

/// A power-profile command exchanged with Flutter.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PowerCommand {
    SetProfile { profile: PowerProfile },
}

#[derive(Deserialize, DartSignal)]
pub struct PowerSetProfile {
    pub profile: PowerProfile,
}

#[derive(Serialize, RustSignal)]
pub struct PowerStatus {
    pub battery_present: bool,
    pub percentage: Option<u8>,
    pub state: PowerBatteryState,
    pub remaining_seconds: Option<u64>,
    pub power_rate_watts: Option<f64>,
    pub voltage: Option<f64>,
    pub temperature_celsius: Option<f64>,
    pub active_profile: Option<PowerProfile>,
    pub available_profiles: Vec<PowerProfile>,
    pub battery_message: Option<String>,
    pub profile_message: Option<String>,
    pub degraded: Option<String>,
    pub inhibited: Option<String>,
}

#[derive(Serialize, RustSignal)]
pub enum PowerCommandResult {
    Started {
        command: PowerCommand,
    },
    Failed {
        command: PowerCommand,
        message: String,
    },
}
