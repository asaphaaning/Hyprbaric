//! Setup-guide policy, completion, and command vocabulary.

use serde::Deserialize;

/// Policy controlling whether an incomplete guide opens during startup.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Startup {
    /// Open the guide until it is finished or skipped once.
    #[default]
    Once,
    /// Never open the guide automatically.
    Never,
}

/// Whether the first-run journey has already been acknowledged.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Completion(bool);

/// Setup state projected to the Flutter application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// The guide should open automatically on this launch.
    Required,
    /// The guide was finished or skipped previously.
    Complete,
    /// Automatic setup was disabled by configuration.
    Disabled,
}

/// Why the user acknowledged the setup journey.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The final setup step was completed.
    Finished,
    /// The journey was skipped.
    Skipped,
}

/// A setup-guide request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Persist that the setup journey was acknowledged.
    Complete(Outcome),
}

/// Result of applying one [`Command`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Report {
    /// Persistence began.
    Started(Command),
    /// Completion was persisted.
    Saved(Command),
    /// Completion could not be persisted.
    Failed { command: Command, message: String },
}

impl Startup {
    /// Returns the TOML representation of this policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::Never => "never",
        }
    }
}

impl Completion {
    /// The journey has not been acknowledged.
    pub const PENDING: Self = Self(false);

    /// The journey has been acknowledged.
    pub const COMPLETE: Self = Self(true);

    /// Returns whether the journey has been acknowledged.
    pub const fn is_complete(self) -> bool {
        self.0
    }
}
