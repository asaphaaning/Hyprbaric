//! Audio system boundary.
//!
//! [`Devices`] owns `wpctl` calls and parsing so the runtime can stay focused on
//! publishing typed [`Snapshot`] and [`super::Report`]
//! values.

use tokio::process::Command as ProcessCommand;
use tracing::instrument;

use super::{
    Error,
    domain::{Endpoint, EndpointKind, Percent, Snapshot},
};

/// PipeWire default endpoint reader and writer.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Devices;

impl Devices {
    /// Reads the current audio snapshot.
    #[instrument(skip(self))]
    pub(super) async fn read_snapshot(self) -> Result<Snapshot, Error> {
        let output = self.read_endpoint(EndpointKind::Output).await.ok();
        let input = self.read_endpoint(EndpointKind::Input).await.ok();
        if output.is_none() && input.is_none() {
            return Err(Error::Unavailable);
        }

        Ok(Snapshot::Available { output, input })
    }

    /// Reads one default endpoint.
    #[instrument(skip(self))]
    async fn read_endpoint(self, kind: EndpointKind) -> Result<Endpoint, Error> {
        let inspect = run("wpctl", &["inspect", kind.selector()]).await?;
        let volume = run("wpctl", &["get-volume", kind.selector()]).await?;
        let (volume, muted) = parse_volume(&volume)?;

        Ok(Endpoint {
            kind,
            id: parse_id(&inspect),
            name: parse_name(&inspect).unwrap_or_else(|| kind.fallback_name().to_string()),
            volume,
            muted,
        })
    }

    /// Sets the volume for one endpoint class.
    #[instrument(skip(self))]
    pub(super) async fn set_volume(self, kind: EndpointKind, volume: Percent) -> Result<(), Error> {
        let value = format!("{}%", volume.as_u8());
        run("wpctl", &["set-volume", kind.selector(), &value])
            .await
            .map(|_| ())
    }

    /// Sets mute state for one endpoint class.
    #[instrument(skip(self))]
    pub(super) async fn set_muted(self, kind: EndpointKind, muted: bool) -> Result<(), Error> {
        let value = if muted { "1" } else { "0" };
        run("wpctl", &["set-mute", kind.selector(), value])
            .await
            .map(|_| ())
    }
}

#[instrument(skip(args))]
async fn run(program: &str, args: &[&str]) -> Result<String, Error> {
    let output = ProcessCommand::new(program)
        .args(args)
        .output()
        .await
        .map_err(|source| Error::Spawn {
            program: program.to_string(),
            source,
        })?;
    if !output.status.success() {
        return Err(Error::CommandFailed {
            program: program.to_string(),
            status: output.status.to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    String::from_utf8(output.stdout).map_err(Error::Utf8)
}

fn parse_id(inspect: &str) -> Option<String> {
    inspect
        .lines()
        .next()
        .and_then(|line| line.trim().strip_prefix("id "))
        .and_then(|tail| tail.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn parse_name(inspect: &str) -> Option<String> {
    for key in ["node.description", "node.nick", "node.name"] {
        if let Some(value) = parse_property(inspect, key) {
            return Some(value);
        }
    }
    None
}

fn parse_property(inspect: &str, key: &str) -> Option<String> {
    inspect.lines().find_map(|line| {
        let normalized = line.trim_start().trim_start_matches("* ").trim_start();
        let (left, right) = normalized.split_once(" = ")?;
        if left == key {
            Some(unquote(right.trim()))
        } else {
            None
        }
    })
}

fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}

fn parse_volume(output: &str) -> Result<(Percent, bool), Error> {
    let value = output
        .trim()
        .strip_prefix("Volume:")
        .ok_or_else(|| Error::ParseVolume {
            output: output.trim().to_string(),
        })?
        .split_whitespace()
        .next()
        .ok_or_else(|| Error::ParseVolume {
            output: output.trim().to_string(),
        })?;
    let fraction = value.parse::<f32>().map_err(|_| Error::ParseVolume {
        output: output.trim().to_string(),
    })?;
    Ok((Percent::from_fraction(fraction), output.contains("MUTED")))
}

#[cfg(test)]
mod tests {
    use super::{Percent, parse_id, parse_name, parse_volume};

    #[test]
    fn parses_wpctl_volume() {
        let (volume, muted) = parse_volume("Volume: 0.85\n").unwrap();
        assert_eq!(volume, Percent::new(85));
        assert!(!muted);
    }

    #[test]
    fn parses_muted_wpctl_volume() {
        let (volume, muted) = parse_volume("Volume: 0.42 [MUTED]\n").unwrap();
        assert_eq!(volume, Percent::new(42));
        assert!(muted);
    }

    #[test]
    fn parses_default_endpoint_metadata() {
        let inspect = r#"id 117, type PipeWire:Interface:Node
  * node.description = "EVO4 Analog Surround 4.0"
  * node.name = "alsa_output.usb-Audient_EVO4-00.analog-surround-40"
"#;
        assert_eq!(parse_id(inspect).as_deref(), Some("117"));
        assert_eq!(
            parse_name(inspect).as_deref(),
            Some("EVO4 Analog Surround 4.0")
        );
    }
}
