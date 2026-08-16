use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenshotMode {
    Region,
    Window,
    FullScreen,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenshotCommand {
    Capture,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenshotCommandOutcome {
    Started,
    Saved,
    Cancelled,
    Failed,
}

#[derive(Deserialize, DartSignal)]
pub struct ScreenshotCaptureRequest {
    pub mode: ScreenshotMode,
}

#[derive(Serialize, RustSignal)]
pub struct ScreenshotCommandResult {
    pub command: ScreenshotCommand,
    pub mode: ScreenshotMode,
    pub outcome: ScreenshotCommandOutcome,
    pub path: Option<String>,
    pub message: Option<String>,
}
