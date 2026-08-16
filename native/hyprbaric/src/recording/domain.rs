//! Screen recording domain vocabulary.

use std::{
    fmt,
    path::{Path, PathBuf},
};

/// A screen recording command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    /// Toggle a recording for the supplied mode.
    Toggle { mode: Mode },
}

/// The recording target mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Select a region interactively.
    Region,
}

/// UI-facing recorder state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snapshot {
    /// `wf-recorder` is unavailable.
    Unavailable { message: String },
    /// Recorder is ready and idle.
    Idle,
    /// Hyprbaric is waiting for region selection.
    Selecting { mode: Mode },
    /// Recorder is actively recording.
    Recording { active: Active },
    /// Recorder is finalizing a recording.
    Stopping { active: Active },
}

/// A recording command report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the runtime.
    Started(Command),
    /// A recording was saved.
    Saved { command: Command, path: PathBuf },
    /// The user cancelled selection.
    Cancelled(Command),
    /// Recording failed.
    Failed { command: Command, failure: Failure },
}

/// A live or finalizing recording.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Active {
    mode: Mode,
    path: PathBuf,
    started_at_ms: u64,
}

/// Typed recording failure reasons.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// Region selection or recorder startup timed out.
    Timeout,
    /// The user cancelled region selection.
    Cancelled,
    /// A required process is unavailable.
    MissingTool { tool: &'static str },
    /// A process returned a non-zero status.
    Process {
        /// Process name.
        tool: &'static str,
        /// User-facing failure detail.
        detail: String,
    },
    /// Selected geometry was empty.
    EmptyArea,
    /// Geometry text could not be parsed.
    InvalidGeometry { value: String },
    /// Recorder is unavailable.
    Unavailable { message: String },
    /// Recorder is busy with another transition.
    Busy { message: String },
    /// Process output was not valid UTF-8.
    Utf8 { message: String },
    /// Compositor JSON could not be parsed.
    Json { message: String },
    /// Filesystem or process I/O failed.
    Io { message: String },
}

/// A rectangular recording area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    origin: Point,
    size: Extent,
}

/// Recording-area origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

/// Recording-area size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Extent {
    width: u32,
    height: u32,
}

impl Command {
    /// Returns the mode this command owns.
    pub const fn mode(self) -> Mode {
        match self {
            Self::Toggle { mode } => mode,
        }
    }
}

impl Report {
    /// Creates a started report.
    pub const fn started(command: Command) -> Self {
        Self::Started(command)
    }

    /// Creates a saved report.
    pub fn saved(command: Command, path: PathBuf) -> Self {
        Self::Saved { command, path }
    }

    /// Creates a cancelled report.
    pub const fn cancelled(command: Command) -> Self {
        Self::Cancelled(command)
    }

    /// Creates a failed report.
    pub const fn failed(command: Command, failure: Failure) -> Self {
        Self::Failed { command, failure }
    }
}

impl Active {
    /// Creates a live recording projection.
    pub(super) const fn new(mode: Mode, path: PathBuf, started_at_ms: u64) -> Self {
        Self {
            mode,
            path,
            started_at_ms,
        }
    }

    /// Returns the recording mode.
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns the target video path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the recording start time as Unix milliseconds.
    pub const fn started_at_ms(&self) -> u64 {
        self.started_at_ms
    }
}

impl Area {
    /// Creates a non-empty recording area.
    pub fn new(origin: Point, size: Extent) -> Result<Self, Failure> {
        if size.width == 0 || size.height == 0 {
            return Err(Failure::EmptyArea);
        }
        Ok(Self { origin, size })
    }

    /// Parses slurp geometry.
    pub fn from_slurp(value: &str) -> Result<Self, Failure> {
        let value = value.trim();
        let (origin, size) = value
            .split_once(' ')
            .ok_or_else(|| Failure::invalid_geometry(value))?;
        let (x, y) = origin
            .split_once(',')
            .ok_or_else(|| Failure::invalid_geometry(value))?;
        let (width, height) = size
            .split_once('x')
            .ok_or_else(|| Failure::invalid_geometry(value))?;

        Self::new(
            Point::new(
                x.parse().map_err(|_| Failure::invalid_geometry(value))?,
                y.parse().map_err(|_| Failure::invalid_geometry(value))?,
            ),
            Extent::new(
                width
                    .parse()
                    .map_err(|_| Failure::invalid_geometry(value))?,
                height
                    .parse()
                    .map_err(|_| Failure::invalid_geometry(value))?,
            ),
        )
    }

    /// Returns the geometry format expected by `wf-recorder`.
    pub(super) fn wf_geometry(self) -> String {
        format!(
            "{},{} {}x{}",
            self.origin.x, self.origin.y, self.size.width, self.size.height
        )
    }
}

impl Point {
    /// Creates a point.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Extent {
    /// Creates an extent.
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Failure {
    fn invalid_geometry(value: &str) -> Self {
        Self::InvalidGeometry {
            value: value.to_owned(),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => f.write_str("recording timed out"),
            Self::Cancelled => f.write_str("recording region selection was cancelled"),
            Self::MissingTool { tool } => write!(f, "`{tool}` is unavailable"),
            Self::Process { tool, detail } => write!(f, "`{tool}` failed: {detail}"),
            Self::EmptyArea => f.write_str("selected recording area is empty"),
            Self::InvalidGeometry { value } => {
                write!(f, "invalid recording geometry `{value}`")
            }
            Self::Unavailable { message } => f.write_str(message),
            Self::Busy { message } => f.write_str(message),
            Self::Utf8 { message } => write!(f, "command output was not valid UTF-8: {message}"),
            Self::Json { message } => write!(f, "compositor output was invalid: {message}"),
            Self::Io { message } => f.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Area, Failure};

    #[test]
    fn parses_slurp_geometry() {
        let area = Area::from_slurp("123,45 640x360\n").unwrap();

        assert_eq!(area.wf_geometry(), "123,45 640x360");
    }

    #[test]
    fn rejects_empty_slurp_geometry() {
        assert_eq!(Area::from_slurp("10,20 0x100"), Err(Failure::EmptyArea));
        assert_eq!(Area::from_slurp("10,20 100x0"), Err(Failure::EmptyArea));
    }

    #[test]
    fn rejects_invalid_slurp_geometry() {
        assert!(matches!(
            Area::from_slurp("not geometry"),
            Err(Failure::InvalidGeometry { .. })
        ));
    }
}
