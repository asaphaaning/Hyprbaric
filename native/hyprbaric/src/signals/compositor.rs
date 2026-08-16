use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkspaceSwitchKind {
    Relative,
    Absolute,
}

#[derive(Deserialize, DartSignal)]
pub struct WorkspaceSwitch {
    pub kind: WorkspaceSwitchKind,
    pub value: i32,
}

#[derive(Serialize, RustSignal)]
pub struct WorkspaceStatus {
    pub id: i32,
    pub name: String,
    pub is_special: bool,
}

#[derive(Serialize, RustSignal)]
pub struct FocusedWindowStatus {
    pub app_name: Option<String>,
    pub title: Option<String>,
    pub hostname: String,
}
