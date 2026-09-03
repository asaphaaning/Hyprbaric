//! Desktop portal settings boundary.

mod domain;
mod signal;

use ashpd::desktop::settings::{ColorScheme as PortalPreference, Settings as PortalSettings};
use tracing::instrument;

pub use domain::ColorScheme;

pub struct Settings {
    color_scheme: ColorScheme,
}

impl Settings {
    #[instrument(skip_all, err)]
    pub async fn read() -> Result<Self, Error> {
        let settings = PortalSettings::new().await.map_err(Error::Connect)?;
        let color_scheme = settings
            .color_scheme()
            .await
            .map(ColorScheme::from_portal)
            .map_err(Error::ReadColorScheme)?;

        Ok(Self { color_scheme })
    }

    pub const fn color_scheme(&self) -> ColorScheme {
        self.color_scheme
    }
}

impl ColorScheme {
    fn from_portal(value: PortalPreference) -> Self {
        match value {
            PortalPreference::NoPreference => Self::None,
            PortalPreference::PreferDark => Self::Dark,
            PortalPreference::PreferLight => Self::Light,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to the portal settings interface")]
    Connect(#[source] ashpd::Error),
    #[error("failed to read the portal color scheme preference")]
    ReadColorScheme(#[source] ashpd::Error),
}
