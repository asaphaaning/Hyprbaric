//! `wf-recorder` and `slurp` process boundaries.

use std::{
    env, io,
    path::PathBuf,
    process::{Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use jiff::Zoned;
use serde::Deserialize;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command as Process},
    time::{self, Duration},
};
use tracing::instrument;

use super::{Active, Area, Failure, Mode};

/// Recording process/filesystem backend.
#[derive(Clone, Copy, Debug)]
pub(super) struct Backend;

/// A spawned recording process and its UI projection.
#[derive(Debug)]
pub(super) struct Started {
    pub(super) child: Child,
    pub(super) active: Active,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct Monitor {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scale: f64,
    disabled: bool,
}

impl Backend {
    /// Probes whether `wf-recorder` can be spawned.
    #[instrument(skip(self), err)]
    pub(super) async fn probe(self) -> Result<(), Failure> {
        match Process::new("wf-recorder")
            .arg("--version")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(process_spawn_failure("wf-recorder", error)),
        }
    }

    /// Starts one recording process.
    #[instrument(skip(self), err)]
    pub(super) async fn start(self, mode: Mode) -> Result<Started, Failure> {
        let area = match mode {
            Mode::Region => select_region().await?,
        };
        let path = recordings_dir().await?.join(filename());
        let started_at_ms = unix_ms();
        let geometry = area.wf_geometry();
        let mut command = Process::new("wf-recorder");
        command
            .arg("-g")
            .arg(geometry)
            .arg("-f")
            .arg(&path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = command
            .spawn()
            .map_err(|error| process_spawn_failure("wf-recorder", error))?;

        time::sleep(Duration::from_millis(150)).await;
        match child
            .try_wait()
            .map_err(|error| process_io_failure("wf-recorder", error))?
        {
            Some(status) if !status.success() => Err(Failure::Process {
                tool: "wf-recorder",
                detail: status.to_string(),
            }),
            _ => Ok(Started {
                child,
                active: Active::new(mode, path, started_at_ms),
            }),
        }
    }

    /// Stops one recording process.
    #[instrument(skip(self, child), err)]
    pub(super) async fn stop(self, mut child: Child, active: &Active) -> Result<PathBuf, Failure> {
        interrupt(&mut child).await?;

        match time::timeout(Duration::from_secs(8), child.wait()).await {
            Ok(Ok(status)) if status.success() || active.path().exists() => {
                Ok(active.path().to_path_buf())
            }
            Ok(Ok(status)) => Err(Failure::Process {
                tool: "wf-recorder",
                detail: status.to_string(),
            }),
            Ok(Err(error)) => Err(process_io_failure("wf-recorder", error)),
            Err(_) => {
                drop(child.start_kill());
                Err(Failure::Timeout)
            }
        }
    }
}

#[instrument(err)]
async fn select_region() -> Result<Area, Failure> {
    time::sleep(SELECTION_SETTLE).await;
    let rects = selectable_rects().await?;
    let output = time::timeout(Duration::from_secs(120), slurp(&rects))
        .await
        .map_err(|_| Failure::Timeout)??;
    if !output.status.success() {
        if output.stdout.is_empty() && output.stderr.is_empty() {
            return Err(Failure::Cancelled);
        }
        return Err(process_failure("slurp", &output));
    }

    let geometry = String::from_utf8(output.stdout).map_err(|error| Failure::Utf8 {
        message: error.to_string(),
    })?;
    if geometry.trim().is_empty() {
        return Err(Failure::Cancelled);
    }
    Area::from_slurp(&geometry)
}

const SELECTION_SETTLE: Duration = Duration::from_millis(180);

#[instrument(err)]
async fn selectable_rects() -> Result<String, Failure> {
    let output = output("hyprctl", &["-j", "monitors"]).await?;
    if !output.status.success() {
        return Err(process_failure("hyprctl", &output));
    }

    let monitors =
        serde_json::from_slice::<Vec<Monitor>>(&output.stdout).map_err(|error| Failure::Json {
            message: error.to_string(),
        })?;
    let rects = monitor_rects(&monitors);
    if rects.is_empty() {
        Err(Failure::Process {
            tool: "hyprctl",
            detail: "no enabled monitors were reported".to_owned(),
        })
    } else {
        Ok(rects)
    }
}

fn monitor_rects(monitors: &[Monitor]) -> String {
    monitors
        .iter()
        .filter_map(Monitor::slurp_rect)
        .collect::<Vec<_>>()
        .join("\n")
}

async fn slurp(rects: &str) -> Result<Output, Failure> {
    let mut child = Process::new("slurp");
    child
        .args(slurp_args())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let mut child = child
        .spawn()
        .map_err(|error| process_spawn_failure("slurp", error))?;
    let Some(mut stdin) = child.stdin.take() else {
        return Err(Failure::Io {
            message: "`slurp` did not expose stdin".to_owned(),
        });
    };
    stdin
        .write_all(rects.as_bytes())
        .await
        .map_err(|error| process_io_failure("slurp", error))?;
    stdin
        .write_all(b"\n")
        .await
        .map_err(|error| process_io_failure("slurp", error))?;
    drop(stdin);

    child
        .wait_with_output()
        .await
        .map_err(|error| process_io_failure("slurp", error))
}

fn slurp_args() -> &'static [&'static str] {
    &[
        "-d",
        "-b",
        "#08121a99",
        "-c",
        "#38bdf8ff",
        "-s",
        "#38bdf833",
        "-B",
        "#0b1118ee",
        "-w",
        "2",
    ]
}

