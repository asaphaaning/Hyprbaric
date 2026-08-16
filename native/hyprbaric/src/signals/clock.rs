use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CalendarCommand {
    PreviousMonth,
    Today,
    NextMonth,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CalendarDay {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub current_month: bool,
    pub today: bool,
}

#[derive(Deserialize, DartSignal)]
pub struct ClockCalendarRequest {
    pub command: CalendarCommand,
}

#[derive(Serialize, RustSignal)]
pub struct ClockStatus {
    pub time_label: String,
    pub date_label: String,
    pub month_label: String,
    pub week_number: i32,
    pub utc_offset: String,
    pub days: Vec<CalendarDay>,
}
