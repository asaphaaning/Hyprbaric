//! RINF projections for host capability diagnostics.

use crate::signals;

use super::{Capability, Entry, Snapshot, State, Tier};

impl From<Capability> for signals::CapabilityId {
    fn from(capability: Capability) -> Self {
        match capability {
            Capability::Hyprland => Self::Hyprland,
            Capability::LayerShell => Self::LayerShell,
            Capability::GlobalShortcuts => Self::GlobalShortcuts,
            Capability::Audio => Self::Audio,
            Capability::Brightness => Self::Brightness,
            Capability::Network => Self::Network,
            Capability::Notifications => Self::Notifications,
            Capability::SystemTray => Self::SystemTray,
            Capability::Launcher => Self::Launcher,
            Capability::Screenshot => Self::Screenshot,
            Capability::Clipboard => Self::Clipboard,
            Capability::ColorPicker => Self::ColorPicker,
            Capability::Recording => Self::Recording,
            Capability::NightLight => Self::NightLight,
            Capability::Caffeine => Self::Caffeine,
            Capability::Power => Self::Power,
            Capability::UserDirectories => Self::UserDirectories,
        }
    }
}

impl From<Tier> for signals::CapabilityTier {
    fn from(tier: Tier) -> Self {
        match tier {
            Tier::Core => Self::Core,
            Tier::Service => Self::Service,
            Tier::Optional => Self::Optional,
        }
    }
}

impl From<&State> for signals::CapabilityAvailability {
    fn from(state: &State) -> Self {
        match state {
            State::Available => Self::Available,
            State::Degraded { .. } => Self::Degraded,
            State::Missing { .. } => Self::Missing,
        }
    }
}

impl State {
    fn message(&self) -> Option<String> {
        match self {
            Self::Available => None,
            Self::Degraded { message } | Self::Missing { message } => Some(message.clone()),
        }
    }
}

impl From<&Snapshot> for signals::CapabilityStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            entries: snapshot.entries.iter().map(Into::into).collect(),
        }
    }
}

impl From<&Entry> for signals::CapabilityEntry {
    fn from(entry: &Entry) -> Self {
        Self {
            capability: entry.capability.into(),
            label: entry.label.to_owned(),
            detail: entry.detail.to_owned(),
            tier: entry.tier.into(),
            availability: (&entry.state).into(),
            message: entry.state.message(),
            features: strings(entry.features),
            commands: strings(entry.commands),
            arch_packages: strings(entry.arch_packages),
            debian_packages: strings(entry.debian_packages),
            rpm_packages: strings(entry.rpm_packages),
        }
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{capabilities::Snapshot, signals};

    #[test]
    fn snapshot_projects_all_entries() {
        let snapshot = Snapshot::read();
        let status = signals::CapabilityStatus::from(&snapshot);

        assert_eq!(status.entries.len(), snapshot.entries.len());
        assert_eq!(
            status.entries[0].capability,
            signals::CapabilityId::Hyprland
        );
    }
}
