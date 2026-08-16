//! Host capability diagnostics.
//!
//! Domain values, packaging metadata, and host probing are deliberately split:
//! the catalog says what a capability means while probes say what this host can
//! currently provide.

mod catalog;
mod domain;
mod probe;
mod signal;

use tracing::instrument;

pub use domain::{Capability, Entry, Snapshot, State, Tier};

impl Snapshot {
    /// Reads the cheap, startup-safe capability projection for this host.
    #[instrument]
    pub fn read() -> Self {
        Self {
            entries: catalog::DEFINITIONS
                .iter()
                .map(catalog::Definition::entry)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Capability, Snapshot, State, probe};

    #[test]
    fn snapshot_has_stable_entries() {
        let snapshot = Snapshot::read();

        assert!(snapshot.entries.len() >= 12);
        assert_eq!(snapshot.entries[0].capability, Capability::Hyprland);
        assert!(snapshot.entries.iter().all(|entry| !entry.label.is_empty()));
    }

    #[test]
    fn missing_command_probe_is_false() {
        assert!(!probe::command_exists("__hyprbaric_missing_command__"));
    }

    #[test]
    fn optional_missing_capabilities_carry_a_message() {
        let snapshot = Snapshot::read();

        for entry in snapshot.entries {
            if let State::Missing { message } = entry.state {
                assert!(!message.is_empty());
            }
        }
    }
}
