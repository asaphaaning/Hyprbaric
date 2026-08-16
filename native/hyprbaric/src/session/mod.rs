//! Desktop session actions.
//!
//! Domain types own action vocabulary and availability, login1 owns system bus
//! operations, process owns compositor process commands, and signal owns RINF
//! projections.

mod domain;
mod login1;
mod process;
mod signal;

use std::sync::Arc;

use tracing::instrument;

pub use domain::{Action, Availability, Report};

pub type Handle = Arc<Actions>;

/// Live session action handle.
#[derive(Clone)]
pub struct Actions {
    availability: Availability,
}

impl Actions {
    /// Detects optional session action availability.
    #[instrument]
    pub async fn detect() -> Self {
        let firmware_reboot_supported = match login1::firmware_reboot_supported().await {
            Ok(value) => value,
            Err(error) => {
                tracing::warn!("Failed to probe firmware reboot support via login1: {error}");
                false
            }
        };

        Self {
            availability: Availability {
                firmware_reboot_supported,
            },
        }
    }

    /// Returns detected optional action availability.
    pub fn availability(&self) -> &Availability {
        &self.availability
    }

    /// Executes a session action.
    #[instrument(skip(self), fields(action = action.label()))]
    pub async fn execute(&self, action: Action) -> Result<(), Error> {
        match action {
            Action::Lock => login1::lock_current_session().await,
            Action::Suspend => login1::suspend().await,
            Action::Logout => process::logout_hyprland(),
            Action::Restart => login1::reboot().await,
            Action::Shutdown => login1::power_off().await,
            Action::RebootToFirmware => {
                if !self.availability.firmware_reboot_supported {
                    return Err(Error::UnsupportedAction { action });
                }

                login1::reboot_to_firmware().await
            }
        }
    }
}

/// Session action failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The action is not available on this system.
    #[error("action {action:?} is unavailable on this system")]
    UnsupportedAction { action: Action },
    /// The system bus could not be reached.
    #[error("failed to connect to the system bus")]
    ConnectSystemBus(#[source] zbus::Error),
    /// The login1 manager proxy could not be created.
    #[error("failed to create the login1 manager proxy")]
    CreateManagerProxy(#[source] zbus::Error),
    /// The current login1 session could not be resolved.
    #[error("failed to resolve the current login1 session")]
    ResolveSession(#[source] zbus::Error),
    /// The login1 session proxy could not be created.
    #[error("failed to create the login1 session proxy")]
    CreateSessionProxy(#[source] zbus::Error),
    /// The current session could not be locked.
    #[error("failed to lock the current session via login1")]
    LockSession(#[source] zbus::Error),
    /// Firmware reboot support could not be queried.
    #[error("failed to query firmware reboot availability via login1")]
    ProbeFirmwareSetup(#[source] zbus::Error),
    /// Suspend failed.
    #[error("failed to suspend the system via login1")]
    Suspend(#[source] zbus::Error),
    /// Power off failed.
    #[error("failed to power off the system via login1")]
    PowerOff(#[source] zbus::Error),
    /// Enabling reboot-to-firmware failed.
    #[error("failed to enable reboot-to-firmware-setup via login1")]
    SetFirmwareSetup(#[source] zbus::Error),
    /// Reboot failed.
    #[error("failed to reboot the system via login1")]
    Reboot(#[source] zbus::Error),
    /// A process command could not start.
    #[error("failed to execute `{command}`")]
    CommandIo {
        command: String,
        #[source]
        source: std::io::Error,
    },
    /// A process command returned a non-zero status.
    #[error("command `{command}` exited with status {status}")]
    CommandStatus {
        command: String,
        status: std::process::ExitStatus,
    },
}
