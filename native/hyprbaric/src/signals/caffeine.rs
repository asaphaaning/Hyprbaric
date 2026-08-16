use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CaffeineCommand {
    SetEnabled { enabled: bool },
}

#[derive(Deserialize, DartSignal)]
pub struct CaffeineSetEnabled {
    pub enabled: bool,
}

#[derive(Serialize, RustSignal)]
pub enum CaffeineStatus {
    Available { enabled: bool },
    Unavailable { message: String },
}

#[derive(Serialize, RustSignal)]
pub enum CaffeineCommandResult {
    Started {
        command: CaffeineCommand,
    },
    Saved {
        command: CaffeineCommand,
    },
    Failed {
        command: CaffeineCommand,
        message: String,
    },
}
