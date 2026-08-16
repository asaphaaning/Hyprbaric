//! RINF projections for night-light state and reports.

use crate::signals;

use super::{Command, Report, Snapshot, Temperature};

impl TryFrom<signals::NightLightSetTemperature> for Command {
    type Error = super::Error;

    fn try_from(request: signals::NightLightSetTemperature) -> Result<Self, Self::Error> {
        Ok(Self::SetTemperature {
            temperature: Temperature::new(request.temperature)?,
        })
    }
}

impl From<signals::NightLightSetEnabled> for Command {
    fn from(request: signals::NightLightSetEnabled) -> Self {
        Self::SetEnabled {
            enabled: request.enabled,
        }
    }
}

impl From<&Snapshot> for signals::NightLightStatus {
    fn from(snapshot: &Snapshot) -> Self {
        match snapshot {
            Snapshot::Available {
                enabled,
                temperature,
            } => Self::Available {
                enabled: *enabled,
                temperature: temperature.as_u32(),
            },
            Snapshot::Unavailable {
                enabled,
                temperature,
                message,
            } => Self::Unavailable {
                enabled: *enabled,
                temperature: temperature.as_u32(),
                message: message.clone(),
            },
        }
    }
}

impl From<&Report> for signals::NightLightCommandResult {
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

impl From<&Command> for signals::NightLightCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetEnabled { enabled } => Self::SetEnabled { enabled: *enabled },
            Command::SetTemperature { temperature } => Self::SetTemperature {
                temperature: temperature.as_u32(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        night_light::{Command, Report, Snapshot, Temperature},
        signals,
    };

    #[test]
    fn snapshot_projects_available_status() {
        let status = signals::NightLightStatus::from(&Snapshot::Available {
            enabled: true,
            temperature: Temperature::new(2700).expect("temperature should be valid"),
        });

        assert!(matches!(
            status,
            signals::NightLightStatus::Available {
                enabled: true,
                temperature: 2700
            }
        ));
    }

    #[test]
    fn report_projects_saved_command() {
        let command = Command::SetEnabled { enabled: false };
        let report = signals::NightLightCommandResult::from(&Report::Saved(command));

        assert!(matches!(
            report,
            signals::NightLightCommandResult::Saved {
                command: signals::NightLightCommand::SetEnabled { enabled: false }
            }
        ));
    }
}
