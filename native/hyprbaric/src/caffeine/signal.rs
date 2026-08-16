//! RINF projections for Caffeine state and reports.

use crate::signals;

use super::{Command, Report, Snapshot};

impl From<signals::CaffeineSetEnabled> for Command {
    fn from(request: signals::CaffeineSetEnabled) -> Self {
        Self::SetEnabled {
            enabled: request.enabled,
        }
    }
}

impl From<&Snapshot> for signals::CaffeineStatus {
    fn from(snapshot: &Snapshot) -> Self {
        match snapshot {
            Snapshot::Available { enabled } => Self::Available { enabled: *enabled },
            Snapshot::Unavailable { message } => Self::Unavailable {
                message: message.clone(),
            },
        }
    }
}

impl From<&Report> for signals::CaffeineCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started(command) => Self::Started {
                command: command.into(),
            },
            Report::Saved(command) => Self::Saved {
                command: command.into(),
            },
            Report::Failed { command, message } => Self::Failed {
                command: command.into(),
                message: message.clone(),
            },
        }
    }
}

impl From<&Command> for signals::CaffeineCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetEnabled { enabled } => Self::SetEnabled { enabled: *enabled },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        caffeine::{Command, Report, Snapshot},
        signals,
    };

    #[test]
    fn snapshot_projects_available_status() {
        let status = signals::CaffeineStatus::from(&Snapshot::Available { enabled: true });

        assert!(matches!(
            status,
            signals::CaffeineStatus::Available { enabled: true }
        ));
    }

    #[test]
    fn report_projects_failed_command() {
        let command = Command::SetEnabled { enabled: true };
        let report = signals::CaffeineCommandResult::from(&Report::Failed {
            command,
            message: "denied".to_owned(),
        });

        assert!(matches!(
            report,
            signals::CaffeineCommandResult::Failed {
                command: signals::CaffeineCommand::SetEnabled { enabled: true },
                message
            } if message == "denied"
        ));
    }
}
