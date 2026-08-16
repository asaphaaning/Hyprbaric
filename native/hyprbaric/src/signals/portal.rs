use rinf::{RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PortalColorScheme {
    NoPreference,
    PreferDark,
    PreferLight,
}

#[derive(Serialize, RustSignal)]
pub struct PortalStatus {
    pub color_scheme: PortalColorScheme,
}
