//! Network state published to the bar.
//!
//! These types keep UI-facing network state independent of NetworkManager and
//! `/proc` details. Boundary modules construct [`Snapshot`] values from system
//! data; the runtime only needs to publish them.

/// A UI-facing network snapshot.
///
/// [`Snapshot`] intentionally remains a projection for the bar. It carries the
/// traffic, Wi-Fi list, interface rows, and unavailable copy Flutter renders in
/// one update; NetworkManager-specific availability stays at the boundary that
/// constructs it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    /// Whether Wi-Fi is enabled in NetworkManager.
    pub wifi_enabled: bool,
    /// Whether NetworkManager reports a wireless device.
    pub device_present: bool,
    /// Whether the current snapshot was read around a requested scan.
    pub scanning: bool,
    /// The SSID NetworkManager reports as active.
    pub active_ssid: Option<String>,
    /// Current traffic counters and rates.
    pub traffic: Traffic,
    /// Visible Wi-Fi entries after duplicate SSIDs are merged.
    pub networks: Vec<Entry>,
    /// Network interfaces visible to NetworkManager.
    pub interfaces: Vec<Interface>,
    /// A user-facing unavailable or failure detail.
    pub message: Option<String>,
}

/// One visible Wi-Fi entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The human-facing network name.
    pub ssid: String,
    /// The selected access point address when one is known.
    pub bssid: Option<String>,
    /// Signal strength in NetworkManager's percent-like scale.
    pub strength: u8,
    /// Whether the access point requires a secret.
    pub secure: bool,
    /// The connection state NetworkManager reports for this visible entry.
    pub state: EntryState,
}

/// The display state of one visible Wi-Fi entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EntryState {
    /// The entry is visible but has no active connection transition.
    #[default]
    Available,
    /// The entry represents the active SSID.
    Active,
    /// NetworkManager is connecting the active SSID.
    Connecting,
}

/// Transfer rate and reachability data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Traffic {
    /// Bytes sent by non-loopback interfaces.
    pub upload: Transfer,
    /// Bytes received by non-loopback interfaces.
    pub download: Transfer,
    /// Best-effort reachability latency.
    pub ping_ms: Option<u16>,
}

/// A directional traffic counter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    /// Current byte rate derived from adjacent samples.
    pub bytes_per_second: u64,
    /// Total bytes observed in the current system counter sample.
    pub total_bytes: u64,
}

/// One NetworkManager interface row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interface {
    /// The system interface name.
    pub name: String,
    /// The preferred IP address without CIDR suffix.
    pub address: Option<String>,
    /// Whether NetworkManager reports the interface as activated.
    pub active: bool,
}

impl Snapshot {
    /// Creates a network snapshot for an unavailable system boundary.
    pub(super) fn unavailable(message: impl Into<String>) -> Self {
        Self {
            wifi_enabled: false,
            device_present: false,
            scanning: false,
            active_ssid: None,
            traffic: Traffic::empty(),
            networks: Vec::new(),
            interfaces: Vec::new(),
            message: Some(message.into()),
        }
    }
}

impl Entry {
    /// Merges duplicate SSIDs and sorts the visible list for display.
    ///
    /// NetworkManager can expose multiple access points for the same network.
    /// The bar keeps one candidate per SSID, preferring active and stronger
    /// candidates while preserving security and connecting state across the
    /// merged entries.
    pub(super) fn visible(entries: Vec<Self>) -> Vec<Self> {
        let mut entries = Self::merge_ssids(entries);
        entries.sort_by(|left, right| {
            right
                .state
                .is_active()
                .cmp(&left.state.is_active())
                .then_with(|| right.strength.cmp(&left.strength))
                .then_with(|| left.ssid.to_lowercase().cmp(&right.ssid.to_lowercase()))
        });
        entries
    }

