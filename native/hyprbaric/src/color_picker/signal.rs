//! RINF projections for color picker reports.

use crate::signals;

use super::{Command, Report};

impl From<Command> for signals::ColorPickerCommand {
    fn from(command: Command) -> Self {
        match command {
            Command::Pick => Self::Pick,
        }
    }
}

impl From<&Report> for signals::ColorPickerCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started { command } => Self {
                command: (*command).into(),
                outcome: signals::ColorPickerCommandOutcome::Started,
                color: None,
                message: None,
            },
            Report::Picked { command, color } => Self {
                command: (*command).into(),
                outcome: signals::ColorPickerCommandOutcome::Picked,
                color: Some(color.as_str().to_owned()),
                message: None,
            },
            Report::Cancelled { command } => Self {
                command: (*command).into(),
                outcome: signals::ColorPickerCommandOutcome::Cancelled,
                color: None,
                message: None,
            },
            Report::Failed { command, failure } => Self {
                command: (*command).into(),
                outcome: signals::ColorPickerCommandOutcome::Failed,
                color: None,
                message: Some(failure.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color_picker::{Color, Command, Failure, Report},
        signals,
    };

    #[test]
    fn picked_report_preserves_color() {
        let report = Report::picked(
            Command::Pick,
            Color::parse("#38bdf8").expect("color should parse"),
        );
        let signal = signals::ColorPickerCommandResult::from(&report);

        assert_eq!(signal.command, signals::ColorPickerCommand::Pick);
        assert_eq!(signal.outcome, signals::ColorPickerCommandOutcome::Picked);
        assert_eq!(signal.color.as_deref(), Some("#38bdf8"));
    }

    #[test]
    fn cancelled_report_maps_without_message() {
        let signal = signals::ColorPickerCommandResult::from(&Report::cancelled(Command::Pick));

        assert_eq!(
            signal.outcome,
            signals::ColorPickerCommandOutcome::Cancelled
        );
        assert_eq!(signal.message, None);
    }

    #[test]
    fn failed_report_uses_failure_copy() {
        let report = Report::failed(Command::Pick, Failure::MissingTool { tool: "hyprpicker" });
        let signal = signals::ColorPickerCommandResult::from(&report);

        assert_eq!(signal.outcome, signals::ColorPickerCommandOutcome::Failed);
        assert_eq!(
            signal.message.as_deref(),
            Some("`hyprpicker` is unavailable")
        );
    }
}