impl Monitor {
    fn slurp_rect(&self) -> Option<String> {
        if self.disabled || self.width == 0 || self.height == 0 {
            return None;
        }

        let scale = if self.scale.is_finite() && self.scale > 0.0 {
            self.scale
        } else {
            1.0
        };
        let width = scaled_extent(self.width, scale);
        let height = scaled_extent(self.height, scale);
        Some(format!("{},{} {}x{}", self.x, self.y, width, height))
    }
}

fn scaled_extent(value: u32, scale: f64) -> u32 {
    ((f64::from(value) / scale).round() as u32).max(1)
}

#[instrument(err)]
async fn recordings_dir() -> Result<PathBuf, Failure> {
    let videos = match output("xdg-user-dir", &["VIDEOS"]).await {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8(output.stdout).map_err(|error| Failure::Utf8 {
                message: error.to_string(),
            })?;
            let trimmed = text.trim();
            if trimmed.is_empty() {
                fallback_videos_dir()
            } else {
                PathBuf::from(trimmed)
            }
        }
        _ => fallback_videos_dir(),
    };
    let dir = videos.join("Recordings");
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|error| Failure::Io {
            message: format!(
                "failed to create recording directory `{}`: {error}",
                dir.display()
            ),
        })?;
    Ok(dir)
}

fn fallback_videos_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Videos")
}

fn filename() -> String {
    let now = Zoned::now();
    format!(
        "Recording_{:04}-{:02}-{:02}-{:02}-{:02}-{:02}.mp4",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or_default()
}

#[instrument(skip(child), err)]
async fn interrupt(child: &mut Child) -> Result<(), Failure> {
    let Some(id) = child.id() else {
        return Err(Failure::Process {
            tool: "wf-recorder",
            detail: "recording process has no process id".to_owned(),
        });
    };
    let status = Process::new("kill")
        .arg("-INT")
        .arg(id.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map_err(|error| process_spawn_failure("kill", error))?;

    if status.success() {
        Ok(())
    } else {
        Err(Failure::Process {
            tool: "kill",
            detail: status.to_string(),
        })
    }
}

async fn output(program: &'static str, args: &[&str]) -> Result<Output, Failure> {
    Process::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .await
        .map_err(|error| process_spawn_failure(program, error))
}

fn process_spawn_failure(tool: &'static str, error: io::Error) -> Failure {
    if error.kind() == io::ErrorKind::NotFound {
        Failure::MissingTool { tool }
    } else {
        process_io_failure(tool, error)
    }
}

fn process_io_failure(tool: &'static str, error: io::Error) -> Failure {
    Failure::Io {
        message: format!("`{tool}` IO failed: {error}"),
    }
}

fn process_failure(tool: &'static str, output: &Output) -> Failure {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        output.status.to_string()
    };
    Failure::Process { tool, detail }
}

#[cfg(test)]
mod tests {
    use super::{Monitor, fallback_videos_dir, filename, monitor_rects, slurp_args};

    #[test]
    fn filename_uses_mp4_recording_prefix() {
        let filename = filename();

        assert!(filename.starts_with("Recording_"));
        assert!(filename.ends_with(".mp4"));
    }

    #[test]
    fn fallback_videos_dir_ends_in_videos() {
        let path = fallback_videos_dir();

        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("Videos")
        );
    }

    #[test]
    fn slurp_args_freeze_the_selection_frame() {
        assert!(slurp_args().contains(&"-d"));
        assert!(slurp_args().contains(&"#38bdf8ff"));
        assert!(slurp_args().contains(&"#0b1118ee"));
    }

    #[test]
    fn monitor_rects_use_logical_geometry() {
        let rects = monitor_rects(&[Monitor {
            x: 0,
            y: 0,
            width: 3840,
            height: 2160,
            scale: 1.2,
            disabled: false,
        }]);

        assert_eq!(rects, "0,0 3200x1800");
    }

    #[test]
    fn monitor_rects_skip_disabled_outputs() {
        let rects = monitor_rects(&[
            Monitor {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                scale: 1.0,
                disabled: true,
            },
            Monitor {
                x: 1920,
                y: 0,
                width: 2560,
                height: 1440,
                scale: 1.0,
                disabled: false,
            },
        ]);

        assert_eq!(rects, "1920,0 2560x1440");
    }
}
