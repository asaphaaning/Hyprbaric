//! Brightness domain vocabulary.
//!
//! This module owns the small nouns the runtime composes: [`Percent`],
//! [`Device`], [`Snapshot`], [`Command`], and [`Report`]. System backends and
//! the actor loop construct these values, but they do not decide their public
//! shape.

use std::fmt;

/// A clamped brightness percentage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Percent(u8);

/// The system boundary that controls one brightness [`Device`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    /// Linux backlight devices exposed through `/sys/class/backlight`.
    Backlight,
    /// External monitor brightness controlled through DDC/CI.
    DdcCi,
}

/// Stable identity for a brightness [`Device`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeviceId(String);

/// One brightness-capable display target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// Stable backend-specific identity.
    pub id: DeviceId,
    /// Human-facing display label.
    pub label: String,
    /// Boundary used to read and write the device.
    pub kind: DeviceKind,
    /// Last known brightness value.
    pub value: Percent,
}

/// The UI-facing brightness state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snapshot {
    /// Discovery is running and no usable device has been selected yet.
    Discovering {
        /// User-facing progress copy.
        message: String,
    },
    /// A brightness target is ready.
    Available {
        /// Selected target brightness.
        value: Percent,
        /// Selected target label.
        device: String,
    },
    /// No target can currently be used.
    Unavailable {
        /// User-facing failure detail.
        message: String,
    },
}

/// A brightness command that reached the runtime boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Set the selected brightness target to `value`.
    SetLevel {
        /// Requested brightness value.
        value: Percent,
    },
}

/// A brightness command report published to subscribers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the brightness worker.
    Started(Command),
    /// The command could not be accepted or later failed at its backend.
    Failed {
        /// The command that failed.
        command: Command,
        /// User-facing failure detail.
        message: String,
    },
}

/// A target selected for a brightness command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    /// The currently selected brightness device.
    Selected,
}

impl Percent {
    /// Creates a percentage clamped to the inclusive `0..=100` range.
    pub const fn new(value: u8) -> Self {
        Self(if value > 100 { 100 } else { value })
    }

    /// Returns the clamped value as an integer percentage.
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Returns the value expected by brightness backends.
    pub(crate) const fn as_backend_value(self) -> u32 {
        self.0 as u32
    }
}

impl Snapshot {
    /// Creates the initial discovery snapshot.
    pub(crate) fn discovering(message: impl Into<String>) -> Self {
        Self::Discovering {
            message: message.into(),
        }
    }

    /// Creates an unavailable snapshot.
    pub(crate) fn unavailable(message: impl Into<String>) -> Self {
        Self::Unavailable {
            message: message.into(),
        }
    }
}

impl DeviceId {
    /// Creates a Linux backlight device identifier.
    pub(crate) fn backlight(name: &str) -> Self {
        Self(format!("backlight:{name}"))
    }

    /// Returns the Linux backlight name when this ID targets backlight.
    pub(crate) fn backlight_name(&self) -> Option<&str> {
        self.0.strip_prefix("backlight:")
    }

    /// Creates a DDC/CI display identifier from a bus and stable fingerprint.
    pub(crate) fn ddc(bus: &str, fingerprint: &str) -> Self {
        Self(format!("ddcci:{bus}:{}", stable_segment(fingerprint)))
    }

    /// Returns the I2C bus when this ID targets DDC/CI.
    pub(crate) fn ddc_bus(&self) -> Option<&str> {
        self.0
            .strip_prefix("ddcci:")
            .and_then(|value| value.split(':').next())
            .filter(|value| !value.is_empty())
    }
}

impl DeviceKind {
    /// Returns the backend name used in user-facing errors.
    pub(crate) const fn backend_name(self) -> &'static str {
        match self {
            Self::Backlight => "brightness",
            Self::DdcCi => "ddcutil",
        }
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn stable_segment(value: &str) -> String {
    let segment = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let segment = segment.trim_matches('-');
    if segment.is_empty() {
        "display".to_owned()
    } else {
        segment.to_owned()
    }
}
