use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Deserializer, de};
use toml_edit::DocumentMut;
use tracing::instrument;

use crate::{
    appearance, brightness, modules, network, night_light, power, schedule, shortcuts, workspaces,
};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub appearance: appearance::Configuration,
    pub brightness: brightness::Configuration,
    pub modules: modules::Configuration,
    pub network: network::Configuration,
    pub night_light: night_light::Configuration,
    pub power: power::Configuration,
    pub schedules: schedule::Configuration,
    pub shortcuts: shortcuts::Configuration,
    pub workspaces: workspaces::Configuration,
}

impl Configuration {
    #[instrument]
    pub fn load() -> Result<Self, Error> {
        for candidate in candidates() {
            match read(&candidate)? {
                Some(config) => {
                    tracing::info!(path = %candidate.display(), "Loaded Hyprbaric config");
                    return Ok(config);
                }
                None => continue,
            }
        }

        tracing::debug!("No Hyprbaric config override found; using defaults");
        Ok(Self::default())
    }
}

pub(crate) fn writable_user_path() -> Result<PathBuf, Error> {
    let Some(config_home) = config_home() else {
        return Err(Error::ConfigHome);
    };
    Ok(config_home.join("hyprbaric/config.toml"))
}

/// Edits the writable user configuration and replaces it atomically.
///
/// Feature modules own the shape of their TOML tables. This boundary owns the
/// shared filesystem operation so parsing and replacement behave consistently.
#[instrument(skip(apply))]
pub(crate) fn edit(apply: impl FnOnce(&mut DocumentMut)) -> Result<(), Error> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let path = writable_user_path()?;
    edit_path(&path, apply)
}

pub(crate) fn edit_path(path: &Path, apply: impl FnOnce(&mut DocumentMut)) -> Result<(), Error> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(source) => {
            return Err(Error::ReadDocument {
                path: path.to_path_buf(),
                source,
            });
        }
    };
    let mut document = source
        .parse::<DocumentMut>()
        .map_err(|source| Error::ParseDocument {
            path: path.to_path_buf(),
            source,
        })?;

    apply(&mut document);
    replace(path, document.to_string().as_bytes())
}

fn replace(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::CreateDirectory {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let temporary = path.with_extension("toml.tmp");
    fs::write(&temporary, bytes).map_err(|source| Error::WriteDocument {
        path: temporary.clone(),
        source,
    })?;
    fs::rename(&temporary, path).map_err(|source| Error::ReplaceDocument {
        from: temporary,
        to: path.to_path_buf(),
        source,
    })
}

/// A non-zero timing cadence loaded from Hyprbaric configuration.
///
/// [`Cadence`] accepts human-readable durations through [`humantime_serde`]
/// while ensuring timer intervals and command timeouts cannot be zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Cadence(Duration);

impl Cadence {
    pub(crate) const fn seconds(seconds: u64) -> Self {
        Self(Duration::from_secs(seconds))
    }

    pub(crate) const fn milliseconds(milliseconds: u64) -> Self {
        Self(Duration::from_millis(milliseconds))
    }

    pub(crate) const fn duration(self) -> Duration {
        self.0
    }
}

impl<'de> Deserialize<'de> for Cadence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let duration: Duration = humantime_serde::deserialize(deserializer)?;

        if duration.is_zero() {
            return Err(de::Error::custom("cadence cannot be zero"));
        }

        Ok(Self(duration))
    }
}

fn read(path: &Path) -> Result<Option<Configuration>, Error> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(Error::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    toml::from_str(&source)
        .map(Some)
        .map_err(|source| Error::Parse {
            path: path.to_path_buf(),
            source,
        })
}

fn candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(path) = env::var_os("HYPRBARIC_CONFIG") {
        paths.push(PathBuf::from(path));
    }

    if let Some(config_home) = config_home() {
        paths.push(config_home.join("hyprbaric/config.toml"));
    }

    for config_dir in config_dirs() {
        let path = config_dir.join("hyprbaric/config.toml");
        if !paths.contains(&path) {
            paths.push(path);
        }
    }

    paths
}

fn config_home() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
}

fn config_dirs() -> Vec<PathBuf> {
    env::var_os("XDG_CONFIG_DIRS")
        .map(|dirs| env::split_paths(&dirs).collect())
        .unwrap_or_else(|| vec![PathBuf::from("/etc/xdg")])
}

#[cfg(test)]
pub(crate) fn environment_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not determine the user config directory")]
    ConfigHome,
    #[error("failed to read config `{}`", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse config `{}`", path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("failed to read user config `{}`", path.display())]
    ReadDocument {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse user config `{}` for editing", path.display())]
    ParseDocument {
        path: PathBuf,
        #[source]
        source: toml_edit::TomlError,
    },
    #[error("failed to create user config directory `{}`", path.display())]
    CreateDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to write user config `{}`", path.display())]
    WriteDocument {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to replace user config `{}` with `{}`", to.display(), from.display())]
    ReplaceDocument {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: io::Error,
    },
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde::Deserialize;

    use super::Cadence;

    #[derive(Debug, Deserialize)]
    struct CadenceFixture {
        cadence: Cadence,
    }

    #[test]
    fn cadence_accepts_human_durations() {
        let fixture = toml::from_str::<CadenceFixture>(r#"cadence = "250ms""#)
            .expect("cadence should parse human duration");

        assert_eq!(fixture.cadence.duration(), Duration::from_millis(250));
    }

    #[test]
    fn cadence_rejects_zero_duration() {
        let error = toml::from_str::<CadenceFixture>(r#"cadence = "0s""#)
            .expect_err("zero cadence should be rejected");

        assert!(error.to_string().contains("cadence cannot be zero"));
    }

    #[test]
    fn cadence_constructors_are_duration_projections() {
        assert_eq!(Cadence::seconds(2).duration(), Duration::from_secs(2));
        assert_eq!(
            Cadence::milliseconds(300).duration(),
            Duration::from_millis(300)
        );
    }
}
