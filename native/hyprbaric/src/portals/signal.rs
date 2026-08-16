//! RINF projection for portal domain values.

use super::ColorScheme;

impl From<ColorScheme> for crate::signals::PortalColorScheme {
    fn from(value: ColorScheme) -> Self {
        match value {
            ColorScheme::None => Self::NoPreference,
            ColorScheme::Dark => Self::PreferDark,
            ColorScheme::Light => Self::PreferLight,
        }
    }
}
