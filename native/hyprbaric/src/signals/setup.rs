use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

/// Why the setup journey was acknowledged.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SetupOutcome {
    /// The final step was completed.
    Finished,
    /// The guide was skipped.
    Skipped,
}

/// Setup-guide command sent by Flutter.
#[derive(
    Serialize, Deserialize, DartSignal, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub enum SetupCommand {
    /// Persist an acknowledged journey.
    Complete { outcome: SetupOutcome },
}

/// Startup state of the setup guide.
#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SetupState {
    /// The guide should open automatically.
    Required,
    /// The journey has already been acknowledged.
    Complete,
    /// Automatic display is disabled by TOML policy.
    Disabled,
}

/// Current setup-guide state.
#[derive(Serialize, RustSignal)]
pub struct SetupStatus {
    pub state: SetupState,
}

/// Result of persisting setup completion.
#[derive(Serialize, RustSignal)]
pub enum SetupCommandResult {
    /// Persistence started.
    Started { command: SetupCommand },
    /// Completion was persisted.
    Saved { command: SetupCommand },
    /// Completion could not be persisted.
    Failed {
        command: SetupCommand,
        message: String,
    },
}
