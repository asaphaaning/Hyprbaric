//! First-run setup-guide policy and persistence.

mod domain;
mod settings;
mod signal;

use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::{Mutex, broadcast};
use tracing::instrument;

pub use domain::{Command, Completion, Outcome, Report, Startup, Status};

/// Setup-guide configuration loaded from `[setup]`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Configuration {
    startup: Startup,
    completed: Completion,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            startup: Startup::Once,
            completed: Completion::PENDING,
        }
    }
}

impl Configuration {
    /// Returns the automatic startup policy.
    pub const fn startup(&self) -> Startup {
        self.startup
    }

    /// Returns the persisted completion fact.
    pub const fn completion(&self) -> Completion {
        self.completed
    }

    /// Projects configuration into an exhaustive UI status.
    pub const fn status(&self) -> Status {
        match (self.startup, self.completed.is_complete()) {
            (Startup::Never, _) => Status::Disabled,
            (Startup::Once, true) => Status::Complete,
            (Startup::Once, false) => Status::Required,
        }
    }

    /// Returns this configuration with completion acknowledged.
    #[must_use]
    pub const fn completed(&self) -> Self {
        Self {
            startup: self.startup,
            completed: Completion::COMPLETE,
        }
    }
}

/// Shared setup-guide runtime handle.
pub type Handle = Arc<Guide>;

/// Persists setup completion and publishes typed state transitions.
pub struct Guide {
    state: Mutex<Configuration>,
    events: broadcast::Sender<Status>,
    reports: broadcast::Sender<Report>,
}

impl Guide {
    /// Creates the setup runtime and its initial status.
    pub fn bootstrap(configuration: &Configuration) -> (Handle, Status) {
        let (events, _) = broadcast::channel(8);
        let (reports, _) = broadcast::channel(8);
        let status = configuration.status();
        let guide = Arc::new(Self {
            state: Mutex::new(configuration.clone()),
            events,
            reports,
        });

        (guide, status)
    }

    /// Applies one setup command.
    #[instrument(name = "hyprbaric::setup::guide::apply", skip(self))]
    pub async fn apply(&self, command: Command) {
        drop(self.reports.send(Report::Started(command)));
        let current = self.state.lock().await.clone();

        match settings::complete(&current) {
            Ok(next) => {
                *self.state.lock().await = next;
                drop(self.events.send(Status::Complete));
                drop(self.reports.send(Report::Saved(command)));
            }
            Err(error) => {
                drop(self.reports.send(Report::Failed {
                    command,
                    message: error.to_string(),
                }));
            }
        }
    }

    /// Subscribes to setup statuses.
    pub fn subscribe(&self) -> broadcast::Receiver<Status> {
        self.events.subscribe()
    }

    /// Subscribes to setup command reports.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.reports.subscribe()
    }
}

/// Setup-guide failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The `[setup]` configuration item is not a TOML table.
    #[error("setup configuration must be a TOML table")]
    InvalidTable,
    /// The user configuration could not be updated.
    #[error("failed to persist setup completion")]
    Configuration(#[from] crate::config::Error),
}

#[cfg(test)]
mod tests {
    use super::{Configuration, Status};

    #[test]
    fn startup_status_is_exhaustive() {
        let required = Configuration::default();
        let complete = toml::from_str::<Configuration>("completed = true")
            .expect("completed setup should parse");
        let disabled = toml::from_str::<Configuration>("startup = \"never\"")
            .expect("disabled setup should parse");

        assert_eq!(required.status(), Status::Required);
        assert_eq!(complete.status(), Status::Complete);
        assert_eq!(disabled.status(), Status::Disabled);
    }
}
