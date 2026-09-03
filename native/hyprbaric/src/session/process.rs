//! Compositor process command boundary.

use std::process::Command;

use tracing::instrument;

use super::Error;

/// Ends the current Hyprland session.
#[instrument(err)]
pub(super) fn logout_hyprland() -> Result<(), Error> {
    let command = "hyprctl dispatch exit".to_string();
    let status = Command::new("hyprctl")
        .args(["dispatch", "exit"])
        .status()
        .map_err(|source| Error::CommandIo {
            command: command.clone(),
            source,
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandStatus { command, status })
    }
}
