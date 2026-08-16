use rinf::{RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityId {
    Hyprland,
    LayerShell,
    GlobalShortcuts,
    Audio,
    Brightness,
    Network,
    Notifications,
    SystemTray,
    Launcher,
    Screenshot,
    Clipboard,
    ColorPicker,
    Recording,
    NightLight,
    Caffeine,
    Power,
    UserDirectories,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityTier {
    Core,
    Service,
    Optional,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityAvailability {
    Available,
    Degraded,
    Missing,
}

#[derive(Serialize, Deserialize, SignalPiece, Clone, Debug, PartialEq, Eq)]
pub struct CapabilityEntry {
    pub capability: CapabilityId,
    pub label: String,
    pub detail: String,
    pub tier: CapabilityTier,
    pub availability: CapabilityAvailability,
    pub message: Option<String>,
    pub features: Vec<String>,
    pub commands: Vec<String>,
    pub arch_packages: Vec<String>,
    pub debian_packages: Vec<String>,
    pub rpm_packages: Vec<String>,
}

#[derive(Serialize, RustSignal)]
pub struct CapabilityStatus {
    pub entries: Vec<CapabilityEntry>,
}
