//! Runtime composition for the app launcher.

use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};

use freedesktop_desktop_entry::get_languages_from_env;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
    sync::{RwLock, broadcast, mpsc},
    task,
    time::{Instant, sleep},
};
use tracing::instrument;

use super::{
    Error,
    desktop::{self, DesktopDirectory, WatchTarget},
    domain::{Cache, Id, Phase, Results, State},
    process, usage,
};

pub type Handle = Arc<Launcher>;

const WATCH_DEBOUNCE: Duration = Duration::from_millis(160);

#[derive(Clone)]
pub struct Launcher {
    state: Arc<RwLock<State>>,
    events: broadcast::Sender<Results>,
    desktop_dirs: Arc<[DesktopDirectory]>,
    watch_targets: Arc<[WatchTarget]>,
    locales: Arc<[String]>,
    usage_path: PathBuf,
}

impl Launcher {
    #[instrument]
    pub async fn bootstrap() -> (Handle, Results) {
        let desktop_dirs = desktop::application_directories();
        let watch_targets = desktop::watch_targets(&desktop_dirs);
        let locales = get_languages_from_env();
        let usage_path = usage::usage_path();
        let usage = match usage::read_usage_file(usage_path.clone()).await {
            Ok(usage) => usage,
            Err(error) => {
                tracing::warn!("Failed to load launcher usage file: {error}");
                HashMap::new()
            }
        };

        let (events, _) = broadcast::channel(32);
        let launcher = Arc::new(Self {
            state: Arc::new(RwLock::new(State {
                phase: Phase::Loading,
                query: String::new(),
                cache: Cache {
                    entries: Vec::new(),
                    usage,
                    icons: None,
                },
            })),
            events,
            desktop_dirs: Arc::from(desktop_dirs),
            watch_targets: Arc::from(watch_targets),
            locales: Arc::from(locales),
            usage_path,
        });

        let initial = Results {
            phase: Phase::Loading,
            query: String::new(),
            entries: Vec::new(),
        };
        launcher.spawn_rebuild();
        launcher.spawn_watcher();
        (launcher, initial)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Results> {
        self.events.subscribe()
    }

    #[instrument(skip(self), fields(query = %query))]
    pub async fn update_query(&self, query: String) -> Results {
        let results = {
            let mut state = self.state.write().await;
            state.query = query;
            state.results()
        };
        let _ = self.events.send(results.clone());
        self.spawn_icon_refresh();
        results
    }

    #[instrument(skip(self))]
    pub async fn rebuild(&self) -> Result<Results, Error> {
        let desktop_dirs = self.desktop_dirs.to_vec();
        let locales = self.locales.to_vec();
        let entries = task::spawn_blocking(move || desktop::build_index(desktop_dirs, locales))
            .await
            .map_err(Error::Join)?;

        let results = {
            let mut state = self.state.write().await;
            state.phase = Phase::Ready;
            state.cache.entries = entries;
            state.cache.icons = None;
            state.results()
        };

        let _ = self.events.send(results.clone());
        self.spawn_icon_refresh();
        Ok(results)
    }

    #[instrument(skip(self), fields(entry_id = %entry_id))]
    pub async fn launch(&self, entry_id: Id) -> Result<(), Error> {
        let entry = {
            let state = self.state.read().await;
            state
                .cache
                .entry(&entry_id)
                .cloned()
                .ok_or_else(|| Error::UnknownEntry {
                    id: entry_id.clone(),
                })?
        };

        process::start_entry(&entry)?;

        let (results, usage) = {
            let mut state = self.state.write().await;
            state.cache.record_launch(&entry_id);
            (state.results(), state.cache.usage.clone())
        };
        let _ = self.events.send(results);

        if let Err(error) = usage::write_usage_file(self.usage_path.clone(), usage).await {
            tracing::warn!("Failed to persist launcher usage file: {error}");
        }

        Ok(())
    }

    fn spawn_watcher(self: &Arc<Self>) {
        if self.watch_targets.is_empty() {
            return;
        }

        let launcher = Arc::clone(self);
        tokio::spawn(async move {
            let (tx, mut rx) = mpsc::unbounded_channel::<notify::Result<Event>>();
            let watcher = match create_watcher(tx, &launcher.watch_targets) {
                Ok(watcher) => watcher,
                Err(error) => {
                    tracing::warn!("Failed to start app-launcher watcher: {error}");
                    return;
                }
            };

            while rx.recv().await.is_some() {
                let deadline = Instant::now() + WATCH_DEBOUNCE;
                let timer = sleep_until(deadline);
                tokio::pin!(timer);

                loop {
                    tokio::select! {
                        _ = &mut timer => break,
                        event = rx.recv() => {
                            let Some(event) = event else {
                                return;
                            };
                            if event.is_ok() {
                                timer.as_mut().reset(Instant::now() + WATCH_DEBOUNCE);
                                continue;
                            }
                            if let Err(error) = event {
                                tracing::warn!("App-launcher watcher error: {error}");
                            }
                            timer.as_mut().reset(Instant::now() + WATCH_DEBOUNCE);
                        }
                    }
                }

                if let Err(error) = launcher.rebuild().await {
                    tracing::warn!("Failed to rebuild app-launcher index: {error}");
                }
            }

            drop(watcher);
        });
    }

    fn spawn_rebuild(self: &Arc<Self>) {
        let launcher = Arc::clone(self);
        tokio::spawn(async move {
            if let Err(error) = launcher.rebuild().await {
                launcher.fail(error.to_string()).await;
            }
        });
    }

    fn spawn_icon_refresh(&self) {
        let state = Arc::clone(&self.state);
        let events = self.events.clone();
        tokio::spawn(async move {
            let results = {
                let mut state = state.write().await;
                let query = state.query.clone();
                if !state.cache.resolve_visible_icons(&query) {
                    return;
                }
                state.results()
            };

            let _ = events.send(results);
        });
    }

    async fn fail(&self, message: String) -> Results {
        let results = {
            let mut state = self.state.write().await;
            state.phase = Phase::Failed { message };
            state.results()
        };
        let _ = self.events.send(results.clone());
        results
    }
}

fn create_watcher(
    tx: mpsc::UnboundedSender<notify::Result<Event>>,
    targets: &[WatchTarget],
) -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(
        move |event| {
            let _ = tx.send(event);
        },
        Config::default(),
    )
    .map_err(Error::Watch)?;

    for target in targets {
        watcher
            .watch(
                &target.path,
                if target.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                },
            )
            .map_err(Error::Watch)?;
    }

    Ok(watcher)
}

fn sleep_until(deadline: Instant) -> tokio::time::Sleep {
    sleep(deadline.saturating_duration_since(Instant::now()))
}
