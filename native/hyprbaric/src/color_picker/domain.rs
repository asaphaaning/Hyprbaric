//! Color picker domain vocabulary.
//!
//! The domain keeps the picked color and command lifecycle separate from the
//! `hyprpicker` process boundary.

use std::fmt;

/// A color picker command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    /// Pick one color from the screen.
    Pick,
}

/// A color picker command report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the runtime.
    Started { command: Command },
    /// A color was picked and copied by `hyprpicker`.
    Picked { command: Command, color: Color },
    /// The user cancelled picking.
    Cancelled { command: Command },
    /// Picking failed.
    Failed { command: Command, failure: Failure },
}

/// A lowercase RGB hex color.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color(String);

/// Typed color picker failure reasons.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// Picking timed out.
    Timeout,
    /// The user cancelled picking.
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
    /// Process output was not a valid lowercase RGB hex color.
    InvalidColor { value: String },
    /// Process output was not valid UTF-8.
    Utf8 { message: String },
    /// Process I/O failed.
    Io { message: String },
}

impl Report {
    /// Creates a started report.
    pub const fn started(command: Command) -> Self {
        Self::Started { command }
    }

    /// Creates a picked report.
    pub const fn picked(command: Command, color: Color) -> Self {
        Self::Picked { command, color }
    }

    /// Creates a cancelled report.
    pub const fn cancelled(command: Command) -> Self {
        Self::Cancelled { command }
    }

    /// Creates a failed report.
    pub const fn failed(command: Command, failure: Failure) -> Self {
        Self::Failed { command, failure }
    }
}

impl Color {
    /// Parses a `#rrggbb` color and normalizes it to lowercase.
    pub fn parse(value: &str) -> Result<Self, Failure> {
        let trimmed = value.trim();
        let bytes = trimmed.as_bytes();
        let valid = bytes.len() == 7
            && bytes[0] == b'#'
            && bytes[1..].iter().all(|byte| byte.is_ascii_hexdigit());

        if valid {
            Ok(Self(trimmed.to_ascii_lowercase()))
        } else {
            Err(Failure::InvalidColor {
                value: trimmed.to_owned(),
            })
        }
    }

    /// Returns the hex color string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => f.write_str("color pick timed out"),
            Self::Cancelled => f.write_str("color pick was cancelled"),
            Self::MissingTool { tool } => write!(f, "`{tool}` is unavailable"),
            Self::Process { tool, detail } => write!(f, "`{tool}` failed: {detail}"),
            Self::InvalidColor { value } => {
                write!(f, "`hyprpicker` returned invalid color `{value}`")
            }
            Self::Utf8 { message } => write!(f, "command output was not valid UTF-8: {message}"),
            Self::Io { message } => f.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Failure};

    #[test]
    fn color_accepts_rgb_hex() {
        let color = Color::parse("#38BDF8").expect("hex color should parse");

        assert_eq!(color.as_str(), "#38bdf8");
    }

    #[test]
    fn color_rejects_invalid_values() {
        assert!(matches!(
            Color::parse("38bdf8"),
            Err(Failure::InvalidColor { .. })
        ));
        assert!(matches!(
            Color::parse("#38bdf80"),
            Err(Failure::InvalidColor { .. })
        ));
        assert!(matches!(
            Color::parse("#38bdzz"),
            Err(Failure::InvalidColor { .. })
        ));
    }
}
