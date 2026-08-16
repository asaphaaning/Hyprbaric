use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NightLightCommand {
    SetEnabled { enabled: bool },
    SetTemperature { temperature: u32 },
}

#[derive(Deserialize, DartSignal)]
pub struct NightLightSetEnabled {
    pub enabled: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct NightLightSetTemperature {
    pub temperature: u32,
}

#[derive(Serialize, RustSignal)]
pub enum NightLightStatus {
    Available {
        enabled: bool,
        temperature: u32,
    },
    Unavailable {
        enabled: bool,
        temperature: u32,
        message: String,
    },
}

#[derive(Serialize, RustSignal)]
pub enum NightLightCommandResult {
    Started {
        command: NightLightCommand,
    },
    Saved {
        command: NightLightCommand,
    },
    Failed {
        command: NightLightCommand,
        message: String,
    },
}
