//! `hyprpicker` process boundary.

use std::{io, process::Output};

use tokio::process::Command as Process;
use tracing::instrument;

use super::{Color, Failure};

/// Color picker process backend.
#[derive(Clone, Copy, Debug)]
pub(super) struct Backend;

impl Backend {
    /// Runs one color pick command against `hyprpicker`.
    #[instrument(skip(self))]
    pub(super) async fn pick(self) -> Result<Color, Failure> {
        let output = Process::new("hyprpicker")
            .arg("--format=hex")
            .arg("--lowercase-hex")
            .arg("--autocopy")
            .arg("--quiet")
            .arg("--render-inactive")
            .output()
            .await
            .map_err(|error| process_spawn_failure("hyprpicker", error))?;

        parse_output("hyprpicker", output)
    }
}

fn parse_output(tool: &'static str, output: Output) -> Result<Color, Failure> {
    if output.status.success() {
        let value = String::from_utf8(output.stdout).map_err(|error| Failure::Utf8 {
            message: error.to_string(),
        })?;
        Color::parse(&value)
    } else if output.stdout.is_empty() && output.stderr.is_empty() {
        Err(Failure::Cancelled)
    } else {
        Err(process_failure(tool, &output))
    }
}

fn process_spawn_failure(tool: &'static str, error: io::Error) -> Failure {
    if error.kind() == io::ErrorKind::NotFound {
        Failure::MissingTool { tool }
    } else {
        Failure::Io {
            message: format!("`{tool}` IO failed: {error}"),
        }
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
    use std::{os::unix::process::ExitStatusExt, process::Output};

    use super::{Failure, parse_output};

    #[test]
    fn output_parses_picked_color() {
        let output = Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"#38bdf8\n".to_vec(),
            stderr: Vec::new(),
        };

        let color = parse_output("hyprpicker", output).expect("color should parse");

        assert_eq!(color.as_str(), "#38bdf8");
    }

    #[test]
    fn empty_failed_output_maps_to_cancelled() {
        let output = Output {
            status: std::process::ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };

        assert_eq!(parse_output("hyprpicker", output), Err(Failure::Cancelled));
    }

    #[test]
    fn invalid_success_output_is_a_failure() {
        let output = Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"not-a-color\n".to_vec(),
            stderr: Vec::new(),
        };

        assert!(matches!(
            parse_output("hyprpicker", output),
            Err(Failure::InvalidColor { .. })
        ));
    }
}
