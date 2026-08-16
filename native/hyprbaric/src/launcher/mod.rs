//! Application launcher indexing, search, icon resolution, and launch commands.
//!
//! The launcher keeps domain ranking separate from desktop-entry IO, icon-theme
//! probing, process spawning, usage persistence, and RINF signal projection. The
//! runtime module composes those pieces into the handle consumed by supervision
//! and route handlers.

mod desktop;
mod domain;
mod icons;
mod process;
mod runtime;
mod signal;
mod usage;

use std::{io, path::PathBuf};

use tokio::task;

pub use domain::{Id, Results};
pub use runtime::{Handle, Launcher};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to join an app-launcher worker task")]
    Join(#[source] task::JoinError),
    #[error("failed to watch launcher desktop-entry directories")]
    Watch(#[source] notify::Error),
    #[error("unknown app-launcher entry `{id}`")]
    UnknownEntry { id: Id },
    #[error("desktop entry `{id}` contains an invalid Exec line: `{exec}`")]
    InvalidExec { id: Id, exec: String },
    #[error("desktop entry `{id}` uses unsupported Exec placeholder `%{placeholder}`")]
    UnsupportedExecPlaceholder { id: Id, placeholder: char },
    #[error("no terminal emulator is available for terminal-based launcher entries")]
    MissingTerminalEmulator,
    #[error("the configured terminal command is invalid: `{command}`")]
    InvalidTerminalCommand { command: String },
    #[error("the configured terminal executable is unavailable: `{command}`")]
    MissingConfiguredTerminal { command: String },
    #[error("failed to spawn launcher command `{command}`")]
    Spawn {
        command: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to read launcher usage file `{path}`")]
    ReadUsageFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse launcher usage file `{path}`")]
    ParseUsageFile {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to serialize launcher usage file `{path}`")]
    SerializeUsageFile {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to write launcher usage file `{path}`")]
    WriteUsageFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
