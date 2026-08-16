//! Power and battery state published to the bar.
//!
//! The runtime keeps D-Bus strings and UPower integers at the boundary. This
//! module owns Hyprbaric's small power vocabulary: [`Profile`], [`Battery`],
//! [`Snapshot`], [`Command`], and [`Report`].

use std::time::Duration;

/// A clamped battery percentage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Percent(u8);

/// A power profile exposed by power-profiles-daemon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Profile {
    /// Lower power draw.
    Saver,
    /// The default balanced system policy.
    Balanced,
    /// Performance-biased policy.
    Performance,
}

/// UPower battery state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BatteryState {
    /// UPower did not report a known state.
    #[default]
    Unknown,
    /// Battery is charging.
    Charging,
    /// Battery is discharging.
    Discharging,
    /// Battery is empty.
    Empty,
    /// Battery is fully charged.
    Full,
    /// Battery is waiting to charge.
    PendingCharge,
    /// Battery is waiting to discharge.
    PendingDischarge,
}

/// Battery availability and telemetry.
#[derive(Clone, Debug, PartialEq)]
pub enum Battery {
    /// A system battery is present.
    Present {
        /// Current charge level.
        percent: Percent,
        /// Current charging state.
        state: BatteryState,
        /// Best-known time remaining.
        remaining: Option<Duration>,
        /// Signed power rate in watts. Discharge is negative.
        power_rate_watts: Option<f64>,
        /// Battery voltage in volts.
        voltage: Option<f64>,
        /// Battery temperature in degrees Celsius.
        temperature_celsius: Option<f64>,
    },
    /// No system battery is present or readable.
    Missing {
        /// User-facing detail when one exists.
        message: Option<String>,
    },
}

/// Power-profile availability and state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Profiles {
    /// power-profiles-daemon is available.
    Available {
        /// Active profile when the backend reported one Hyprbaric understands.
        active: Option<Profile>,
        /// Selectable profiles reported by the backend.
        available: Vec<Profile>,
        /// Optional performance degradation detail.
        degraded: Option<String>,
        /// Optional performance inhibition detail.
        inhibited: Option<String>,
    },
    /// power-profiles-daemon is not currently usable.
    Unavailable {
        /// User-facing failure detail.
        message: String,
    },
}

/// UI-facing power snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    /// Battery state.
    pub battery: Battery,
    /// Power-profile state.
    pub profiles: Profiles,
}

/// A power command that reached the runtime boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Select a power profile.
    SetProfile {
        /// Requested profile.
        profile: Profile,
    },
}

/// A power command report published to subscribers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command reached its system boundary.
    Started(Command),
    /// The command failed before or at the boundary.
    Failed {
        /// The command that failed.
        command: Command,
        /// User-facing failure detail.
        message: String,
    },
}

impl Percent {
    /// Creates a percentage clamped to `0..=100`.
    pub fn new(value: f64) -> Self {
        if !value.is_finite() || value <= 0.0 {
            return Self(0);
        }

        if value >= 100.0 {
            Self(100)
        } else {
            Self(value.round() as u8)
        }
    }

    /// Returns this percentage as an integer.
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl Profile {
    /// Parses a power-profiles-daemon profile identifier.
    pub fn from_backend(value: &str) -> Option<Self> {
        match value {
            "power-saver" => Some(Self::Saver),
            "balanced" => Some(Self::Balanced),
            "performance" => Some(Self::Performance),
            _ => None,
        }
    }

    /// Returns the power-profiles-daemon profile identifier.
    pub const fn as_backend(self) -> &'static str {
        match self {
            Self::Saver => "power-saver",
            Self::Balanced => "balanced",
            Self::Performance => "performance",
        }
    }
}

impl BatteryState {
    /// Converts a UPower state integer into Hyprbaric's closed state model.
    pub const fn from_upower(value: u32) -> Self {
        match value {
            1 => Self::Charging,
            2 => Self::Discharging,
            3 => Self::Empty,
            4 => Self::Full,
            5 => Self::PendingCharge,
            6 => Self::PendingDischarge,
            _ => Self::Unknown,
        }
    }

    /// Returns whether rates should be shown as draining the battery.
    pub const fn is_discharging(self) -> bool {
        matches!(self, Self::Discharging | Self::PendingDischarge)
    }
}

impl Profiles {
    /// Returns whether a profile is currently selectable.
    pub fn contains(&self, profile: Profile) -> bool {
        match self {
            Self::Available { available, .. } => available.contains(&profile),
            Self::Unavailable { .. } => false,
        }
    }
}

impl Snapshot {
    /// Creates a fully unavailable snapshot.
    pub fn unavailable(message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            battery: Battery::Missing {
                message: Some(message.clone()),
            },
            profiles: Profiles::Unavailable { message },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BatteryState, Percent, Profile};

    #[test]
    fn profile_mapping_matches_power_profiles_daemon() {
        assert_eq!(Profile::from_backend("power-saver"), Some(Profile::Saver));
        assert_eq!(Profile::from_backend("balanced"), Some(Profile::Balanced));
        assert_eq!(
            Profile::from_backend("performance"),
            Some(Profile::Performance)
        );
        assert_eq!(Profile::from_backend("unknown"), None);

        assert_eq!(Profile::Saver.as_backend(), "power-saver");
        assert_eq!(Profile::Balanced.as_backend(), "balanced");
        assert_eq!(Profile::Performance.as_backend(), "performance");
    }

    #[test]
    fn battery_state_mapping_matches_upower() {
        assert_eq!(BatteryState::from_upower(1), BatteryState::Charging);
        assert_eq!(BatteryState::from_upower(2), BatteryState::Discharging);
        assert_eq!(BatteryState::from_upower(3), BatteryState::Empty);
        assert_eq!(BatteryState::from_upower(4), BatteryState::Full);
        assert_eq!(BatteryState::from_upower(5), BatteryState::PendingCharge);
        assert_eq!(BatteryState::from_upower(6), BatteryState::PendingDischarge);
        assert_eq!(BatteryState::from_upower(99), BatteryState::Unknown);
    }

    #[test]
    fn percent_is_clamped() {
        assert_eq!(Percent::new(-4.0).as_u8(), 0);
        assert_eq!(Percent::new(72.4).as_u8(), 72);
        assert_eq!(Percent::new(72.6).as_u8(), 73);
        assert_eq!(Percent::new(120.0).as_u8(), 100);
        assert_eq!(Percent::new(f64::NAN).as_u8(), 0);
    }
}
