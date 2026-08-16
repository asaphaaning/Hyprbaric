//! Freedesktop notification center.
//!
//! [`Center`] owns the live notification model exposed to Flutter. The domain
//! module owns notification state transitions, monitor owns D-Bus observation,
//! backend owns notification-daemon commands, and signal owns RINF projection.

mod backend;
mod domain;
mod monitor;
mod signal;

use std::sync::{Arc, Mutex, RwLock};

use tokio::sync::broadcast;
use tracing::instrument;

pub use domain::{Command, Entry, Event, NotificationId, Snapshot, Urgency};

pub type Handle = Arc<Center>;

/// Live notification-center state.
#[derive(Clone)]
pub struct Center {
    events: broadcast::Sender<Snapshot>,
    snapshot: Arc<RwLock<Snapshot>>,
    model: Arc<Mutex<domain::Model>>,
}

impl Center {
    /// Starts the notification monitor and returns the first snapshot.
    #[instrument(skip_all)]
    pub async fn bootstrap() -> (Handle, Snapshot) {
        let (events, _) = broadcast::channel(32);
        let initial_snapshot = Snapshot::empty();
        let center = Arc::new(Self {
            events,
            snapshot: Arc::new(RwLock::new(initial_snapshot.clone())),
            model: Arc::new(Mutex::new(domain::Model::default())),
        });

        let center_for_monitor = Arc::clone(&center);
        match monitor::spawn(move |event| center_for_monitor.accept(event)) {
            Ok(()) => (center, initial_snapshot),
            Err(error) => {
                tracing::warn!("Notification bootstrap failed: {error}");
                let snapshot = Snapshot::unavailable(error.to_string());
                center.publish(snapshot.clone());
                (center, snapshot)
            }
        }
    }

    /// Subscribes to published notification snapshots.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Applies a notification command.
    #[instrument(skip(self))]
    pub async fn apply(&self, command: Command) {
        match command {
            Command::Dismiss(id) => self.dismiss(id).await,
            Command::Clear => self.clear().await,
            Command::SetDoNotDisturb(enabled) => self.set_dnd(enabled),
        }
    }

    /// Dismisses one notification optimistically.
    #[instrument(skip(self), fields(notification_id = id.as_u32()))]
    pub async fn dismiss(&self, id: NotificationId) {
        if let Err(error) = backend::close(id).await {
            tracing::warn!("Notification dismiss failed for {}: {error}", id.as_u32());
        }
        self.forget(id);
    }

    /// Clears the visible notification list optimistically.
    #[instrument(skip(self))]
    pub async fn clear(&self) {
        let ids = self
            .read_snapshot()
            .entries
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();
        for id in ids {
            self.dismiss(id).await;
        }
    }

    /// Toggles Hyprbaric's do-not-disturb mode.
    #[instrument(skip(self), fields(enabled))]
    pub fn set_dnd(&self, enabled: bool) {
        let snapshot = {
            let mut model = lock_mutex(&self.model);
            model.set_dnd(enabled)
        };
        if let Some(snapshot) = snapshot {
            self.publish(snapshot);
        }
    }

    fn accept(&self, event: Event) {
        let snapshot = {
            let mut model = lock_mutex(&self.model);
            model.apply(event)
        };
        if let Some(snapshot) = snapshot {
            self.publish(snapshot);
        }
    }

    fn forget(&self, id: NotificationId) {
        let snapshot = {
            let mut model = lock_mutex(&self.model);
            model.dismiss(id)
        };
        if let Some(snapshot) = snapshot {
            self.publish(snapshot);
        }
    }

    fn publish(&self, snapshot: Snapshot) {
        write_lock(&self.snapshot).clone_from(&snapshot);
        let _ = self.events.send(snapshot);
    }

    fn read_snapshot(&self) -> Snapshot {
        read_lock(&self.snapshot).clone()
    }
}

fn lock_mutex<T>(value: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    value.lock().unwrap_or_else(|poison| poison.into_inner())
}

fn read_lock<T>(value: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    value.read().unwrap_or_else(|poison| poison.into_inner())
}

fn write_lock<T>(value: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    value.write().unwrap_or_else(|poison| poison.into_inner())
}

/// Notification subsystem failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The session bus could not be reached for monitoring.
    #[error("failed to connect to the session bus")]
    ConnectSessionBus(#[source] dbus::Error),
    /// The D-Bus connection could not become a monitor.
    #[error("failed to convert the D-Bus connection into a monitor")]
    BecomeMonitor(#[source] dbus::Error),
    /// The monitor thread could not be spawned.
    #[error("failed to spawn the notification watcher thread")]
    SpawnWatcher(#[source] std::io::Error),
    /// The session bus could not be reached for commands.
    #[error("failed to connect to the session bus for notification commands")]
    ConnectSessionBusZbus(#[source] zbus::Error),
    /// The notification-daemon command proxy could not be created.
    #[error("failed to create the notifications proxy")]
    CreateNotificationsProxy(#[source] zbus::Error),
    /// The notification-daemon close command failed.
    #[error("failed to close notification {0}")]
    CloseNotification(#[source] zbus::Error),
}
