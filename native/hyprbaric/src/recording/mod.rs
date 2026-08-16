//! Screen recording runtime backed by `wf-recorder`.
//!
//! [`Recorder`] owns the live child process and publishes typed state for the
//! controls popup. Domain values live in [`domain`], process/filesystem
//! boundaries live in [`backend`], and RINF projections live in [`signal`].

mod backend;
mod domain;
mod signal;

use std::{mem, sync::Arc};

use tokio::{
    process::Child,
    sync::{Mutex, broadcast},
    time::{self, Duration},
};
use tracing::instrument;

use self::backend::Backend;

pub use domain::{Active, Area, Command, Failure, Mode, Report, Snapshot};

/// Shared recorder runtime handle.
pub type Handle = Arc<Recorder>;

/// Live screen recorder entrypoint.
#[derive(Debug)]
pub struct Recorder {
    events: broadcast::Sender<Snapshot>,
    results: broadcast::Sender<Report>,
    backend: Backend,
    state: Mutex<State>,
}

#[derive(Debug)]
enum State {
    Unavailable { message: String },
    Idle,
    Selecting { mode: Mode },
    Recording { child: Child, active: Active },
    Stopping { active: Active },
}

enum Transition {
    Start { mode: Mode },
    Stop { child: Child, active: Active },
    Reject(Failure),
}

impl Recorder {
    /// Bootstraps recorder availability.
    #[instrument(skip_all)]
    pub async fn bootstrap() -> (Handle, Snapshot) {
        let backend = Backend;
        let (state, snapshot) = match backend.probe().await {
            Ok(()) => (State::Idle, Snapshot::Idle),
            Err(error) => {
                let message = error.to_string();
                (
                    State::Unavailable {
                        message: message.clone(),
                    },
                    Snapshot::Unavailable { message },
                )
            }
        };

        let (events, _) = broadcast::channel(16);
        let (results, _) = broadcast::channel(8);
        let recorder = Arc::new(Self {
            events,
            results,
            backend,
            state: Mutex::new(state),
        });

        (recorder, snapshot)
    }

    /// Subscribes to recording state changes.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Subscribes to recording command reports.
    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    /// Toggles the recorder for the supplied mode.
    #[instrument(skip(self))]
    pub fn toggle(self: &Arc<Self>, mode: Mode) {
        let recorder = Arc::clone(self);
        tokio::spawn(async move {
            recorder.apply(Command::Toggle { mode }).await;
        });
    }

    #[instrument(skip(self))]
    async fn apply(&self, command: Command) {
        drop(self.results.send(Report::started(command)));

        let report = match self.transition(command.mode()).await {
            Transition::Start { mode } => self.start(command, mode).await,
            Transition::Stop { child, active } => Some(self.stop(command, child, active).await),
            Transition::Reject(failure) => Some(Report::failed(command, failure)),
        };

        if let Some(report) = report {
            drop(self.results.send(report));
        }
    }

    async fn transition(&self, mode: Mode) -> Transition {
        let mut state = self.state.lock().await;
        match mem::replace(&mut *state, State::Idle) {
            State::Unavailable { message } => {
                *state = State::Unavailable {
                    message: message.clone(),
                };
                Transition::Reject(Failure::Unavailable { message })
            }
            State::Idle => {
                *state = State::Selecting { mode };
                drop(self.events.send(Snapshot::Selecting { mode }));
                Transition::Start { mode }
            }
            State::Selecting { mode } => {
                *state = State::Selecting { mode };
                Transition::Reject(Failure::Busy {
                    message: "recording region selection is already in progress".to_owned(),
                })
            }
            State::Recording { child, active } => {
                *state = State::Stopping {
                    active: active.clone(),
                };
                drop(self.events.send(Snapshot::Stopping {
                    active: active.clone(),
                }));
                Transition::Stop { child, active }
            }
            State::Stopping { active } => {
                *state = State::Stopping { active };
                Transition::Reject(Failure::Busy {
                    message: "recording is already stopping".to_owned(),
                })
            }
        }
    }

    async fn start(&self, command: Command, mode: Mode) -> Option<Report> {
        let result = time::timeout(Duration::from_secs(30), self.backend.start(mode)).await;
        match result {
            Ok(Ok(started)) => {
                let active = started.active.clone();
                let mut state = self.state.lock().await;
                *state = State::Recording {
                    child: started.child,
                    active: active.clone(),
                };
                drop(self.events.send(Snapshot::Recording { active }));
                None
            }
            Ok(Err(Failure::Cancelled)) => {
                self.reset_after_failed_start().await;
                Some(Report::cancelled(command))
            }
            Ok(Err(
                failure @ Failure::MissingTool {
                    tool: "wf-recorder",
                },
            )) => {
                self.mark_unavailable(failure.to_string()).await;
                Some(Report::failed(command, failure))
            }
            Ok(Err(failure)) => {
                self.reset_after_failed_start().await;
                Some(Report::failed(command, failure))
            }
            Err(_) => {
                self.reset_after_failed_start().await;
                Some(Report::failed(command, Failure::Timeout))
            }
        }
    }

    async fn stop(&self, command: Command, child: Child, active: Active) -> Report {
        match self.backend.stop(child, &active).await {
            Ok(path) => {
                self.set_idle().await;
                Report::saved(command, path)
            }
            Err(failure) => {
                self.set_idle().await;
                Report::failed(command, failure)
            }
        }
    }

    async fn reset_after_failed_start(&self) {
        let mut state = self.state.lock().await;
        *state = State::Idle;
        drop(self.events.send(Snapshot::Idle));
    }

    async fn set_idle(&self) {
        let mut state = self.state.lock().await;
        *state = State::Idle;
        drop(self.events.send(Snapshot::Idle));
    }

    async fn mark_unavailable(&self, message: String) {
        let mut state = self.state.lock().await;
        *state = State::Unavailable {
            message: message.clone(),
        };
        drop(self.events.send(Snapshot::Unavailable { message }));
    }
}
