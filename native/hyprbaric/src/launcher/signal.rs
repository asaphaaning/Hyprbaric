//! RINF projections for launcher domain values.

use super::domain::{Phase, ResultEntry, Results};

impl From<&Results> for crate::signals::AppLauncherResults {
    fn from(results: &Results) -> Self {
        Self {
            phase: (&results.phase).into(),
            query: results.query.clone(),
            entries: results.entries.iter().map(Into::into).collect(),
            message: results.phase.message().map(ToOwned::to_owned),
        }
    }
}

impl From<&Phase> for crate::signals::AppLauncherPhase {
    fn from(phase: &Phase) -> Self {
        match phase {
            Phase::Loading => Self::Loading,
            Phase::Ready => Self::Ready,
            Phase::Failed { .. } => Self::Failed,
        }
    }
}

impl From<&ResultEntry> for crate::signals::AppLauncherEntry {
    fn from(entry: &ResultEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            name: entry.name.clone(),
            subtitle: entry.subtitle.clone(),
            icon_name: entry.icon_name.clone(),
            icon_path: entry
                .icon_path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            terminal: entry.terminal,
        }
    }
}
