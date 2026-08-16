//! Cheap host probes used by the capability catalog.

use std::{
    env,
    ffi::OsString,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
pub(super) enum Probe {
    AlwaysAvailable,
    Command(&'static str),
    AnyCommand(&'static [&'static str]),
    AllCommands(&'static [&'static str]),
    AnyPath(&'static [&'static str]),
    Brightness,
    HyprlandSession,
    SystemdLogin,
}

impl Probe {
    pub(super) fn failure(self) -> Option<String> {
        match self {
            Self::AlwaysAvailable => None,
            Self::Command(command) => command_missing(command),
            Self::AnyCommand(commands) => commands
                .iter()
                .all(|command| !command_exists(command))
                .then(|| format!("missing one of: {}", commands.join(", "))),
            Self::AllCommands(commands) => {
                let missing = commands
                    .iter()
                    .copied()
                    .filter(|command| !command_exists(command))
                    .collect::<Vec<_>>();
                (!missing.is_empty()).then(|| format!("missing command(s): {}", missing.join(", ")))
            }
            Self::AnyPath(paths) => paths
                .iter()
                .all(|path| !Path::new(path).exists())
                .then(|| format!("missing one of: {}", paths.join(", "))),
            Self::Brightness => {
                let backlight = fs::read_dir("/sys/class/backlight")
                    .ok()
                    .and_then(|mut entries| entries.next())
                    .is_some();
                (!backlight && !command_exists("ddcutil"))
                    .then(|| "no Linux backlight device or ddcutil command found".to_owned())
            }
            Self::HyprlandSession => env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
                .is_none()
                .then(|| "HYPRLAND_INSTANCE_SIGNATURE is not set".to_owned()),
            Self::SystemdLogin => (!Path::new("/run/systemd/system").exists())
                .then(|| "systemd/login1 runtime is not present".to_owned()),
        }
    }
}

fn command_missing(command: &str) -> Option<String> {
    (!command_exists(command)).then(|| format!("missing command: {command}"))
}

pub(super) fn command_exists(command: &str) -> bool {
    env::var_os("PATH")
        .map(path_entries)
        .unwrap_or_default()
        .into_iter()
        .any(|directory| executable_at(directory.join(command)))
}

fn path_entries(path: OsString) -> Vec<PathBuf> {
    env::split_paths(&path).collect()
}

fn executable_at(path: PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
