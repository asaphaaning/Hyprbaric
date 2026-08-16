use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScheduleAction {
    NightLight,
}

#[derive(Serialize, Deserialize, DartSignal, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScheduleCommand {
    SetDailyWindow {
        action: ScheduleAction,
        enabled: bool,
        start_hour: u8,
        stop_hour: u8,
    },
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduleEntry {
    pub action: ScheduleAction,
    pub enabled: bool,
    pub start_hour: u8,
    pub stop_hour: u8,
}

#[derive(Serialize, RustSignal)]
pub struct ScheduleStatus {
    pub entries: Vec<ScheduleEntry>,
}

#[derive(Serialize, RustSignal)]
pub enum ScheduleCommandResult {
    Started {
        command: ScheduleCommand,
    },
    Saved {
        command: ScheduleCommand,
    },
    Failed {
        command: ScheduleCommand,
        message: String,
    },
}
