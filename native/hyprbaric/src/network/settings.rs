//! System network settings launchers.

use std::process::Command;

use super::Error;

/// Opens the first known network settings application.
pub(super) fn open() -> Result<(), Error> {
    for candidate in [
        Candidate::new("nm-connection-editor", &[]),
        Candidate::new("gnome-control-center", &["wifi"]),
        Candidate::new("systemsettings", &["kcm_networkmanagement"]),
    ] {
        if candidate.spawn() {
            return Ok(());
        }
    }

    Err(Error::SettingsUnavailable)
}

/// A network settings application candidate.
struct Candidate<'a> {
    program: &'a str,
    args: &'a [&'a str],
}

impl<'a> Candidate<'a> {
    /// Creates one settings application candidate.
    const fn new(program: &'a str, args: &'a [&'a str]) -> Self {
        Self { program, args }
    }

    /// Attempts to launch this settings application.
    fn spawn(self) -> bool {
        Command::new(self.program).args(self.args).spawn().is_ok()
    }
}
