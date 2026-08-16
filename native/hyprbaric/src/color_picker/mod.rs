//! Color picker runtime backed by `hyprpicker`.
//!
//! [`Picker`] is the compact handle used by the controls popup. Domain values
//! live in [`domain`], process details live in [`backend`], and RINF
//! projections live in [`signal`].

mod backend;
mod domain;
mod signal;

use std::sync::Arc;

use tokio::{
    sync::broadcast,
    time::{self, Duration},
};
use tracing::instrument;

use self::backend::Backend;

pub use domain::{Color, Command, Failure, Report};

/// Shared color picker runtime handle.
pub type Handle = Arc<Picker>;

/// Live color picker command entrypoint.
#[derive(Debug)]
pub struct Picker {
    results: broadcast::Sender<Report>,
    backend: Backend,
}

impl Picker {
    /// Creates the color picker runtime handle.
    pub fn new() -> Handle {
        let (results, _) = broadcast::channel(16);
        Arc::new(Self {
            results,
            backend: Backend,
        })
    }

    /// Subscribes to color picker command reports.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    /// Starts one color pick command.
    #[instrument(skip(self))]
    pub fn pick(self: &Arc<Self>) {
        let picker = Arc::clone(self);
        tokio::spawn(async move {
            picker.run(Command::Pick).await;
        });
    }

    #[instrument(skip(self))]
    async fn run(&self, command: Command) {
        drop(self.results.send(Report::started(command)));

        let report = match time::timeout(Duration::from_secs(30), self.backend.pick()).await {
            Ok(Ok(color)) => Report::picked(command, color),
            Ok(Err(Failure::Cancelled)) => Report::cancelled(command),
            Ok(Err(failure)) => Report::failed(command, failure),
            Err(_) => Report::failed(command, Failure::Timeout),
        };

        drop(self.results.send(report));
    }
}
