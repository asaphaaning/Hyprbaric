use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecordingMode {
    Region,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecordingAction {
    Toggle,
}

#[derive(Deserialize, DartSignal)]
pub struct RecordingRequest {
    pub action: RecordingAction,
    pub mode: RecordingMode,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecordingCommand {
    Toggle { mode: RecordingMode },
}

#[derive(Serialize, RustSignal)]
pub enum RecordingStatus {
    Unavailable {
        message: String,
    },
    Idle,
    Selecting {
        mode: RecordingMode,
    },
    Recording {
        mode: RecordingMode,
        path: String,
        started_at_ms: u64,
    },
    Stopping {
        mode: RecordingMode,
        path: String,
        started_at_ms: u64,
    },
}

#[derive(Serialize, RustSignal)]
pub enum RecordingCommandResult {
    Started {
        command: RecordingCommand,
    },
    Saved {
        command: RecordingCommand,
        path: String,
    },
    Cancelled {
        command: RecordingCommand,
    },
    Failed {
        command: RecordingCommand,
        message: String,
    },
}
