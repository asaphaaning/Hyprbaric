use std::{env, sync::Once};

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(init_once);
}

fn init_once() {
    let log_tracer_warning = tracing_log::LogTracer::init()
        .err()
        .map(|error| error.to_string());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let (journald, journald_warning) = match tracing_journald::layer() {
        Ok(layer) => (Some(layer), None),
        Err(error) => (None, Some(error.to_string())),
    };

    let result = if Format::from_env() == Format::Json {
        tracing_subscriber::registry()
            .with(filter)
            .with(journald)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_ansi(false)
                    .with_current_span(false)
                    .with_span_list(false)
                    .with_target(true),
            )
            .try_init()
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(journald)
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_ansi(ansi_enabled())
                    .with_file(false)
                    .with_line_number(false)
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_thread_names(false),
            )
            .try_init()
    };

    match result {
        Ok(()) => {
            if let Some(error) = log_tracer_warning {
                tracing::warn!(%error, "Failed to install log tracer");
            }
            if let Some(error) = journald_warning {
                tracing::warn!(%error, "Failed to install journald tracing layer");
            }
        }
        Err(error) => {
            tracing::debug!(%error, "Tracing subscriber was already installed");
        }
    }
}

fn ansi_enabled() -> bool {
    !matches!(env::var("RUST_LOG_STYLE").as_deref(), Ok("never"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    Compact,
    Json,
}

impl Format {
    fn from_env() -> Self {
        match env::var("HYPRBARIC_LOG") {
            Ok(value) if value.eq_ignore_ascii_case("json") => Self::Json,
            _ => Self::Compact,
        }
    }
}
