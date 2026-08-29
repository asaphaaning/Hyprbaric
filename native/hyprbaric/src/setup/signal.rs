//! RINF projections for the setup guide.

use crate::signals;

use super::{Command, Outcome, Report, Status};

impl From<signals::SetupOutcome> for Outcome {
    fn from(outcome: signals::SetupOutcome) -> Self {
        match outcome {
            signals::SetupOutcome::Finished => Self::Finished,
            signals::SetupOutcome::Skipped => Self::Skipped,
        }
    }
}

impl From<Outcome> for signals::SetupOutcome {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Finished => Self::Finished,
            Outcome::Skipped => Self::Skipped,
        }
    }
}

impl From<signals::SetupCommand> for Command {
    fn from(command: signals::SetupCommand) -> Self {
        match command {
            signals::SetupCommand::Complete { outcome } => Self::Complete(outcome.into()),
        }
    }
}

impl From<Status> for signals::SetupState {
    fn from(status: Status) -> Self {
        match status {
            Status::Required => Self::Required,
            Status::Complete => Self::Complete,
            Status::Disabled => Self::Disabled,
        }
    }
}

impl From<&Command> for signals::SetupCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::Complete(outcome) => Self::Complete {
                outcome: (*outcome).into(),
            },
        }
    }
}

impl From<&Report> for signals::SetupCommandResult {
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
