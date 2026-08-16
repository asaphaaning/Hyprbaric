//! Caffeine runtime backed by login1 inhibitors.
//!
//! [`Caffeine`] owns the live inhibitor fd used by the controls popup. The
//! domain module defines the public vocabulary, while [`login1`] owns the
//! system bus boundary.

mod domain;
mod login1;
mod signal;

use std::sync::Arc;

use tokio::sync::{Mutex, broadcast};
use tracing::instrument;

pub use domain::{Command, Report, Snapshot};

/// Shared Caffeine runtime handle.
pub type Handle = Arc<Caffeine>;

/// Live login1 inhibitor controller.
pub struct Caffeine {
    events: broadcast::Sender<Snapshot>,
    results: broadcast::Sender<Report>,
    state: Mutex<Running>,
}

enum Running {
    Disabled,
    Enabled { guard: login1::Guard },
    Unavailable,
}

impl Caffeine {
    /// Bootstraps Caffeine state without acquiring an inhibitor.
    #[instrument(skip_all)]
    pub async fn bootstrap() -> (Handle, Snapshot) {
        let (state, snapshot) = match login1::probe().await {
            Ok(()) => (Running::Disabled, Snapshot::Available { enabled: false }),
            Err(error) => {
                let message = error.to_string();
                (Running::Unavailable, Snapshot::Unavailable { message })
            }
        };

        let (events, _) = broadcast::channel(16);
        let (results, _) = broadcast::channel(8);
        let caffeine = Arc::new(Self {
            events,
            results,
            state: Mutex::new(state),
        });

        (caffeine, snapshot)
    }

    /// Subscribes to Caffeine [`Snapshot`] changes.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Subscribes to command [`Report`] updates.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    /// Enables or disables Caffeine.
    #[instrument(skip(self))]
    pub async fn set_enabled(&self, enabled: bool) {
        self.apply(Command::SetEnabled { enabled }).await;
    }

    #[instrument(skip(self, command))]
    async fn apply(&self, command: Command) {
        drop(self.results.send(Report::Started(command)));

        let result = match command {
            Command::SetEnabled { enabled } => self.set_running(enabled).await,
        };

        match result {
            Ok(snapshot) => {
                drop(self.results.send(Report::Saved(command)));
                drop(self.events.send(snapshot));
            }
            Err(error) => {
                let message = error.to_string();
                let snapshot = Snapshot::Unavailable {
                    message: message.clone(),
                };
                let mut state = self.state.lock().await;
                *state = Running::Unavailable;
                drop(self.results.send(Report::Failed { command, message }));
                drop(self.events.send(snapshot));
            }
        }
    }

    async fn set_running(&self, enabled: bool) -> Result<Snapshot, Error> {
        if enabled {
            if matches!(*self.state.lock().await, Running::Enabled { .. }) {
                return Ok(Snapshot::Available { enabled: true });
            }

            let guard = login1::inhibit().await?;
            let mut state = self.state.lock().await;
            *state = Running::Enabled { guard };
            Ok(Snapshot::Available { enabled: true })
        } else {
            let mut state = self.state.lock().await;
            *state = Running::Disabled;
            Ok(Snapshot::Available { enabled: false })
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        match self {
            Self::Enabled { guard } => {
                let _ = guard;
            }
            Self::Disabled | Self::Unavailable => {}
        }
    }
}

/// A Caffeine runtime or boundary error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Could not connect to the system bus.
    #[error("failed to connect to the system bus")]
    ConnectSystemBus(#[source] zbus::Error),
    /// login1 proxy creation failed.
    #[error("failed to connect to login1")]
    CreateManagerProxy(#[source] zbus::Error),
    /// login1 rejected the inhibitor.
    #[error("failed to enable Caffeine through login1")]
    Inhibit(#[source] zbus::Error),
}
