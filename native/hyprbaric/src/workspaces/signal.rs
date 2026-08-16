//! RINF projections for workspace indicator settings.

use crate::signals;

use super::{Command, IndicatorStyle, Report, Snapshot, VisibleRange};

impl From<IndicatorStyle> for signals::WorkspaceIndicatorStyle {
    fn from(style: IndicatorStyle) -> Self {
        match style {
            IndicatorStyle::Roman => Self::Roman,
            IndicatorStyle::Numeric => Self::Numeric,
        }
    }
}

impl From<signals::WorkspaceIndicatorStyle> for IndicatorStyle {
    fn from(style: signals::WorkspaceIndicatorStyle) -> Self {
        match style {
            signals::WorkspaceIndicatorStyle::Roman => Self::Roman,
            signals::WorkspaceIndicatorStyle::Numeric => Self::Numeric,
        }
    }
}

impl From<VisibleRange> for signals::WorkspaceVisibleRange {
    fn from(range: VisibleRange) -> Self {
        match range {
            VisibleRange::Small => Self::Small,
            VisibleRange::Medium => Self::Medium,
            VisibleRange::Large => Self::Large,
        }
    }
}

impl From<signals::WorkspaceVisibleRange> for VisibleRange {
    fn from(range: signals::WorkspaceVisibleRange) -> Self {
        match range {
            signals::WorkspaceVisibleRange::Small => Self::Small,
            signals::WorkspaceVisibleRange::Medium => Self::Medium,
            signals::WorkspaceVisibleRange::Large => Self::Large,
        }
    }
}

impl From<signals::WorkspaceSettingsCommand> for Command {
    fn from(command: signals::WorkspaceSettingsCommand) -> Self {
        match command {
            signals::WorkspaceSettingsCommand::SetIndicatorStyle { indicator_style } => {
                Self::SetIndicatorStyle {
                    indicator_style: indicator_style.into(),
                }
            }
            signals::WorkspaceSettingsCommand::SetClickable { clickable } => {
                Self::SetClickable { clickable }
            }
            signals::WorkspaceSettingsCommand::SetVisibleRange { visible_range } => {
                Self::SetVisibleRange {
                    visible_range: visible_range.into(),
                }
            }
        }
    }
}

impl From<&Snapshot> for signals::WorkspaceSettingsStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            indicator_style: snapshot.indicator_style.into(),
            clickable: snapshot.clickable,
            visible_range: snapshot.visible_range.into(),
            visible_count: snapshot.visible_range.count(),
        }
    }
}

impl From<&Report> for signals::WorkspaceSettingsCommandResult {
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

impl From<&Command> for signals::WorkspaceSettingsCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetIndicatorStyle { indicator_style } => Self::SetIndicatorStyle {
                indicator_style: (*indicator_style).into(),
            },
            Command::SetClickable { clickable } => Self::SetClickable {
                clickable: *clickable,
            },
            Command::SetVisibleRange { visible_range } => Self::SetVisibleRange {
                visible_range: (*visible_range).into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        signals,
        workspaces::{Command, Configuration, IndicatorStyle, Snapshot},
    };

    #[test]
    fn command_projects_from_transport() {
        let command = Command::from(signals::WorkspaceSettingsCommand::SetIndicatorStyle {
            indicator_style: signals::WorkspaceIndicatorStyle::Numeric,
        });

        assert!(matches!(
            command,
            Command::SetIndicatorStyle {
                indicator_style: IndicatorStyle::Numeric
            }
        ));
    }

    #[test]
    fn snapshot_projects_status() {
        let status = signals::WorkspaceSettingsStatus::from(&Snapshot {
            ..Configuration::default().snapshot()
        });

        assert_eq!(
            status.indicator_style,
            signals::WorkspaceIndicatorStyle::Roman
        );
        assert!(status.clickable);
        assert_eq!(status.visible_range, signals::WorkspaceVisibleRange::Medium);
        assert_eq!(status.visible_count, 7);
    }
}
