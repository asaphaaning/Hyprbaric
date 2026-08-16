//! RINF projections for brightness state and reports.

use crate::signals;

use super::{Command, Report, Snapshot};

impl From<&Snapshot> for signals::BrightnessStatus {
    fn from(snapshot: &Snapshot) -> Self {
        match snapshot {
            Snapshot::Discovering { message } => Self::Discovering {
                message: message.clone(),
            },
            Snapshot::Available { value, device } => Self::Available {
                value: value.as_u8(),
                device: device.clone(),
            },
            Snapshot::Unavailable { message } => Self::Unavailable {
                message: message.clone(),
            },
        }
    }
}

impl From<&Report> for signals::BrightnessCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started(command) => Self::Started {
                command: command.into(),
            },
            Report::Failed { command, message } => Self::Failed {
                command: command.into(),
                message: message.clone(),
            },
        }
    }
}

impl From<&Command> for signals::BrightnessCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetLevel { value } => Self::SetLevel {
                value: value.as_u8(),
            },
        }
    }
}
