//! hyprsunset process and systemd boundaries.

use std::{
    env, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use tokio::process::Command as ProcessCommand;
use tokio::time::sleep;
use tracing::instrument;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{Error, Temperature};

const HYPRSUNSET: &str = "hyprsunset";
const HYPRCTL: &str = "hyprctl";
const SYSTEMD_DESTINATION: &str = "org.freedesktop.systemd1";
const SYSTEMD_PATH: &str = "/org/freedesktop/systemd1";
const SYSTEMD_MANAGER: &str = "org.freedesktop.systemd1.Manager";
const HYPRSUNSET_UNIT: &str = "hyprsunset.service";
const HYPRSUNSET_SOCKET_RETRIES: usize = 40;
const HYPRSUNSET_SOCKET_RETRY_DELAY: Duration = Duration::from_millis(125);
const DIRECT_START_RETRIES: usize = 2;

/// Boundary object for controlling hyprsunset.
#[derive(Clone, Copy, Debug, Default)]
pub struct Backend;

impl Backend {
    /// Returns whether `hyprsunset` is discoverable on `PATH`.
    #[instrument]
    pub fn is_available(&self) -> bool {
        program_available(HYPRSUNSET)
    }

    /// Restarts hyprsunset through systemd user services, falling back to spawn.
    #[instrument(skip(self), err)]
    async fn restart(&self) -> Result<(), Error> {
        if !self.is_available() {
            return Err(Error::HyprsunsetUnavailable);
        }

        match systemd_unit(UnitAction::Restart).await {
            Ok(()) => Ok(()),
            Err(error) => {
                tracing::warn!("Failed to restart hyprsunset through systemd: {error}");
                if let Err(reset) = reset_failed_systemd_unit().await {
                    tracing::warn!("Failed to reset hyprsunset.service failure state: {reset}");
                } else if let Ok(()) = systemd_unit(UnitAction::Start).await {
                    return Ok(());
                }
                start_process().await
            }
        }
    }

    /// Applies a temperature, starting hyprsunset only if its IPC socket is absent.
    #[instrument(skip(self), err)]
    pub async fn set_temperature_or_start(&self, temperature: Temperature) -> Result<(), Error> {
        let value = temperature.as_u32().to_string();
        match run_hyprsunset_command(&["temperature", &value]).await {
            Ok(()) => Ok(()),
            Err(error) if missing_hyprsunset_socket(&error) => {
                self.recover_and_run(&["temperature", &value]).await
            }
            Err(error) => Err(error),
        }
    }

    /// Clears hyprsunset's transform through `hyprctl hyprsunset identity`.
    #[instrument(skip(self), err)]
    pub async fn disable(&self) -> Result<(), Error> {
        match run_hyprsunset_command(&["identity"]).await {
            Ok(()) => Ok(()),
            Err(Error::CommandFailed { stderr, .. }) if is_missing_socket(&stderr) => {
                tracing::debug!("hyprsunset socket is absent while disabling; treating as off");
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    async fn recover_and_run(&self, args: &[&str]) -> Result<(), Error> {
        self.restart().await?;
        match run_hyprsunset_command_with_retry(args).await {
            Ok(()) => Ok(()),
            Err(error) if missing_hyprsunset_socket(&error) => {
                tracing::warn!(
                    "hyprsunset socket stayed unavailable after systemd restart; trying direct spawn"
                );
                for _ in 0..DIRECT_START_RETRIES {
                    start_process().await?;
                    match run_hyprsunset_command_with_retry(args).await {
                        Ok(()) => return Ok(()),
                        Err(error) if missing_hyprsunset_socket(&error) => continue,
                        Err(error) => return Err(error),
                    }
                }
                Err(error)
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum UnitAction {
    Start,
    Restart,
}

impl UnitAction {
    const fn method(self) -> &'static str {
        match self {
            Self::Start => "StartUnit",
            Self::Restart => "RestartUnit",
        }
    }
}

async fn run_hyprsunset_command_with_retry(args: &[&str]) -> Result<(), Error> {
    let mut attempt = 0;
    loop {
        match run_hyprsunset_command(args).await {
            Ok(()) => return Ok(()),
            Err(Error::CommandFailed { stderr, .. }) if is_missing_socket(&stderr) => {
                if attempt >= HYPRSUNSET_SOCKET_RETRIES {
                    return Err(Error::CommandFailed {
                        program: HYPRCTL,
                        status: "missing hyprsunset socket".to_owned(),
                        stderr,
                    });
                }
                attempt += 1;
                sleep(HYPRSUNSET_SOCKET_RETRY_DELAY).await;
            }
            Err(error) => return Err(error),
        }
    }
}

async fn run_hyprsunset_command(args: &[&str]) -> Result<(), Error> {
    let output = ProcessCommand::new(HYPRCTL)
        .arg("hyprsunset")
        .args(args)
        .output()
        .await
        .map_err(|source| Error::Spawn {
            program: HYPRCTL,
            source,
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = command_output(&output);
        Err(Error::CommandFailed {
            program: HYPRCTL,
            status: output.status.to_string(),
            stderr,
        })
    }
}

fn command_output(output: &std::process::Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    [stdout.trim(), stderr.trim()]
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_missing_socket(stderr: &str) -> bool {
    stderr.contains(".hyprsunset.sock") || stderr.contains("Couldn't connect")
}

fn missing_hyprsunset_socket(error: &Error) -> bool {
    match error {
        Error::CommandFailed { status, stderr, .. } => {
            status == "missing hyprsunset socket" || is_missing_socket(stderr)
        }
        _ => false,
    }
}

#[instrument(err)]
async fn systemd_unit(action: UnitAction) -> Result<(), Error> {
    let conn = Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    let reply = conn
        .call_method(
            Some(SYSTEMD_DESTINATION),
            SYSTEMD_PATH,
            Some(SYSTEMD_MANAGER),
            action.method(),
            &(HYPRSUNSET_UNIT, "replace"),
        )
        .await
        .map_err(|source| Error::UnitAction {
            action: action.method(),
            source,
        })?;
    let _: OwnedObjectPath =
        reply
            .body()
            .deserialize()
            .map_err(|source| Error::UnitActionReply {
                action: action.method(),
                source,
            })?;

    Ok(())
}

#[instrument(err)]
async fn reset_failed_systemd_unit() -> Result<(), Error> {
    let conn = Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    conn.call_method(
        Some(SYSTEMD_DESTINATION),
        SYSTEMD_PATH,
        Some(SYSTEMD_MANAGER),
        "ResetFailedUnit",
        &(HYPRSUNSET_UNIT,),
    )
    .await
    .map_err(Error::ResetFailedUnit)?;

    Ok(())
}

#[instrument(err)]
async fn start_process() -> Result<(), Error> {
    ProcessCommand::new(HYPRSUNSET)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(false)
        .spawn()
        .map_err(|source| Error::Spawn {
            program: HYPRSUNSET,
            source,
        })?;

    Ok(())
}

fn program_available(program: &str) -> bool {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| executable(path.join(program))))
        .unwrap_or(false)
}

fn executable(path: PathBuf) -> bool {
    metadata(&path)
        .map(|permissions| permissions.mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn metadata(path: &Path) -> io::Result<std::fs::Permissions> {
    Ok(std::fs::metadata(path)?.permissions())
}

#[cfg(test)]
mod tests {
    use super::{Error, command_output, is_missing_socket, missing_hyprsunset_socket};
    use std::{os::unix::process::ExitStatusExt, process::Output};

    #[test]
    fn detects_missing_hyprsunset_socket_errors() {
        assert!(is_missing_socket(
            "Couldn't connect to /run/user/1000/hypr/example/.hyprsunset.sock. (3)"
        ));
        assert!(!is_missing_socket("invalid command"));
    }

    #[test]
    fn detects_retry_exhausted_missing_socket_errors() {
        let error = Error::CommandFailed {
            program: "hyprctl",
            status: "missing hyprsunset socket".to_owned(),
            stderr: String::new(),
        };

        assert!(missing_hyprsunset_socket(&error));
    }

    #[test]
    fn failed_hyprctl_output_preserves_stdout_diagnostics() {
        let output = Output {
            status: std::process::ExitStatus::from_raw(3 << 8),
            stdout: b"Couldn't connect to /run/user/1000/hypr/example/.hyprsunset.sock. (3)\n"
                .to_vec(),
            stderr: Vec::new(),
        };

        assert!(is_missing_socket(&command_output(&output)));
    }
}
