//! Night-light state published to the bar.
//!
//! This module owns Hyprbaric's small hyprsunset vocabulary. Process and
//! systemd details stay at the backend boundary.

use serde::{Deserialize, Deserializer, de};

/// Default hyprsunset warmth in Kelvin.
pub const DEFAULT_TEMPERATURE: u32 = 3500;

/// A non-zero hyprsunset temperature in Kelvin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Temperature(u32);

/// UI-facing night-light state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snapshot {
    /// hyprsunset can be controlled.
    Available {
        /// Whether the filter is currently enabled by Hyprbaric.
        enabled: bool,
        /// Configured temperature.
        temperature: Temperature,
    },
    /// hyprsunset or its control path is unavailable.
    Unavailable {
        /// Persisted enabled preference.
        enabled: bool,
        /// Configured temperature.
        temperature: Temperature,
        /// User-facing failure detail.
        message: String,
    },
}

/// A night-light command that reached the runtime boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Enable or disable the blue-light filter.
    SetEnabled {
        /// Requested enabled state.
        enabled: bool,
    },
    /// Persist and optionally apply a new temperature.
    SetTemperature {
        /// Requested temperature.
        temperature: Temperature,
    },
}

/// A night-light command report published to subscribers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// The command was accepted by the worker.
    Started(Command),
    /// The command completed and persisted settings.
    Saved(Command),
    /// The command failed before or at its backend.
    Failed {
        /// The command that failed.
        command: Command,
        /// User-facing failure detail.
        message: String,
    },
}

impl Temperature {
    /// Creates a non-zero temperature.
    pub const fn new(value: u32) -> Result<Self, TemperatureError> {
        if value == 0 {
            Err(TemperatureError::Zero)
        } else {
            Ok(Self(value))
        }
    }

    /// Creates the default night-light temperature.
    pub const fn default_value() -> Self {
        Self(DEFAULT_TEMPERATURE)
    }

    /// Returns the Kelvin value expected by hyprsunset.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self::default_value()
    }
}

impl<'de> Deserialize<'de> for Temperature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// Temperature validation error.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TemperatureError {
    /// hyprsunset temperatures must be non-zero.
    #[error("night-light temperature cannot be zero")]
    Zero,
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_TEMPERATURE, Temperature};

    #[test]
    fn temperature_accepts_non_zero_kelvin() {
        let temperature = Temperature::new(2500).expect("temperature should be valid");

        assert_eq!(temperature.as_u32(), 2500);
    }

    #[test]
    fn temperature_rejects_zero() {
        let error = Temperature::new(0).expect_err("zero temperature should fail");

        assert_eq!(error.to_string(), "night-light temperature cannot be zero");
    }

    #[test]
    fn temperature_default_is_warm() {
        assert_eq!(Temperature::default().as_u32(), DEFAULT_TEMPERATURE);
    }
}
