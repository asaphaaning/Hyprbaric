//! RINF projections for session actions and reports.

use crate::signals;

use super::{Action, Availability, Report};

impl From<signals::SessionAction> for Action {
    fn from(action: signals::SessionAction) -> Self {
        match action {
            signals::SessionAction::Lock => Self::Lock,
            signals::SessionAction::Suspend => Self::Suspend,
            signals::SessionAction::Logout => Self::Logout,
            signals::SessionAction::Restart => Self::Restart,
            signals::SessionAction::Shutdown => Self::Shutdown,
            signals::SessionAction::RebootToFirmware => Self::RebootToFirmware,
        }
    }
}

impl From<Action> for signals::SessionAction {
    fn from(action: Action) -> Self {
        match action {
            Action::Lock => Self::Lock,
            Action::Suspend => Self::Suspend,
            Action::Logout => Self::Logout,
            Action::Restart => Self::Restart,
            Action::Shutdown => Self::Shutdown,
            Action::RebootToFirmware => Self::RebootToFirmware,
        }
    }
}

impl From<signals::SessionCommand> for Action {
    fn from(command: signals::SessionCommand) -> Self {
        command.action.into()
    }
}

impl From<&Availability> for signals::SessionActionAvailability {
    fn from(availability: &Availability) -> Self {
        Self {
            firmware_reboot_supported: availability.firmware_reboot_supported,
        }
    }
}

impl From<&Report> for signals::SessionCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started { action } => Self {
                action: (*action).into(),
                outcome: signals::SessionCommandOutcome::Started,
                message: None,
            },
            Report::Failed { action, message } => Self {
                action: (*action).into(),
                outcome: signals::SessionCommandOutcome::Failed,
                message: Some(message.clone()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{session::Action, signals};

    #[test]
    fn session_actions_project_to_domain_vocabulary() {
        assert_eq!(Action::from(signals::SessionAction::Lock), Action::Lock);
        assert_eq!(
            Action::from(signals::SessionAction::RebootToFirmware),
            Action::RebootToFirmware
        );
    }

    #[test]
    fn availability_projection_preserves_firmware_support() {
        let signal = signals::SessionActionAvailability::from(&crate::session::Availability {
            firmware_reboot_supported: true,
        });
        assert!(signal.firmware_reboot_supported);
    }
}
