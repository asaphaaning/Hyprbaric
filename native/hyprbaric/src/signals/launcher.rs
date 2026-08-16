use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppLauncherPhase {
    Loading,
    Ready,
    Failed,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppLaunchOutcome {
    Started,
    Failed,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AppLauncherEntry {
    pub id: String,
    pub name: String,
    pub subtitle: Option<String>,
    pub icon_name: Option<String>,
    pub icon_path: Option<String>,
    pub terminal: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct AppLauncherQuery {
    pub query: String,
}

#[derive(Deserialize, DartSignal)]
pub struct AppLaunchRequest {
    pub id: String,
}

#[derive(Serialize, RustSignal)]
pub struct AppLauncherResults {
    pub phase: AppLauncherPhase,
    pub query: String,
    pub entries: Vec<AppLauncherEntry>,
    pub message: Option<String>,
}

#[derive(Serialize, RustSignal)]
pub struct AppLaunchResult {
    pub id: String,
    pub outcome: AppLaunchOutcome,
    pub message: Option<String>,
}
