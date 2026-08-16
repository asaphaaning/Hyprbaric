//! XDG desktop-entry discovery and parsing.

use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

use freedesktop_desktop_entry::DesktopEntry;

use super::{
    domain::{Entry, Id, SearchFields, normalize, normalize_field},
    process::{command_exists, command_name},
};

#[derive(Clone, Debug)]
pub(super) struct DesktopDirectory {
    pub(super) root: PathBuf,
}

#[derive(Clone, Debug)]
pub(super) struct WatchTarget {
    pub(super) path: PathBuf,
    pub(super) recursive: bool,
}

enum Shadowed {
    Hidden,
    Present(Entry),
}

pub(super) fn build_index(desktop_dirs: Vec<DesktopDirectory>, locales: Vec<String>) -> Vec<Entry> {
    let desktops = current_desktops();
    let mut entries = HashMap::<String, Shadowed>::new();

    for directory in desktop_dirs {
        for path in collect_desktop_files(&directory.root) {
            let Some(id) = desktop_id(&directory.root, &path) else {
                continue;
            };
            if entries.contains_key(&id) {
                continue;
            }

            let desktop_entry = match DesktopEntry::from_path(&path, Some(&locales)) {
                Ok(entry) => entry,
                Err(error) => {
                    tracing::warn!(
                        "Skipping malformed desktop entry {}: {error}",
                        path.display()
                    );
                    continue;
                }
            };

            match build_entry(id.clone(), path, desktop_entry, &locales, &desktops) {
                Some(Shadowed::Hidden) => {
                    entries.insert(id, Shadowed::Hidden);
                }
                Some(Shadowed::Present(entry)) => {
                    entries.insert(entry.id.to_string(), Shadowed::Present(entry));
                }
                None => {}
            }
        }
    }

    let mut values = entries
        .into_values()
        .filter_map(|entry| match entry {
            Shadowed::Hidden => None,
            Shadowed::Present(entry) => Some(entry),
        })
        .collect::<Vec<_>>();
    values.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.id.cmp(&right.id))
    });
    values
}

fn build_entry(
    id: String,
    path: PathBuf,
    desktop: DesktopEntry,
    locales: &[String],
    desktops: &HashSet<String>,
) -> Option<Shadowed> {
    if desktop.hidden() {
        return Some(Shadowed::Hidden);
    }
    if desktop.type_() != Some("Application") || desktop.no_display() {
        return None;
    }
    if !matches_current_desktop(&desktop, desktops) {
        return None;
    }
    let exec = normalize_field(desktop.exec()?)?;
    if let Some(try_exec) = desktop.try_exec().and_then(normalize_field) {
        if !command_exists(&try_exec) {
            return None;
        }
    }

    let name = desktop
        .full_name(locales)
        .or_else(|| desktop.name(locales))
        .map(|value| value.into_owned())
        .and_then(|value| normalize_field(&value))?;
    let generic_name = desktop
        .generic_name(locales)
        .map(|value| value.into_owned())
        .and_then(|value| normalize_field(&value));
    let comment = desktop
        .comment(locales)
        .map(|value| value.into_owned())
        .and_then(|value| normalize_field(&value));
    let subtitle = comment.clone().or(generic_name);
    let keywords = desktop
        .keywords(locales)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|keyword| normalize_field(keyword.as_ref()))
        .collect::<Vec<_>>();

    let icon_name = desktop.icon().and_then(normalize_field);

    Some(Shadowed::Present(Entry {
        id: Id::new(id)?,
        name: name.clone(),
        subtitle: subtitle.clone(),
        icon_resolved: icon_name.is_none(),
        icon_name,
        icon_path: None,
        terminal: desktop.terminal(),
        desktop_path: path,
        exec: exec.clone(),
        working_dir: desktop.path().and_then(normalize_field).map(PathBuf::from),
        normalized: SearchFields {
            name: normalize(&name),
            exec: normalize(&command_name(&exec)),
            subtitle: subtitle.as_deref().map(normalize).unwrap_or_default(),
            keywords: keywords.iter().map(|keyword| normalize(keyword)).collect(),
        },
    }))
}

fn matches_current_desktop(entry: &DesktopEntry, desktops: &HashSet<String>) -> bool {
    if let Some(only_show_in) = entry.only_show_in() {
        if desktops.is_empty() {
            return false;
        }
        if !only_show_in
            .iter()
            .any(|desktop| desktops.contains(&desktop.to_ascii_lowercase()))
        {
            return false;
        }
    }

    if let Some(not_show_in) = entry.not_show_in() {
        if not_show_in
            .iter()
            .any(|desktop| desktops.contains(&desktop.to_ascii_lowercase()))
        {
            return false;
        }
    }

    true
}

fn current_desktops() -> HashSet<String> {
    env::var("XDG_CURRENT_DESKTOP")
        .ok()
        .into_iter()
        .flat_map(|value| value.split(':').map(str::to_string).collect::<Vec<_>>())
        .map(|desktop| desktop.to_ascii_lowercase())
        .collect()
}

fn collect_desktop_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    visit_desktop_files(root, &mut files);
    files.sort();
    files
}

fn visit_desktop_files(root: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_desktop_files(&path, files);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "desktop")
        {
            files.push(path);
        }
    }
}

pub(super) fn desktop_id(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok().unwrap_or(path);
    let mut parts = Vec::new();
    for component in relative.components() {
        let component = component.as_os_str().to_str()?;
        parts.push(component);
    }
    if parts.is_empty() {
        return None;
    }
    Some(parts.join("-"))
}

pub(super) fn application_directories() -> Vec<DesktopDirectory> {
    [
        user_data_directory().join("applications"),
        PathBuf::from("/usr/local/share/applications"),
        PathBuf::from("/usr/share/applications"),
    ]
    .into_iter()
    .map(|root| DesktopDirectory { root })
    .collect()
}

pub(super) fn watch_targets(directories: &[DesktopDirectory]) -> Vec<WatchTarget> {
    let mut seen = HashSet::new();
    let mut targets = Vec::new();

    for directory in directories {
        let (path, recursive) = if directory.root.exists() {
            (directory.root.clone(), true)
        } else if let Some(parent) = directory.root.parent() {
            (parent.to_path_buf(), false)
        } else {
            continue;
        };

        if seen.insert(path.clone()) {
            targets.push(WatchTarget { path, recursive });
        }
    }

    targets
}

pub(super) fn user_data_directory() -> PathBuf {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("/tmp"))
}

pub(super) fn xdg_data_dirs() -> Vec<PathBuf> {
    env::var_os("XDG_DATA_DIRS")
        .map(|paths| env::split_paths(&paths).collect())
        .filter(|paths: &Vec<PathBuf>| !paths.is_empty())
        .unwrap_or_else(|| {
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share"),
            ]
        })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::desktop_id;

    #[test]
    fn desktop_ids_use_relative_path_components() {
        let root = Path::new("/usr/share/applications");
        let path = Path::new("/usr/share/applications/org/mozilla/firefox.desktop");
        assert_eq!(
            desktop_id(root, path).as_deref(),
            Some("org-mozilla-firefox.desktop")
        );
    }
}
