//! RINF projections for notification state and commands.

use crate::signals;

use super::{Command, Entry, NotificationId, Snapshot, Urgency};

impl From<signals::NotificationDismissRequest> for Command {
    fn from(request: signals::NotificationDismissRequest) -> Self {
        Self::Dismiss(NotificationId::new(request.id))
    }
}

impl From<signals::NotificationClearRequest> for Command {
    fn from(_: signals::NotificationClearRequest) -> Self {
        Self::Clear
    }
}

impl From<signals::NotificationSetDoNotDisturb> for Command {
    fn from(request: signals::NotificationSetDoNotDisturb) -> Self {
        Self::SetDoNotDisturb(request.enabled)
    }
}

impl From<&Snapshot> for signals::NotificationStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            available: snapshot.available,
            entries: snapshot.entries.iter().map(Into::into).collect(),
            unread_count: snapshot.unread_count,
            dnd_enabled: snapshot.dnd_enabled,
            message: snapshot.message.clone(),
        }
    }
}

impl From<&Entry> for signals::NotificationEntry {
    fn from(entry: &Entry) -> Self {
        Self {
            id: entry.id.as_u32(),
            app: entry.app.clone(),
            message: entry.message.clone(),
            created_at_ms: entry.created_at_ms,
            urgency: entry.urgency.into(),
        }
    }
}

impl From<Urgency> for signals::NotificationUrgency {
    fn from(urgency: Urgency) -> Self {
        match urgency {
            Urgency::Low => Self::Low,
            Urgency::Normal => Self::Normal,
            Urgency::Critical => Self::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        notifications::{Entry, NotificationId, Snapshot, Urgency},
        signals,
    };

    #[test]
    fn urgency_projects_to_signal_vocabulary() {
        assert_eq!(
            signals::NotificationUrgency::from(Urgency::Low),
            signals::NotificationUrgency::Low
        );
        assert_eq!(
            signals::NotificationUrgency::from(Urgency::Normal),
            signals::NotificationUrgency::Normal
        );
        assert_eq!(
            signals::NotificationUrgency::from(Urgency::Critical),
            signals::NotificationUrgency::Critical
        );
    }

    #[test]
    fn snapshot_projection_preserves_status_fields() {
        let snapshot = Snapshot {
            available: true,
            entries: vec![Entry {
                id: NotificationId::new(9),
                app: "GitHub".to_string(),
                message: "Merged".to_string(),
                created_at_ms: 42,
                urgency: Urgency::Critical,
            }],
            unread_count: 1,
            dnd_enabled: true,
            message: Some("hello".to_string()),
        };

        let signal = signals::NotificationStatus::from(&snapshot);

        assert!(signal.available);
        assert_eq!(signal.unread_count, 1);
        assert!(signal.dnd_enabled);
        assert_eq!(signal.message.as_deref(), Some("hello"));
        assert_eq!(signal.entries[0].id, 9);
        assert_eq!(signal.entries[0].app, "GitHub");
        assert_eq!(signal.entries[0].message, "Merged");
        assert_eq!(
            signal.entries[0].urgency,
            signals::NotificationUrgency::Critical
        );
    }
}
