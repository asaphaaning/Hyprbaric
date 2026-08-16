//! RINF projections for screen recording state and reports.

use crate::signals;

use super::{Active, Command, Mode, Report, Snapshot};

impl From<signals::RecordingMode> for Mode {
    fn from(mode: signals::RecordingMode) -> Self {
        match mode {
            signals::RecordingMode::Region => Self::Region,
        }
    }
}

impl From<Mode> for signals::RecordingMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Region => Self::Region,
        }
    }
}

impl From<signals::RecordingRequest> for Command {
    fn from(request: signals::RecordingRequest) -> Self {
        match request.action {
            signals::RecordingAction::Toggle => Self::Toggle {
                mode: request.mode.into(),
            },
        }
    }
}

impl From<Command> for signals::RecordingCommand {
    fn from(command: Command) -> Self {
        match command {
            Command::Toggle { mode } => Self::Toggle { mode: mode.into() },
        }
    }
}

impl From<&Snapshot> for signals::RecordingStatus {
    fn from(snapshot: &Snapshot) -> Self {
        match snapshot {
            Snapshot::Unavailable { message } => Self::Unavailable {
                message: message.clone(),
            },
            Snapshot::Idle => Self::Idle,
            Snapshot::Selecting { mode } => Self::Selecting {
                mode: (*mode).into(),
            },
            Snapshot::Recording { active } => active_status(active, StatusKind::Recording),
            Snapshot::Stopping { active } => active_status(active, StatusKind::Stopping),
        }
    }
}

impl From<&Report> for signals::RecordingCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started(command) => Self::Started {
                command: (*command).into(),
            },
            Report::Saved { command, path } => Self::Saved {
                command: (*command).into(),
                path: path.display().to_string(),
            },
            Report::Cancelled(command) => Self::Cancelled {
                command: (*command).into(),
            },
            Report::Failed { command, failure } => Self::Failed {
                command: (*command).into(),
                message: failure.to_string(),
            },
        }
    }
}

enum StatusKind {
    Recording,
    Stopping,
}

fn active_status(active: &Active, kind: StatusKind) -> signals::RecordingStatus {
    let mode = active.mode().into();
    let path = active.path().display().to_string();
    let started_at_ms = active.started_at_ms();
    match kind {
        StatusKind::Recording => signals::RecordingStatus::Recording {
            mode,
            path,
            started_at_ms,
        },
        StatusKind::Stopping => signals::RecordingStatus::Stopping {
            mode,
            path,
            started_at_ms,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        recording::{Active, Command, Mode, Report, Snapshot},
        signals,
    };

    #[test]
    fn active_snapshot_projects_recording_status() {
        let active = Active::new(Mode::Region, PathBuf::from("/tmp/demo.mp4"), 42);
        let status = signals::RecordingStatus::from(&Snapshot::Recording { active });

        assert!(matches!(
            status,
            signals::RecordingStatus::Recording {
                mode: signals::RecordingMode::Region,
                path,
                started_at_ms: 42,
            } if path == "/tmp/demo.mp4"
        ));
    }

    #[test]
    fn saved_report_preserves_path() {
        let command = Command::Toggle { mode: Mode::Region };
        let result = signals::RecordingCommandResult::from(&Report::saved(
            command,
            PathBuf::from("/tmp/demo.mp4"),
        ));

        assert!(matches!(
            result,
            signals::RecordingCommandResult::Saved { path, .. } if path == "/tmp/demo.mp4"
        ));
    }
}
