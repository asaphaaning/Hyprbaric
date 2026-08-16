//! RINF projections for battery and power-profile state.

use crate::signals;

use super::{Battery, BatteryState, Command, Profile, Profiles, Report, Snapshot};

impl From<signals::PowerProfile> for Profile {
    fn from(profile: signals::PowerProfile) -> Self {
        match profile {
            signals::PowerProfile::Saver => Self::Saver,
            signals::PowerProfile::Balanced => Self::Balanced,
            signals::PowerProfile::Performance => Self::Performance,
        }
    }
}

impl From<Profile> for signals::PowerProfile {
    fn from(profile: Profile) -> Self {
        match profile {
            Profile::Saver => Self::Saver,
            Profile::Balanced => Self::Balanced,
            Profile::Performance => Self::Performance,
        }
    }
}

impl From<BatteryState> for signals::PowerBatteryState {
    fn from(state: BatteryState) -> Self {
        match state {
            BatteryState::Unknown => Self::Unknown,
            BatteryState::Charging => Self::Charging,
            BatteryState::Discharging => Self::Discharging,
            BatteryState::Empty => Self::Empty,
            BatteryState::Full => Self::Full,
            BatteryState::PendingCharge => Self::PendingCharge,
            BatteryState::PendingDischarge => Self::PendingDischarge,
        }
    }
}

impl From<&Snapshot> for signals::PowerStatus {
    fn from(snapshot: &Snapshot) -> Self {
        let (
            battery_present,
            percentage,
            state,
            remaining_seconds,
            power_rate_watts,
            voltage,
            temperature_celsius,
            battery_message,
        ) = match &snapshot.battery {
            Battery::Present {
                percent,
                state,
                remaining,
                power_rate_watts,
                voltage,
                temperature_celsius,
            } => (
                true,
                Some(percent.as_u8()),
                (*state).into(),
                remaining.map(|duration| duration.as_secs()),
                *power_rate_watts,
                *voltage,
                *temperature_celsius,
                None,
            ),
            Battery::Missing { message } => (
                false,
                None,
                signals::PowerBatteryState::Unknown,
                None,
                None,
                None,
                None,
                message.clone(),
            ),
        };

        let (active_profile, available_profiles, profile_message, degraded, inhibited) =
            match &snapshot.profiles {
                Profiles::Available {
                    active,
                    available,
                    degraded,
                    inhibited,
                } => (
                    active.map(Into::into),
                    available.iter().copied().map(Into::into).collect(),
                    None,
                    degraded.clone(),
                    inhibited.clone(),
                ),
                Profiles::Unavailable { message } => {
                    (None, Vec::new(), Some(message.clone()), None, None)
                }
            };

        Self {
            battery_present,
            percentage,
            state,
            remaining_seconds,
            power_rate_watts,
            voltage,
            temperature_celsius,
            active_profile,
            available_profiles,
            battery_message,
            profile_message,
            degraded,
            inhibited,
        }
    }
}

impl From<&Report> for signals::PowerCommandResult {
    fn from(report: &Report) -> Self {
        match report {
            Report::Started(command) => Self::Started {
                command: command.into(),
            },
            Report::Failed { command, message } => Self::Failed {
                command: command.into(),
                message: message.clone(),
            },
        }
    }
}

impl From<&Command> for signals::PowerCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetProfile { profile } => Self::SetProfile {
                profile: (*profile).into(),
            },
        }
    }
}
