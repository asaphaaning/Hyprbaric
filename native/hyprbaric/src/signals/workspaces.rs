use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkspaceIndicatorStyle {
    Roman,
    Numeric,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkspaceVisibleRange {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, DartSignal, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WorkspaceSettingsCommand {
    SetIndicatorStyle {
        indicator_style: WorkspaceIndicatorStyle,
    },
    SetClickable {
        clickable: bool,
    },
    SetVisibleRange {
        visible_range: WorkspaceVisibleRange,
    },
}

#[derive(Serialize, RustSignal)]
pub struct WorkspaceSettingsStatus {
    pub indicator_style: WorkspaceIndicatorStyle,
    pub clickable: bool,
    pub visible_range: WorkspaceVisibleRange,
    pub visible_count: u8,
}

#[derive(Serialize, RustSignal)]
pub enum WorkspaceSettingsCommandResult {
    Started {
        command: WorkspaceSettingsCommand,
    },
    Saved {
        command: WorkspaceSettingsCommand,
    },
    Failed {
        command: WorkspaceSettingsCommand,
        message: String,
    },
}
