use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SessionAction {
    Lock,
    Suspend,
    Logout,
    Restart,
    Shutdown,
    RebootToFirmware,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SessionCommandOutcome {
    Started,
    Failed,
}

#[derive(Deserialize, DartSignal)]
pub struct SessionCommand {
    pub action: SessionAction,
}

#[derive(Serialize, RustSignal)]
pub struct SessionActionAvailability {
    pub firmware_reboot_supported: bool,
}

#[derive(Serialize, RustSignal)]
pub struct SessionCommandResult {
    pub action: SessionAction,
    pub outcome: SessionCommandOutcome,
    pub message: Option<String>,
}
