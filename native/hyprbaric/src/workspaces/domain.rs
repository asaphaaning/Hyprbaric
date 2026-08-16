//! Workspace indicator settings shared by Rust configuration and Flutter UI.

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorStyle {
    #[default]
    Roman,
    Numeric,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum VisibleRange {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub indicator_style: IndicatorStyle,
    pub clickable: bool,
    pub visible_range: VisibleRange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    SetIndicatorStyle { indicator_style: IndicatorStyle },
    SetClickable { clickable: bool },
    SetVisibleRange { visible_range: VisibleRange },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    Started(Command),
    Saved(Command),
    Failed { command: Command, message: String },
}

impl IndicatorStyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Roman => "roman",
            Self::Numeric => "numeric",
        }
    }
}

impl VisibleRange {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }

    pub const fn count(self) -> u8 {
        match self {
            Self::Small => 5,
            Self::Medium => 7,
            Self::Large => 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VisibleRange;

    #[test]
    fn visible_range_projects_to_odd_counts() {
        assert_eq!(VisibleRange::Small.count(), 5);
        assert_eq!(VisibleRange::Medium.count(), 7);
        assert_eq!(VisibleRange::Large.count(), 9);
    }
}
