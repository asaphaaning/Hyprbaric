//! StatusNotifier/AppIndicator tray runtime.
//!
//! [`Tray`] is the small public handle used by the bar. Domain vocabulary lives
//! in [`domain`], system-tray watching lives in [`watcher`], activation commands
//! live in [`backend`], icon resolution lives in [`icons`], and generated RINF
//! projections live in [`signal`].

mod backend;
mod domain;
mod icons;
mod signal;
mod watcher;

use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;
use tracing::instrument;

pub use domain::{
    Activation, ActivationKind, Icon, Item, ItemId, Menu, MenuActivation, MenuItem, MenuItemId,
    MenuItemKind, Outcome, Position, Snapshot, Status,
};

pub type Handle = Arc<Tray>;

/// Live tray state and command boundary.
#[derive(Clone)]
pub struct Tray {
    client: Option<watcher::ClientHandle>,
    events: broadcast::Sender<Snapshot>,
    latest: Arc<Mutex<Snapshot>>,
}

impl Tray {
    /// Starts the system tray watcher and returns the first snapshot.
    #[instrument(skip_all)]
    pub async fn bootstrap() -> (Handle, Snapshot) {
        let (events, _) = broadcast::channel(32);
        let latest = Arc::new(Mutex::new(Snapshot::default()));
        let client = match watcher::connect().await {
            Ok(client) => client,
            Err(error) => {
                tracing::warn!(%error, "Tray bootstrap failed");
                let snapshot = Snapshot::unavailable(error.to_string());
                replace_snapshot(&latest, snapshot.clone());
                let tray = Arc::new(Self {
                    client: None,
                    events,
                    latest,
                });
                return (tray, snapshot);
            }
        };

        let initial_snapshot = watcher::spawn(
            events.clone(),
            Arc::clone(&client),
            icons::Index::new(),
            Arc::clone(&latest),
        );

        (
            Arc::new(Self {
                client: Some(client),
                events,
                latest,
            }),
            initial_snapshot,
        )
    }

    /// Subscribes to tray snapshots.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Returns the latest tray snapshot for lossless subscriber startup.
    pub fn snapshot(&self) -> Snapshot {
        match self.latest.lock() {
            Ok(snapshot) => snapshot.clone(),
            Err(poisoned) => {
                tracing::warn!("Tray snapshot cache lock was poisoned; recovering current state");
                poisoned.into_inner().clone()
            }
        }
    }

    /// Activates one tray item.
    #[instrument(
        skip(self),
        fields(
            item_id = %activation.item_id.as_str(),
            x = activation.position.x(),
            y = activation.position.y(),
        )
    )]
    pub async fn activate(&self, activation: Activation) -> Result<Outcome, Error> {
        let Some(client) = &self.client else {
            return Err(Error::Unavailable);
        };

        backend::activate(client, activation).await
    }

    /// Activates one tray menu row.
    #[instrument(
        skip(self),
        fields(
            item_id = %activation.item_id.as_str(),
            menu_item_id = activation.menu_item_id.get(),
        )
    )]
    pub async fn activate_menu_item(&self, activation: MenuActivation) -> Result<Outcome, Error> {
        let Some(client) = &self.client else {
            return Err(Error::Unavailable);
        };

        backend::activate_menu_item(client, activation).await
    }
}

fn replace_snapshot(latest: &Arc<Mutex<Snapshot>>, snapshot: Snapshot) {
    match latest.lock() {
        Ok(mut current) => {
            *current = snapshot;
        }
        Err(poisoned) => {
            tracing::warn!("Tray snapshot cache lock was poisoned; recovering current state");
            *poisoned.into_inner() = snapshot;
        }
    }
}

/// Tray subsystem failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system tray is unavailable")]
    Unavailable,
    #[error("failed to connect to the StatusNotifier watcher")]
    Watcher(#[source] system_tray::error::Error),
    #[error("tray item `{item_id}` has no menu")]
    MenuUnavailable { item_id: ItemId },
    /// A tray pixmap reported invalid dimensions.
    #[error("tray icon pixmap dimensions are invalid: {width}x{height}")]
    InvalidPixmapDimensions { width: i32, height: i32 },
    /// A tray pixmap's byte count did not match its dimensions.
    #[error("tray icon pixmap length was {actual}, expected {expected}")]
    InvalidPixmapLength { expected: usize, actual: usize },
    /// A pixmap icon could not be encoded as PNG.
    #[error("failed to encode tray icon as PNG")]
    PngEncode(#[source] image::ImageError),
    /// The session bus could not be reached for a tray command.
    #[error("failed to connect to the session bus for tray activation")]
    ConnectSessionBus(#[source] zbus::Error),
    /// A tray item proxy could not be created.
    #[error("failed to create tray item proxy")]
    CreateItemProxy(#[source] zbus::Error),
    /// A tray item's primary activation could not be sent.
    #[error("failed to send tray item Activate request")]
    Activate(#[source] zbus::Error),
    /// A tray item's secondary activation could not be sent.
    #[error("failed to send tray item SecondaryActivate request")]
    SecondaryActivate(#[source] zbus::Error),
    /// A tray item's Ayatana secondary activation could not be sent.
    #[error("failed to send tray item XAyatanaSecondaryActivate request")]
    AyatanaSecondaryActivate(#[source] zbus::Error),
    /// A tray item's context menu could not be opened.
    #[error("failed to send tray item context-menu request")]
    ContextMenu(#[source] zbus::Error),
    /// A tray menu item could not be activated.
    #[error("failed to send tray menu item activation request")]
    MenuItemActivate(#[source] system_tray::error::Error),
}
