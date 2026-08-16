use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

/// A brightness command reported back to Flutter.
///
/// Command data stays attached to the variant that owns it so reports can be
/// rendered without consulting stale UI state.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BrightnessCommand {
    SetLevel { value: u8 },
}

#[derive(Deserialize, DartSignal)]
pub struct BrightnessSetLevel {
    pub value: u8,
}

/// Current brightness availability and selected target.
#[derive(Serialize, RustSignal)]
pub enum BrightnessStatus {
    Discovering { message: String },
    Available { value: u8, device: String },
    Unavailable { message: String },
}

/// The result of pushing a brightness command toward its system boundary.
#[derive(Serialize, RustSignal)]
pub enum BrightnessCommandResult {
    Started {
        command: BrightnessCommand,
    },
    Failed {
        command: BrightnessCommand,
        message: String,
    },
}
