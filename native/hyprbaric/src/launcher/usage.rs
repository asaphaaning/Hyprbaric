//! Launcher usage persistence and recency scoring.

use std::{
    collections::HashMap,
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tokio::task;

use super::{Error, Id, desktop::user_data_directory};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsageInfo {
    pub launches: u32,
    pub last_used_unix: i64,
}

pub(super) fn usage_bonus(usage: &UsageInfo, now: i64) -> i64 {
    let launches = i64::from(usage.launches.min(30)) * 3;
    let age = now.saturating_sub(usage.last_used_unix);
    let recency = match age {
        0..=86_400 => 90,
        86_401..=604_800 => 48,
        604_801..=2_592_000 => 18,
        _ => 0,
    };
    launches + recency
}

pub(super) fn usage_path() -> PathBuf {
    user_data_directory()
        .join("hyprbaric")
        .join("launcher_usage.json")
}

pub(super) async fn read_usage_file(path: PathBuf) -> Result<HashMap<Id, UsageInfo>, Error> {
    task::spawn_blocking(move || -> Result<HashMap<Id, UsageInfo>, Error> {
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(HashMap::new()),
            Err(error) => {
                return Err(Error::ReadUsageFile {
                    path: path.clone(),
                    source: error,
                });
            }
        };
        serde_json::from_str(&contents).map_err(|source| Error::ParseUsageFile { path, source })
    })
    .await
    .map_err(Error::Join)?
}

pub(super) async fn write_usage_file(
    path: PathBuf,
    usage: HashMap<Id, UsageInfo>,
) -> Result<(), Error> {
    task::spawn_blocking(move || -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| Error::WriteUsageFile {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let payload =
            serde_json::to_vec_pretty(&usage).map_err(|source| Error::SerializeUsageFile {
                path: path.clone(),
                source,
            })?;
        fs::write(&path, payload).map_err(|source| Error::WriteUsageFile { path, source })
    })
    .await
    .map_err(Error::Join)?
}

pub(super) fn unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::{UsageInfo, unix_timestamp, usage_bonus};

    #[test]
    fn usage_bonus_rewards_recency_and_frequency() {
        let now = unix_timestamp();
        let recent = UsageInfo {
            launches: 5,
            last_used_unix: now,
        };
        let stale = UsageInfo {
            launches: 1,
            last_used_unix: now - 10_000_000,
        };
        assert!(usage_bonus(&recent, now) > usage_bonus(&stale, now));
    }
}
