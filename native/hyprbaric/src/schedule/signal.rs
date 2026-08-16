//! RINF projections for daily scheduling.

use crate::signals;

use super::{Action, Command, DailyWindow, Entry, Hour, Report, Snapshot};

impl From<Action> for signals::ScheduleAction {
    fn from(action: Action) -> Self {
        match action {
            Action::NightLight => Self::NightLight,
        }
    }
}

impl From<signals::ScheduleAction> for Action {
    fn from(action: signals::ScheduleAction) -> Self {
        match action {
            signals::ScheduleAction::NightLight => Self::NightLight,
        }
    }
}

impl TryFrom<signals::ScheduleCommand> for Command {
    type Error = super::Error;

    fn try_from(command: signals::ScheduleCommand) -> Result<Self, Self::Error> {
        match command {
            signals::ScheduleCommand::SetDailyWindow {
                action,
                enabled,
                start_hour,
                stop_hour,
            } => Ok(Self::SetDailyWindow {
                action: action.into(),
                window: DailyWindow {
                    enabled,
                    start: Hour::new(start_hour)?,
                    stop: Hour::new(stop_hour)?,
                },
            }),
        }
    }
}

impl From<&Snapshot> for signals::ScheduleStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            entries: snapshot.entries.iter().map(Into::into).collect(),
        }
    }
}

impl From<&Entry> for signals::ScheduleEntry {
    fn from(entry: &Entry) -> Self {
        Self {
            action: entry.action.into(),
            enabled: entry.window.enabled,
            start_hour: entry.window.start.as_u8(),
            stop_hour: entry.window.stop.as_u8(),
        }
    }
}

impl From<&Report> for signals::ScheduleCommandResult {
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

impl From<&Command> for signals::ScheduleCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetDailyWindow { action, window } => Self::SetDailyWindow {
                action: (*action).into(),
                enabled: window.enabled,
                start_hour: window.start.as_u8(),
                stop_hour: window.stop.as_u8(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        schedule::{Action, Command, DailyWindow, Hour, Snapshot},
        signals,
    };

    #[test]
    fn command_rejects_invalid_hours() {
        let error = Command::try_from(signals::ScheduleCommand::SetDailyWindow {
            action: signals::ScheduleAction::NightLight,
            enabled: true,
            start_hour: 24,
            stop_hour: 7,
        })
        .expect_err("invalid hour should fail");

        assert!(error.to_string().contains("outside 0..=23"));
    }

    #[test]
    fn snapshot_projects_entries() {
        let status = signals::ScheduleStatus::from(&Snapshot::night_light(DailyWindow {
            enabled: true,
            start: Hour::new(22).expect("hour should parse"),
            stop: Hour::new(6).expect("hour should parse"),
        }));

        assert_eq!(status.entries.len(), 1);
        assert_eq!(
            status.entries[0].action,
            signals::ScheduleAction::NightLight
        );
        assert_eq!(status.entries[0].start_hour, 22);
        assert_eq!(status.entries[0].stop_hour, 6);
    }

    #[test]
    fn command_projects_to_transport() {
        let command = signals::ScheduleCommand::from(&Command::SetDailyWindow {
            action: Action::NightLight,
            window: DailyWindow::default(),
        });

        assert!(matches!(
            command,
            signals::ScheduleCommand::SetDailyWindow {
                action: signals::ScheduleAction::NightLight,
                enabled: false,
                start_hour: 21,
                stop_hour: 7,
            }
        ));
    }
}
