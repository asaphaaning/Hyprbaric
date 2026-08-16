//! Runtime shortcut installation and activation delivery.
//!
//! [`Registry`] owns the portal session, remembers desired [`Spec`] values, and
//! retries transient unfinished installs until both the portal and Hyprland
//! projections agree with configuration.
//!
//! ```text
//! shortcut install ──> Registry ──> portal::Session::bind
//!                         │
//! supervised run ─────────┴──────> portal::Session::next ──> Event
//! ```
//!
//! The portal session owns its activation stream. The registry shares that
//! session between installation and its supervised runner without forwarding
//! the stream through a second channel.

use std::{
    collections::{HashMap, HashSet},
    fmt, future,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use tokio::{
    sync::{Mutex, Notify, broadcast},
    task,
    time::{interval, timeout},
};
use tracing::instrument;

use super::{Error, Event, Shortcut, Spec, hyprland, identity, portal};

const HYPRLAND_BIND_SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(2);

/// A shared handle to the process-wide shortcut [`Registry`].
pub type Handle = Arc<Registry>;

/// The installed shortcut registry.
///
/// The registry is idempotent at the [`Spec`] boundary. Registering the same
/// spec again is a no-op, while a changed spec is reconciled through the portal
/// and Hyprland before its activation events are forwarded.
#[derive(Clone)]
pub struct Registry {
    state: Arc<Mutex<State>>,
    events: broadcast::Sender<Event>,
    runtime_ready: Arc<Notify>,
    runner_started: Arc<AtomicBool>,
}

struct State {
    /// The portal runtime, created lazily when the first spec needs it.
    runtime: Option<Arc<portal::Session>>,
    /// Desired and installed state keyed by stable shortcut meaning.
    tracked: HashMap<Shortcut, Registration>,
}

/// The installation lifecycle for one tracked [`Shortcut`].
///
/// Portal and compositor progress are recorded separately so a failed
/// reconciliation can resume the unfinished side without rebinding work that
/// already succeeded.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Registration {
    /// A desired spec waiting to reconcile one or both boundaries.
    Pending { desired: Spec, progress: Progress },
    /// A desired spec currently being reconciled by a registry task.
    Installing { desired: Spec, progress: Progress },
    /// A desired spec that cannot be retried safely without user action.
    Blocked {
        desired: Spec,
        progress: Progress,
        reason: BlockedReason,
    },
    /// A spec installed in both portal and compositor state.
    Ready { installed: Spec },
}

impl Registration {
    /// Starts tracking a spec before any boundary has installed it.
    fn pending(desired: Spec) -> Self {
        Self::Pending {
            desired,
            progress: Progress::pending(),
        }
    }

    /// Returns whether this lifecycle already represents the requested spec.
    fn tracks(&self, spec: &Spec) -> bool {
        match self {
            Self::Pending { desired, .. } | Self::Installing { desired, .. } => desired == spec,
            Self::Blocked { .. } => false,
            Self::Ready { installed } => installed == spec,
        }
    }

    /// Returns unfinished boundary work for retryable registrations.
    const fn pending_work(&self) -> Option<Progress> {
        match self {
            Self::Pending { progress, .. } => Some(*progress),
            Self::Installing { .. } | Self::Blocked { .. } | Self::Ready { .. } => None,
        }
    }
}

/// A terminal reason that leaves a shortcut waiting for a new configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockedReason {
    /// The configured chord is shared with a non-Hyprbaric compositor bind.
    SharedForeignChord,
}

impl BlockedReason {
    /// Classifies terminal shortcut reconciliation errors.
    const fn from_error(error: &Error) -> Option<Self> {
        if error.blocks_shortcut_retry() {
            Some(Self::SharedForeignChord)
        } else {
            None
        }
    }
}

impl fmt::Display for BlockedReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SharedForeignChord => formatter.write_str("shared foreign chord"),
        }
    }
}

/// Installation progress across the two shortcut boundaries.
///
/// The portal registration and Hyprland bind can fail independently. Keeping
/// the boundary phases apart lets a retry complete partial work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Progress {
    /// Progress for the desktop shortcuts portal.
    portal: InstallPhase,
    /// Progress for the Hyprland `global` bind.
    compositor: InstallPhase,
}

/// A retryable shortcut and its unfinished boundary work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingWork {
    shortcut: Shortcut,
    work: Progress,
}

impl Progress {
    /// Starts with both shortcut boundaries waiting for installation.
    const fn pending() -> Self {
        Self {
            portal: InstallPhase::Pending,
            compositor: InstallPhase::Pending,
        }
    }

