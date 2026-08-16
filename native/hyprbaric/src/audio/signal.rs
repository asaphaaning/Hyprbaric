//! RINF projections for audio state and commands.

use crate::signals;

use super::{Command, Endpoint, EndpointKind, Report, Snapshot};

impl From<signals::AudioEndpointKind> for EndpointKind {
    fn from(kind: signals::AudioEndpointKind) -> Self {
        match kind {
            signals::AudioEndpointKind::Output => Self::Output,
            signals::AudioEndpointKind::Input => Self::Input,
        }
    }
}

impl From<EndpointKind> for signals::AudioEndpointKind {
    fn from(kind: EndpointKind) -> Self {
        match kind {
            EndpointKind::Output => Self::Output,
            EndpointKind::Input => Self::Input,
        }
    }
}

impl From<&Snapshot> for signals::AudioStatus {
    fn from(snapshot: &Snapshot) -> Self {
        match snapshot {
            Snapshot::Available { output, input } => Self::Available {
                output: output.as_ref().map(Into::into),
                input: input.as_ref().map(Into::into),
            },
            Snapshot::Unavailable { message } => Self::Unavailable {
                message: message.clone(),
            },
        }
    }
}

impl From<&Report> for signals::AudioCommandResult {
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

impl From<&Command> for signals::AudioCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::SetVolume { kind, volume } => Self::SetVolume {
                kind: (*kind).into(),
                volume: volume.as_u8(),
            },
            Command::SetMuted { kind, muted } => Self::SetMuted {
                kind: (*kind).into(),
                muted: *muted,
            },
        }
    }
}

impl From<&Endpoint> for signals::AudioEndpoint {
    fn from(endpoint: &Endpoint) -> Self {
        Self {
            kind: endpoint.kind.into(),
            id: endpoint.id.clone(),
            name: endpoint.name.clone(),
            volume: endpoint.volume.as_u8(),
            muted: endpoint.muted,
        }
    }
}