    /// Reduces NetworkManager access points to one entry per SSID.
    fn merge_ssids(entries: Vec<Self>) -> Vec<Self> {
        let mut networks: Vec<Self> = Vec::new();

        for entry in entries {
            if let Some(existing) = networks
                .iter_mut()
                .find(|network| network.ssid == entry.ssid)
            {
                let secure = existing.secure || entry.secure;
                let state = existing.state.merge(entry.state);
                let replace = entry.state.is_active() && !existing.state.is_active()
                    || (entry.state.is_active() == existing.state.is_active()
                        && entry.strength > existing.strength);

                if replace {
                    *existing = Self {
                        secure,
                        state,
                        ..entry
                    };
                } else {
                    existing.secure = secure;
                    existing.state = state;
                }
            } else {
                networks.push(entry);
            }
        }

        networks
    }
}

impl EntryState {
    /// Rebuilds one visible-entry state from NetworkManager transition flags.
    pub(super) const fn from_connection(active: bool, connecting: bool) -> Self {
        if active && connecting {
            Self::Connecting
        } else if active {
            Self::Active
        } else {
            Self::Available
        }
    }

    /// Returns whether this state represents the active SSID.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active | Self::Connecting)
    }

    /// Preserves the strongest connection transition across merged access points.
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Connecting, _) | (_, Self::Connecting) => Self::Connecting,
            (Self::Active, _) | (_, Self::Active) => Self::Active,
            (Self::Available, Self::Available) => Self::Available,
        }
    }
}

impl Traffic {
    /// Creates an empty traffic view for unavailable network state.
    pub(super) fn empty() -> Self {
        Self {
            upload: Transfer::empty(),
            download: Transfer::empty(),
            ping_ms: None,
        }
    }
}

impl Transfer {
    /// Creates an empty directional counter.
    const fn empty() -> Self {
        Self {
            bytes_per_second: 0,
            total_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, EntryState, Snapshot};

    #[test]
    fn unavailable_snapshot_has_no_networks() {
        let snapshot = Snapshot::unavailable("NetworkManager failed");

        assert!(!snapshot.wifi_enabled);
        assert!(!snapshot.device_present);
        assert!(snapshot.networks.is_empty());
        assert!(snapshot.interfaces.is_empty());
        assert_eq!(snapshot.traffic.upload.bytes_per_second, 0);
        assert_eq!(snapshot.traffic.download.bytes_per_second, 0);
        assert_eq!(snapshot.message.as_deref(), Some("NetworkManager failed"));
    }

    #[test]
    fn network_entries_can_mark_active_connection() {
        let entry = Entry {
            ssid: "Fiber_5G".to_string(),
            bssid: Some("aa:bb:cc:dd:ee:ff".to_string()),
            strength: 88,
            secure: true,
            state: EntryState::Active,
        };

        assert!(entry.state.is_active());
        assert_ne!(entry.state, EntryState::Connecting);
        assert!(entry.secure);
        assert_eq!(entry.strength, 88);
    }

    #[test]
    fn visible_entries_collapse_duplicate_ssids_to_the_best_candidate() {
        let networks = Entry::visible(vec![
            Entry {
                ssid: "Haan".to_string(),
                bssid: Some("aa:aa:aa:aa:aa:aa".to_string()),
                strength: 44,
                secure: true,
                state: EntryState::Available,
            },
            Entry {
                ssid: "Haan".to_string(),
                bssid: Some("bb:bb:bb:bb:bb:bb".to_string()),
                strength: 82,
                secure: true,
                state: EntryState::Available,
            },
            Entry {
                ssid: "Cafe".to_string(),
                bssid: None,
                strength: 70,
                secure: false,
                state: EntryState::Active,
            },
        ]);

        assert_eq!(networks.len(), 2);
        assert_eq!(networks[0].ssid, "Cafe");
        assert_eq!(networks[1].ssid, "Haan");
        assert_eq!(networks[1].bssid.as_deref(), Some("bb:bb:bb:bb:bb:bb"));
        assert_eq!(networks[1].strength, 82);
    }
}
