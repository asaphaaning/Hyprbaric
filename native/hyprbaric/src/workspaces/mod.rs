//! Live workspace indicator configuration.

mod domain;
pub mod settings;
mod signal;

use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::{Mutex, broadcast};
use tracing::instrument;

pub use domain::{Command, IndicatorStyle, Report, Snapshot, VisibleRange};

/// Workspace indicator configuration loaded from Hyprbaric TOML.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Configuration {
    indicator_style: IndicatorStyle,
    clickable: bool,
    visible_range: VisibleRange,
}

/// Shared workspace settings runtime handle.
pub type Handle = Arc<Workspaces>;

/// Runtime owner for persisted workspace indicator state.
pub struct Workspaces {
    events: broadcast::Sender<Snapshot>,
    results: broadcast::Sender<Report>,
    state: Mutex<Configuration>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            indicator_style: IndicatorStyle::default(),
            clickable: true,
            visible_range: VisibleRange::default(),
        }
    }
}

impl Configuration {
    pub const fn indicator_style(self) -> IndicatorStyle {
        self.indicator_style
    }

    pub const fn clickable(self) -> bool {
        self.clickable
    }

    pub const fn visible_range(self) -> VisibleRange {
        self.visible_range
    }

    pub(crate) fn apply(self, command: &Command) -> Self {
        match command {
            Command::SetIndicatorStyle { indicator_style } => Self {
                indicator_style: *indicator_style,
                ..self
            },
            Command::SetClickable { clickable } => Self {
                clickable: *clickable,
                ..self
            },
            Command::SetVisibleRange { visible_range } => Self {
                visible_range: *visible_range,
                ..self
            },
        }
    }

    pub const fn snapshot(self) -> Snapshot {
        Snapshot {
            indicator_style: self.indicator_style,
            clickable: self.clickable,
            visible_range: self.visible_range,
        }
    }
}

impl Workspaces {
    #[instrument(skip_all)]
    pub fn bootstrap(config: &Configuration) -> (Handle, Snapshot) {
        let (events, _) = broadcast::channel(16);
        let (results, _) = broadcast::channel(8);
        let workspaces = Arc::new(Self {
            events,
            results,
            state: Mutex::new(*config),
        });

        (workspaces, config.snapshot())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    pub fn subscribe_results(&self) -> broadcast::Receiver<Report> {
        self.results.subscribe()
    }

    #[instrument(skip(self))]
    pub async fn apply(&self, command: Command) {
        drop(self.results.send(Report::Started(command.clone())));
        let current = { *self.state.lock().await };
        let next = match settings::save(&command, current) {
            Ok(next) => next,
            Err(error) => {
                drop(self.results.send(Report::Failed {
                    command,
                    message: error.to_string(),
                }));
                return;
            }
        };

        {
            let mut state = self.state.lock().await;
            *state = next;
        }

        drop(self.results.send(Report::Saved(command)));
        drop(self.events.send(next.snapshot()));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] crate::config::Error),
}

#[cfg(test)]
mod tests {
    use super::{Command, Configuration, IndicatorStyle, VisibleRange};

    #[test]
    fn defaults_match_current_workspace_strip() {
        let config = Configuration::default();

        assert_eq!(config.indicator_style(), IndicatorStyle::Roman);
        assert!(config.clickable());
        assert_eq!(config.visible_range(), VisibleRange::Medium);
        assert_eq!(config.visible_range().count(), 7);
    }

    #[test]
    fn config_accepts_workspace_overrides() {
        let config = toml::from_str::<Configuration>(
            r#"
indicator_style = "numeric"
clickable = false
visible_range = "large"
"#,
        )
        .expect("workspace config should parse");

        assert_eq!(config.indicator_style(), IndicatorStyle::Numeric);
        assert!(!config.clickable());
        assert_eq!(config.visible_range(), VisibleRange::Large);
    }

    #[test]
    fn command_updates_only_selected_setting() {
        let config = Configuration::default().apply(&Command::SetVisibleRange {
            visible_range: VisibleRange::Small,
        });

        assert_eq!(config.indicator_style(), IndicatorStyle::Roman);
        assert!(config.clickable());
        assert_eq!(config.visible_range(), VisibleRange::Small);
    }
}
