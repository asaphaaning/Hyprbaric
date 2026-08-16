//! RINF projections for screenshot reports.

use crate::signals;

use super::{Clipboard, Command, Mode, Report};

impl From<signals::ScreenshotMode> for Mode {
    fn from(mode: signals::ScreenshotMode) -> Self {
        match mode {
            signals::ScreenshotMode::Region => Self::Region,
            signals::ScreenshotMode::Window => Self::Window,
            signals::ScreenshotMode::FullScreen => Self::FullScreen,
        }
    }
}

impl From<Mode> for signals::ScreenshotMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Region => Self::Region,
            Mode::Window => Self::Window,
            Mode::FullScreen => Self::FullScreen,
        }
    }
}

impl From<Command> for signals::ScreenshotCommand {
    fn from(command: Command) -> Self {
        match command {
            Command::Capture(_) => Self::Capture,
        }
    }
}

impl From<&Report> for signals::ScreenshotCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started { command } => Self {
                command: (*command).into(),
                mode: command.mode().into(),
                outcome: signals::ScreenshotCommandOutcome::Started,
                path: None,
                message: None,
            },
            Report::Saved { command, saved } => Self {
                command: (*command).into(),
                mode: command.mode().into(),
                outcome: signals::ScreenshotCommandOutcome::Saved,
                path: Some(saved.path().to_string_lossy().into_owned()),
                message: clipboard_message(saved.clipboard()),
            },
            Report::Cancelled { command } => Self {
                command: (*command).into(),
                mode: command.mode().into(),
                outcome: signals::ScreenshotCommandOutcome::Cancelled,
                path: None,
                message: None,
            },
            Report::Failed { command, failure } => Self {
                command: (*command).into(),
                mode: command.mode().into(),
                outcome: signals::ScreenshotCommandOutcome::Failed,
                path: None,
                message: Some(failure.to_string()),
            },
        }
    }
}

fn clipboard_message(clipboard: &Clipboard) -> Option<String> {
    match clipboard {
        Clipboard::Copied => None,
        Clipboard::Failed { message } => {
            Some(format!("Saved, but clipboard copy failed: {message}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        screenshot::{Clipboard, Command, Failure, Mode, Report, Saved},
        signals,
    };

    #[test]
    fn screenshot_modes_round_trip() {
        for signal in [
            signals::ScreenshotMode::Region,
            signals::ScreenshotMode::Window,
            signals::ScreenshotMode::FullScreen,
        ] {
            let domain = Mode::from(signal);
            assert_eq!(signals::ScreenshotMode::from(domain), signal);
        }
    }

    #[test]
    fn saved_report_preserves_path_and_clipboard_warning() {
        let command = Command::Capture(Mode::Region);
        let report = Report::saved(
            command,
            Saved::new(
                PathBuf::from("/tmp/screen.png"),
                Clipboard::Failed {
                    message: "wl-copy failed".to_owned(),
                },
            ),
        );
        let signal = signals::ScreenshotCommandResult::from(&report);

        assert_eq!(signal.command, signals::ScreenshotCommand::Capture);
        assert_eq!(signal.mode, signals::ScreenshotMode::Region);
        assert_eq!(signal.outcome, signals::ScreenshotCommandOutcome::Saved);
        assert_eq!(signal.path.as_deref(), Some("/tmp/screen.png"));
        assert_eq!(
            signal.message.as_deref(),
            Some("Saved, but clipboard copy failed: wl-copy failed")
        );
    }

    #[test]
    fn cancelled_report_maps_without_message() {
        let report = Report::cancelled(Command::Capture(Mode::Window));
        let signal = signals::ScreenshotCommandResult::from(&report);

        assert_eq!(signal.outcome, signals::ScreenshotCommandOutcome::Cancelled);
        assert_eq!(signal.mode, signals::ScreenshotMode::Window);
        assert_eq!(signal.message, None);
    }

    #[test]
    fn failed_report_uses_failure_copy() {
        let report = Report::failed(
            Command::Capture(Mode::FullScreen),
            Failure::MissingTool { tool: "grim" },
        );
        let signal = signals::ScreenshotCommandResult::from(&report);

        assert_eq!(signal.outcome, signals::ScreenshotCommandOutcome::Failed);
        assert_eq!(signal.mode, signals::ScreenshotMode::FullScreen);
        assert_eq!(signal.message.as_deref(), Some("`grim` is unavailable"));
    }
}