    /// Returns whether both boundaries have accepted the spec.
    const fn is_ready(self) -> bool {
        matches!(self.portal, InstallPhase::Installed)
            && matches!(self.compositor, InstallPhase::Installed)
    }

    /// Returns whether this shortcut still needs Hyprland bind work.
    const fn needs_compositor(self) -> bool {
        matches!(self.compositor, InstallPhase::Pending)
    }
}

/// The install state for one shortcut boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InstallPhase {
    /// The boundary still needs to install or reconcile the spec.
    Pending,
    /// The boundary accepted the spec.
    Installed,
}

impl Registry {
    /// Creates an empty registry.
    ///
    /// Retry scheduling and activation delivery begin when [`Registry::run`]
    /// enters the application's supervised lifecycle.
    pub fn new() -> Self {
        let (events, _) = broadcast::channel(32);

        Self {
            state: Arc::new(Mutex::new(State {
                runtime: None,
                tracked: HashMap::new(),
            })),
            events,
            runtime_ready: Arc::new(Notify::new()),
            runner_started: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Runs retry scheduling and portal activation delivery.
    ///
    /// The registry runner is a process lifecycle and may only be started once.
    #[instrument(name = "hyprbaric::shortcuts::registry::run", skip(self), err)]
    pub async fn run(&self) -> Result<(), Error> {
        if self.runner_started.swap(true, Ordering::AcqRel) {
            return Err(Error::RegistryAlreadyRunning);
        }

        let mut ticker = interval(Duration::from_secs(5));
        ticker.tick().await;

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(error) = self.reconcile_pending().await {
                        tracing::warn!("Global shortcuts reconcile failed: {error}");
                    }
                }
                message = self.next_activation() => {
                    match message {
                        Ok(message) => match portal::parse_activation(&message) {
                            Ok(Some(shortcut)) => {
                                drop(self.events.send(Event { shortcut }));
                            }
                            Ok(None) => {}
                            Err(error) => {
                                tracing::warn!(
                                    "Ignoring malformed shortcut activation: {error}"
                                );
                            }
                        },
                        Err(error) => {
                            tracing::warn!("Shortcut activation listener failed: {error}");
                        }
                    }
                }
            }
        }
    }

    /// Subscribes to activated shortcut [`Event`] values.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.events.subscribe()
    }

    /// Installs each enabled shortcut spec from configuration.
    #[instrument(skip_all, err)]
    pub async fn install(&self, specs: impl IntoIterator<Item = Spec>) -> Result<(), Error> {
        let pending = self.track_specs(specs).await;
        if pending.is_empty() {
            return Ok(());
        }

        let snapshot = match read_bind_snapshot().await {
            Ok(snapshot) => snapshot,
            Err(error) => {
                tracing::warn!("Shortcut install deferred: failed to read Hyprland binds: {error}");
                return Err(error);
            }
        };
        let mut first_error = None;

        for shortcut in pending {
            if let Err(error) = self.reconcile(shortcut, Some(&snapshot)).await {
                if error.blocks_shortcut_retry() {
                    tracing::debug!(
                        shortcut_id = %shortcut,
                        "Shortcut left foreign-owned"
                    );
                } else if first_error.is_none() {
                    tracing::warn!(shortcut_id = %shortcut, "Shortcut install failed: {error}");
                    first_error = Some(error);
                } else {
                    tracing::warn!(shortcut_id = %shortcut, "Shortcut install failed: {error}");
                }
            }
            task::yield_now().await;
        }

        if let Some(error) = first_error {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Synchronizes the registry with a complete enabled shortcut set.
    ///
    /// Shortcuts absent from `specs` are treated as disabled and have their
    /// Hyprland forwarding binds removed when it is safe to do so.
    #[instrument(skip_all, err)]
    pub async fn sync(&self, specs: impl IntoIterator<Item = Spec>) -> Result<(), Error> {
        let specs = specs.into_iter().collect::<Vec<_>>();
        let desired = specs
            .iter()
            .map(Spec::shortcut)
            .collect::<HashSet<Shortcut>>();
        let stale = {
            let guard = self.state.lock().await;
            guard
                .tracked
                .keys()
                .copied()
                .filter(|shortcut| !desired.contains(shortcut))
                .collect::<Vec<_>>()
        };

        for shortcut in stale {
            self.unregister(shortcut).await?;
        }
        self.install(specs).await
    }

    /// Stops tracking and forwarding a shortcut.
    #[instrument(skip(self), fields(shortcut_id = %shortcut), err)]
    async fn unregister(&self, shortcut: Shortcut) -> Result<(), Error> {
        let was_tracked = {
            let mut guard = self.state.lock().await;
            guard.tracked.remove(&shortcut).is_some()
        };

        if was_tracked {
            hyprland::remove(shortcut).await?;
        }

        Ok(())
    }

    /// Stores desired specs and returns shortcuts that need reconciliation.
    async fn track_specs(&self, specs: impl IntoIterator<Item = Spec>) -> Vec<Shortcut> {
        let mut pending = Vec::new();
        let mut guard = self.state.lock().await;

        for spec in specs {
            let shortcut = spec.shortcut();
            match guard.tracked.get(&shortcut) {
                Some(registration) if registration.tracks(&spec) => {}
                Some(_) | None => {
                    guard.tracked.insert(shortcut, Registration::pending(spec));
                    pending.push(shortcut);
                }
            }
        }

        pending
    }

    /// Retries every [`Registration::Pending`] spec.
    ///
    /// One failed shortcut is logged and left pending without stopping retries
    /// for the remaining shortcut set.
    #[instrument(skip(self), err)]
    async fn reconcile_pending(&self) -> Result<(), Error> {
        let pending = {
            let guard = self.state.lock().await;
            guard
                .tracked
                .iter()
                .filter_map(|(shortcut, registration)| {
                    registration.pending_work().map(|work| PendingWork {
                        shortcut: *shortcut,
                        work,
                    })
                })
                .collect::<Vec<_>>()
        };

        let snapshot = if pending
            .iter()
            .any(|pending| pending.work.needs_compositor())
        {
            match read_bind_snapshot().await {
                Ok(snapshot) => Some(snapshot),
                Err(error) => {
                    tracing::warn!(
                        "Pending shortcut reconcile deferred: failed to read Hyprland binds: {error}"
                    );
                    return Ok(());
                }
            }
        } else {
            None
        };

        for pending in pending {
            if let Err(error) = self.reconcile(pending.shortcut, snapshot.as_ref()).await {
                tracing::warn!(
                    "Pending shortcut reconcile failed for '{}': {error}",
                    pending.shortcut
                );
            }
            task::yield_now().await;
        }

        Ok(())
    }

    /// Reconciles one tracked shortcut through the portal and Hyprland.
    ///
    /// A shortcut already being installed or already ready is ignored. A pending
    /// shortcut resumes from its [`Progress`] so portal work is not repeated
    /// after a compositor failure.
    #[instrument(skip(self, snapshot), fields(shortcut_id = %shortcut), err)]
    async fn reconcile(
        &self,
        shortcut: Shortcut,
        snapshot: Option<&hyprland::BindSnapshot>,
    ) -> Result<(), Error> {
        let Some((spec, mut progress)) = self.begin_reconcile(shortcut).await? else {
            return Ok(());
        };

        if matches!(progress.compositor, InstallPhase::Pending) {
            let result = match snapshot {
                Some(snapshot) => hyprland::reconcile_from(&spec, snapshot).await,
                None => Err(Error::CommandTimeout {
                    command: "hyprctl binds -j".to_owned(),
                }),
            };
            if let Err(error) = result {
                if let Some(reason) = BlockedReason::from_error(&error) {
                    self.block_reconcile(shortcut, spec, progress, reason)
                        .await?;
                } else {
                    self.finish_reconcile(shortcut, spec, progress).await?;
                }
                return Err(error);
            }
            progress.compositor = InstallPhase::Installed;
        }

        let runtime = self.ensure_runtime().await?;

        match runtime.bind(&spec).await {
            Ok(()) => {
                progress.portal = InstallPhase::Installed;
                self.finish_reconcile(shortcut, spec, progress).await?;
                Ok(())
            }
            Err(error) => {
                self.finish_reconcile(shortcut, spec, progress).await?;
                Err(error)
            }
        }
    }

    /// Claims the pending work for a shortcut before crossing async boundaries.
    ///
    /// The returned [`Progress`] captures which boundary work has already
    /// completed, and the registry moves the record into
    /// [`Registration::Installing`] so concurrent reconciliation calls do not
    /// duplicate it.
    async fn begin_reconcile(&self, shortcut: Shortcut) -> Result<Option<(Spec, Progress)>, Error> {
        let mut guard = self.state.lock().await;
        let registration = guard
            .tracked
            .get_mut(&shortcut)
            .ok_or(Error::UntrackedShortcut { shortcut })?;
        let Registration::Pending { desired, progress } = registration.clone() else {
            return Ok(None);
        };
        *registration = Registration::Installing {
            desired: desired.clone(),
            progress,
        };
        Ok(Some((desired, progress)))
    }

    /// Returns the live portal runtime, creating it on first install.
    async fn ensure_runtime(&self) -> Result<Arc<portal::Session>, Error> {
        let mut guard = self.state.lock().await;
        if let Some(runtime) = guard.runtime.as_ref().map(Arc::clone) {
            return Ok(runtime);
        }

        let identity = identity::Identity::prepare().await?;
        let runtime = portal::Session::connect(&identity).await?;
        guard.runtime = Some(Arc::clone(&runtime));
        self.runtime_ready.notify_one();

        Ok(runtime)
    }

    /// Waits for the lazily created portal session and receives one activation.
    async fn next_activation(&self) -> Result<zbus::Message, zbus::Error> {
        let runtime = self.wait_for_runtime().await;

        match runtime.next().await {
            Some(message) => message,
            None => future::pending().await,
        }
    }

    /// Waits until shortcut installation has established the portal session.
    async fn wait_for_runtime(&self) -> Arc<portal::Session> {
        loop {
            if let Some(runtime) = self.state.lock().await.runtime.as_ref().map(Arc::clone) {
                return runtime;
            }

            self.runtime_ready.notified().await;
        }
    }

    /// Stores the result of a reconciliation attempt.
    ///
    /// If another call replaced the desired spec while the attempt was in
    /// flight, that newer desired state wins and the stale result is discarded.
    async fn finish_reconcile(
        &self,
        shortcut: Shortcut,
        desired: Spec,
        progress: Progress,
    ) -> Result<(), Error> {
        let mut guard = self.state.lock().await;
        let registration = guard
            .tracked
            .get_mut(&shortcut)
            .ok_or(Error::UntrackedShortcut { shortcut })?;

        if !matches!(
            registration,
            Registration::Installing {
                desired: current,
                ..
            } if current == &desired
        ) {
            return Ok(());
        }

        *registration = if progress.is_ready() {
            Registration::Ready { installed: desired }
        } else {
            Registration::Pending { desired, progress }
        };

        Ok(())
    }

    /// Stores a terminal reconciliation failure.
    ///
    /// A blocked registration is intentionally skipped by the retry ticker. A
    /// future call to [`Registry::register`] with the same or a different
    /// [`Spec`] replaces the blocked state with a fresh pending attempt, so user
    /// saves still get an immediate result.
    async fn block_reconcile(
        &self,
        shortcut: Shortcut,
        desired: Spec,
        progress: Progress,
        reason: BlockedReason,
    ) -> Result<(), Error> {
        let mut guard = self.state.lock().await;
        let registration = guard
            .tracked
            .get_mut(&shortcut)
            .ok_or(Error::UntrackedShortcut { shortcut })?;

        if !matches!(
            registration,
            Registration::Installing {
                desired: current,
                ..
            } if current == &desired
        ) {
            return Ok(());
        }

        *registration = Registration::Blocked {
            desired,
            progress,
            reason,
        };

        tracing::debug!(
            shortcut_id = %shortcut,
            %reason,
            "Shortcut reconciliation blocked until configuration changes"
        );

        Ok(())
    }
}

async fn read_bind_snapshot() -> Result<hyprland::BindSnapshot, Error> {
    match timeout(
        HYPRLAND_BIND_SNAPSHOT_TIMEOUT,
        hyprland::BindSnapshot::read(),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => Err(Error::CommandTimeout {
            command: "hyprctl binds -j".to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockedReason, InstallPhase, Progress, Registration};
    use crate::shortcuts::{Configuration, Shortcut};

    fn spec(shortcut: Shortcut) -> super::Spec {
        Configuration::default()
            .specs()
            .find(|spec| spec.shortcut() == shortcut)
            .expect("default shortcuts should be bound")
    }

    #[test]
    fn blocked_registration_does_not_retry_in_background() {
        let registration = Registration::Blocked {
            desired: spec(Shortcut::Controls),
            progress: Progress {
                portal: InstallPhase::Pending,
                compositor: InstallPhase::Pending,
            },
            reason: BlockedReason::SharedForeignChord,
        };

        assert!(registration.pending_work().is_none());
    }

    #[test]
    fn blocked_registration_can_be_replaced_by_user_save() {
        let controls = spec(Shortcut::Controls);
        let registration = Registration::Blocked {
            desired: controls.clone(),
            progress: Progress {
                portal: InstallPhase::Pending,
                compositor: InstallPhase::Pending,
            },
            reason: BlockedReason::SharedForeignChord,
        };

        assert!(!registration.tracks(&controls));
    }
}
