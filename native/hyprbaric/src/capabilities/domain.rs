//! Capability diagnostic vocabulary.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub capability: Capability,
    pub label: &'static str,
    pub detail: &'static str,
    pub tier: Tier,
    pub state: State,
    pub features: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub arch_packages: &'static [&'static str],
    pub debian_packages: &'static [&'static str],
    pub rpm_packages: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Capability {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tier {
    Core,
    Service,
    Optional,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    Available,
    Degraded { message: String },
    Missing { message: String },
}
