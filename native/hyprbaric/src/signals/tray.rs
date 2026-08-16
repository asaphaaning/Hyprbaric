use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrayItemStatus {
    Unknown,
    Passive,
    Active,
    NeedsAttention,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrayIconKind {
    None,
    ThemePath,
    PngBytes,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrayIcon {
    pub kind: TrayIconKind,
    pub path: Option<String>,
    pub png_bytes: Option<Vec<u8>>,
    pub symbolic: bool,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrayItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TrayItemStatus,
    pub icon: TrayIcon,
}

#[derive(Deserialize, DartSignal)]
pub struct TrayActivateRequest {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub kind: TrayActivationKind,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrayActivationKind {
    Primary,
    ContextMenu,
}

#[derive(Deserialize, DartSignal)]
pub struct TrayMenuItemActivateRequest {
    pub item_id: String,
    pub menu_item_id: i32,
}

#[derive(Serialize, RustSignal)]
pub struct TrayStatus {
    pub items: Vec<TrayItem>,
    pub message: Option<String>,
}

#[derive(Serialize, RustSignal)]
pub struct TrayMenuStatus {
    pub item_id: String,
    pub x: i32,
    pub y: i32,
    pub items: Vec<TrayMenuItem>,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrayMenuItemKind {
    Standard,
    Separator,
}

#[derive(Serialize, SignalPiece, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrayMenuItem {
    pub id: i32,
    pub label: String,
    pub enabled: bool,
    pub kind: TrayMenuItemKind,
    pub depth: u8,
}
