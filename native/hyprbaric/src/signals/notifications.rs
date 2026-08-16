use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NotificationEntry {
    pub id: u32,
    pub app: String,
    pub message: String,
    pub created_at_ms: u64,
    pub urgency: NotificationUrgency,
}

#[derive(Deserialize, DartSignal)]
pub struct NotificationDismissRequest {
    pub id: u32,
}

#[derive(Deserialize, DartSignal)]
pub struct NotificationClearRequest {}

#[derive(Deserialize, DartSignal)]
pub struct NotificationSetDoNotDisturb {
    pub enabled: bool,
}

#[derive(Serialize, RustSignal)]
pub struct NotificationStatus {
    pub available: bool,
    pub entries: Vec<NotificationEntry>,
    pub unread_count: u32,
    pub dnd_enabled: bool,
    pub message: Option<String>,
}
