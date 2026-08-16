//! Session action domain vocabulary.

/// Optional session action availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Availability {
    /// Whether login1 can reboot into firmware setup.
    pub firmware_reboot_supported: bool,
}

/// A session action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Lock the current session.
    Lock,
    /// Suspend the system.
    Suspend,
    /// End the current Hyprland session.
    Logout,
    /// Restart the system.
    Restart,
    /// Shut down the system.
    Shutdown,
    /// Reboot into firmware setup.
    RebootToFirmware,
}

/// Session action report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The action started successfully.
    Started { action: Action },
    /// The action failed.
    Failed { action: Action, message: String },
}

impl Action {
    /// Returns a display label for tracing and command results.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Lock => "Lock",
            Self::Suspend => "Suspend",
            Self::Logout => "Logout",
            Self::Restart => "Restart",
            Self::Shutdown => "Shutdown",
            Self::RebootToFirmware => "Reboot to Firmware",
        }
    }
}

impl Report {
    /// Creates a started report.
    pub const fn started(action: Action) -> Self {
        Self::Started { action }
    }

    /// Creates a failed report.
    pub fn failed(action: Action, error: impl ToString) -> Self {
        Self::Failed {
            action,
            message: error.to_string(),
        }
    }
}

pub(super) fn firmware_setup_supported(result: &str) -> bool {
    matches!(result.trim(), "yes" | "challenge")
}

#[cfg(test)]
mod tests {
    use super::firmware_setup_supported;

    #[test]
    fn firmware_setup_is_available_for_yes_or_challenge() {
        assert!(firmware_setup_supported("yes"));
        assert!(firmware_setup_supported("challenge"));
    }

    #[test]
    fn firmware_setup_is_unavailable_for_no_or_na() {
        assert!(!firmware_setup_supported("no"));
        assert!(!firmware_setup_supported("na"));
    }
}
