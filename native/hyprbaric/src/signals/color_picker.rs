use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorPickerCommand {
    Pick,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorPickerCommandOutcome {
    Started,
    Picked,
    Cancelled,
    Failed,
}

#[derive(Deserialize, DartSignal)]
pub struct ColorPickRequest;

#[derive(Serialize, RustSignal)]
pub struct ColorPickerCommandResult {
    pub command: ColorPickerCommand,
    pub outcome: ColorPickerCommandOutcome,
    pub color: Option<String>,
    pub message: Option<String>,
}
