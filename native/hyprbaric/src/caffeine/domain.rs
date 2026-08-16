//! Caffeine state published to the controls popup.

/// UI-facing Caffeine state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snapshot {
    /// login1 can hold an idle/sleep inhibitor.
    Available {
        /// Whether Hyprbaric currently holds the inhibitor fd.
        enabled: bool,
    },
    /// login1 or its inhibitor API is unavailable.
    Unavailable {
        /// User-facing failure detail.
        message: String,
    },
}

/// A Caffeine command that reached the runtime boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Enable or disable the login1 inhibitor.
    SetEnabled {
        /// Requested enabled state.
        enabled: bool,
    },
}

/// A Caffeine command report published to subscribers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the worker.
    Started(Command),
    /// The command completed.
    Saved(Command),
    /// The command failed before or at its backend.
    Failed {
        /// The command that failed.
        command: Command,
        /// User-facing failure detail.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::{Command, Snapshot};

    #[test]
    fn snapshot_tracks_enabled_state() {
        assert_eq!(
            Snapshot::Available { enabled: true },
            Snapshot::Available { enabled: true }
        );
    }

    #[test]
    fn command_tracks_requested_state() {
        assert_eq!(
            Command::SetEnabled { enabled: false },
            Command::SetEnabled { enabled: false }
        );
    }
}
