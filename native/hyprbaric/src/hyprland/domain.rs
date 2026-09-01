//! Typed projections and commands independent of the Hyprland crate.

use std::collections::BTreeSet;

/// A Hyprland workspace identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceId(i32);

/// A typed compositor request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    SwitchWorkspace(WorkspaceTarget),
}

/// A legal workspace switch target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceTarget {
    Relative(i32),
    Absolute(WorkspaceId),
}

/// The active workspace state consumed by the bar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceSnapshot {
    /// Numeric Hyprland workspace identifier.
    pub id: WorkspaceId,
    /// User-visible workspace name.
    pub name: String,
    /// Whether the active workspace is a special workspace.
    pub is_special: bool,
    /// The regular workspaces that currently contain one or more windows.
    pub occupied: WorkspaceOccupancy,
}

impl WorkspaceSnapshot {
    pub(super) fn new(
        id: i32,
        name: String,
        is_special: bool,
        occupied: WorkspaceOccupancy,
    ) -> Self {
        Self {
            id: WorkspaceId::observed(id),
            name,
            is_special,
            occupied,
        }
    }
}

/// The regular Hyprland workspaces that currently contain windows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkspaceOccupancy(BTreeSet<WorkspaceId>);

impl WorkspaceOccupancy {
    /// Builds occupancy from workspace IDs whose corresponding workspace has windows.
    pub(super) fn from_occupied_ids(ids: impl IntoIterator<Item = i32>) -> Self {
        Self(ids.into_iter().map(WorkspaceId::observed).collect())
    }

    /// Iterates over the IDs of occupied workspaces in ascending order.
    pub fn ids(&self) -> impl Iterator<Item = WorkspaceId> + '_ {
        self.0.iter().copied()
    }
}

/// The active client identity consumed by the center bar cluster.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedWindowSnapshot {
    /// Normalized application class, when Hyprland reports one.
    pub app_name: Option<String>,
    /// Normalized window title, when Hyprland reports one.
    pub title: Option<String>,
    /// Hostname displayed when no client identity is available.
    pub hostname: String,
}

impl FocusedWindowSnapshot {
    pub(super) fn new(app_name: Option<&str>, title: Option<&str>, hostname: &str) -> Self {
        Self {
            app_name: app_name.and_then(normalize_app_name),
            title: title.and_then(normalize_title),
            hostname: hostname.to_owned(),
        }
    }
}

impl WorkspaceId {
    pub const fn get(self) -> i32 {
        self.0
    }

    const fn observed(value: i32) -> Self {
        Self(value)
    }

    pub fn target(value: i32) -> Option<Self> {
        (value > 0).then_some(Self(value))
    }
}

impl WorkspaceTarget {
    pub const fn relative(offset: i32) -> Self {
        Self::Relative(offset)
    }

    pub fn absolute(id: i32) -> Option<Self> {
        WorkspaceId::target(id).map(Self::Absolute)
    }
}

fn normalize_title(title: &str) -> Option<String> {
    normalize(title)
}

fn normalize_app_name(name: &str) -> Option<String> {
    normalize(name)
}

fn normalize(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        FocusedWindowSnapshot, WorkspaceId, WorkspaceOccupancy, WorkspaceTarget,
        normalize_app_name, normalize_title,
    };

    #[test]
    fn normalize_title_rejects_blank_values() {
        assert_eq!(normalize_title("   "), None);
        assert_eq!(normalize_title(" terminal "), Some("terminal".to_owned()));
    }

    #[test]
    fn normalize_app_name_rejects_blank_values() {
        assert_eq!(normalize_app_name("   "), None);
        assert_eq!(normalize_app_name(" zed "), Some("zed".to_owned()));
    }

    #[test]
    fn focused_window_snapshot_preserves_hostname_without_title() {
        let snapshot = FocusedWindowSnapshot {
            app_name: None,
            title: None,
            hostname: "workstation".to_owned(),
        };

        assert_eq!(snapshot.app_name, None);
        assert_eq!(snapshot.title, None);
        assert_eq!(snapshot.hostname, "workstation");
    }

    #[test]
    fn absolute_targets_reject_non_positive_ids() {
        assert_eq!(WorkspaceTarget::absolute(0), None);
        assert_eq!(WorkspaceTarget::absolute(-1), None);
        assert!(WorkspaceTarget::absolute(1).is_some());
    }

    #[test]
    fn workspace_occupancy_contains_only_observed_workspace_ids() {
        let occupancy = WorkspaceOccupancy::from_occupied_ids([3, 1, 3]);

        assert_eq!(
            occupancy.ids().map(WorkspaceId::get).collect::<Vec<_>>(),
            vec![1, 3]
        );
    }
}
