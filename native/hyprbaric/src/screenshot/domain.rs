//! Screenshot domain vocabulary.
//!
//! This module keeps successful, cancelled, and failed states as separate
//! variants so saved captures always carry a path and failures always carry a
//! typed reason.

use std::{
    fmt,
    path::{Path, PathBuf},
};

/// A screenshot capture command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    /// Capture a screenshot using the supplied mode.
    Capture(Mode),
}

/// The screenshot target mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Select a region interactively.
    Region,
    /// Capture the active Hyprland window.
    Window,
    /// Capture the full desktop.
    FullScreen,
}

/// A screenshot command report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the runtime.
    Started { command: Command },
    /// A capture was saved.
    Saved { command: Command, saved: Saved },
    /// The user cancelled selection.
    Cancelled { command: Command },
    /// Capture failed.
    Failed { command: Command, failure: Failure },
}

/// A saved screenshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Saved {
    path: PathBuf,
    clipboard: Clipboard,
}

/// Clipboard copy state for a saved screenshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Clipboard {
    /// Screenshot bytes were copied, or `wl-copy` outlived our wait window.
    Copied,
    /// The image was saved, but clipboard copy failed.
    Failed { message: String },
}

/// Typed screenshot failure reasons.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// Capture timed out.
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
    /// Hyprland did not report an active window.
    NoActiveWindow,
    /// JSON from a system boundary could not be parsed.
    Json { message: String },
    /// Process output was not valid UTF-8.
    Utf8 { message: String },
    /// Filesystem or process I/O failed.
    Io { message: String },
}

/// A rectangular capture area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    origin: Point,
    size: Extent,
}

/// Capture-area origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

/// Capture-area size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Extent {
    width: u32,
    height: u32,
}

impl Command {
    /// Returns the mode this command owns.
    pub const fn mode(self) -> Mode {
        match self {
            Self::Capture(mode) => mode,
        }
    }
}

impl Report {
    /// Creates a started report.
    pub const fn started(command: Command) -> Self {
        Self::Started { command }
    }

    /// Creates a saved report.
    pub fn saved(command: Command, saved: Saved) -> Self {
        Self::Saved { command, saved }
    }

    /// Creates a cancelled report.
    pub const fn cancelled(command: Command) -> Self {
        Self::Cancelled { command }
    }

    /// Creates a failed report.
    pub fn failed(command: Command, failure: Failure) -> Self {
        Self::Failed { command, failure }
    }
}

impl Saved {
    /// Creates a saved capture.
    pub(super) fn new(path: PathBuf, clipboard: Clipboard) -> Self {
        Self { path, clipboard }
    }

    /// Returns the saved image path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns clipboard copy state.
    pub const fn clipboard(&self) -> &Clipboard {
        &self.clipboard
    }
}

impl Clipboard {
    /// Creates failed clipboard state from a displayable error.
    pub(super) fn failed(error: impl fmt::Display) -> Self {
        Self::Failed {
            message: error.to_string(),
        }
    }
}

impl Area {
    /// Creates a non-empty capture area.
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

    /// Creates an area from Hyprland active-window coordinates.
    pub(super) fn from_window_parts(at: [i32; 2], size: [u32; 2]) -> Result<Self, Failure> {
        Self::new(Point::new(at[0], at[1]), Extent::new(size[0], size[1]))
    }

    /// Returns the geometry format expected by grim.
    pub(super) fn grim_geometry(self) -> String {
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
            Self::Timeout => f.write_str("screenshot selection timed out"),
            Self::Cancelled => f.write_str("region selection was cancelled"),
            Self::MissingTool { tool } => write!(f, "`{tool}` is unavailable"),
            Self::Process { tool, detail } => write!(f, "`{tool}` failed: {detail}"),
            Self::EmptyArea => f.write_str("selected screenshot area is empty"),
            Self::InvalidGeometry { value } => {
                write!(f, "invalid screenshot geometry `{value}`")
            }
            Self::NoActiveWindow => f.write_str("no active window is available to capture"),
            Self::Json { message } => {
                write!(f, "failed to parse Hyprland active window JSON: {message}")
            }
            Self::Utf8 { message } => write!(f, "command output was not valid UTF-8: {message}"),
            Self::Io { message } => f.write_str(message),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Region => "region",
            Self::Window => "window",
            Self::FullScreen => "full screen",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Area, Extent, Failure, Point};

    #[test]
    fn parses_slurp_geometry() {
        let area = Area::from_slurp("123,45 640x360\n").unwrap();

        assert_eq!(
            area,
            Area::new(Point::new(123, 45), Extent::new(640, 360)).unwrap()
        );
        assert_eq!(area.grim_geometry(), "123,45 640x360");
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

    #[test]
    fn parses_active_window_geometry() {
        let area = Area::from_window_parts([20, 30], [800, 600]).unwrap();

        assert_eq!(area.grim_geometry(), "20,30 800x600");
    }
}
