//! Icon-theme and pixmap lookup for launcher entries.

use std::{
    collections::HashMap,
    env, fs,
    io::Read,
    path::{Path, PathBuf},
};

use super::{
    desktop::{user_data_directory, xdg_data_dirs},
    domain::normalize_field,
};

const SVG_PROBE_BYTES: u64 = 128 * 1024;
const PRIMARY_ICON_CONTEXTS: &[&str] = &["apps", "legacy"];
const FALLBACK_ICON_CONTEXTS: &[&str] = &[
    "actions",
    "categories",
    "devices",
    "mimetypes",
    "places",
    "preferences",
    "status",
];

pub(super) struct IconIndex {
    primary_directories: Vec<IconDirectory>,
    fallback_directories: Option<Vec<IconDirectory>>,
    pixmap_directories: Vec<IconDirectory>,
    cache: HashMap<String, Option<PathBuf>>,
}

impl IconIndex {
    pub(super) fn new() -> Self {
        let mut primary_directories = Vec::new();
        let mut pixmap_directories = Vec::new();

        for (root_priority, root) in icon_theme_roots().into_iter().enumerate() {
            collect_icon_theme_directories(
                &root,
                root_priority,
                PRIMARY_ICON_CONTEXTS,
                &mut primary_directories,
            );
        }
        for (root_priority, root) in icon_pixmap_roots().into_iter().enumerate() {
            pixmap_directories.push(IconDirectory {
                root_priority,
                path: root,
            });
        }

        Self {
            primary_directories,
            fallback_directories: None,
            pixmap_directories,
            cache: HashMap::new(),
        }
    }

    pub(super) fn resolve(&mut self, icon: &str) -> Option<PathBuf> {
        let direct = Path::new(icon);
        if (direct.is_absolute() || icon.contains('/')) && direct.exists() {
            return Some(direct.to_path_buf());
        }

        let name = icon_name(icon)?;
        if let Some(candidate) = self.cache.get(name.cache_key()) {
            return candidate.clone();
        }

        let candidate = self.resolve_name(&name);
        self.cache
            .insert(name.cache_key().to_string(), candidate.clone());
        candidate
    }

    fn resolve_name(&mut self, name: &IconName) -> Option<PathBuf> {
        Self::resolve_in_directories(
            name,
            self.primary_directories
                .iter()
                .chain(&self.pixmap_directories),
        )
        .or_else(|| {
            if self.fallback_directories.is_none() {
                self.fallback_directories = Some(collect_fallback_icon_directories());
            }
            Self::resolve_in_directories(
                name,
                self.fallback_directories
                    .as_deref()
                    .unwrap_or_default()
                    .iter(),
            )
        })
    }

    fn resolve_in_directories<'a>(
        name: &IconName,
        directories: impl IntoIterator<Item = &'a IconDirectory>,
    ) -> Option<PathBuf> {
        let mut best = None::<IconCandidate>;

        for directory in directories {
            for stem in name.stems() {
                for extension in ["svg", "png", "xpm"] {
                    let path = directory.path.join(format!("{stem}.{extension}"));
                    if !path.exists() {
                        continue;
                    }

                    let candidate = IconCandidate {
                        score: icon_score(&path, directory.root_priority),
                        path,
                    };

                    match &best {
                        Some(existing) if existing.score >= candidate.score => {}
                        _ => best = Some(candidate),
                    }
                }
            }
        }

        best.map(|candidate| candidate.path)
    }
}

fn collect_fallback_icon_directories() -> Vec<IconDirectory> {
    let mut directories = Vec::new();

    for (root_priority, root) in icon_theme_roots().into_iter().enumerate() {
        collect_icon_theme_directories(
            &root,
            root_priority,
            FALLBACK_ICON_CONTEXTS,
            &mut directories,
        );
    }

    directories
}

struct IconDirectory {
    path: PathBuf,
    root_priority: usize,
}

struct IconCandidate {
    path: PathBuf,
    score: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IconName {
    cache_key: String,
    stems: Vec<String>,
}

impl IconName {
    fn cache_key(&self) -> &str {
        &self.cache_key
    }

    fn stems(&self) -> &[String] {
        &self.stems
    }
}

fn collect_icon_theme_directories(
    root: &Path,
    root_priority: usize,
    contexts: &[&str],
    directories: &mut Vec<IconDirectory>,
) {
    for theme in child_directories(root) {
        push_icon_context_directories(&theme, root_priority, contexts, directories);
        push_icon_context_directories(
            &theme.join("symbolic"),
            root_priority,
            contexts,
            directories,
        );

        for size in child_directories(&theme) {
            push_icon_context_directories(&size, root_priority, contexts, directories);
            push_icon_context_directories(
                &size.join("symbolic"),
                root_priority,
                contexts,
                directories,
            );
        }
    }
}

fn push_icon_context_directories(
    root: &Path,
    root_priority: usize,
    contexts: &[&str],
    directories: &mut Vec<IconDirectory>,
) {
    for context in contexts {
        push_icon_directory_branch(root.join(context), root_priority, directories);
    }
}

fn push_icon_directory_branch(
    path: PathBuf,
    root_priority: usize,
    directories: &mut Vec<IconDirectory>,
) {
    push_icon_directory(path.clone(), root_priority, directories);

    for child in child_directories(&path) {
        push_icon_directory(child, root_priority, directories);
    }
}

fn push_icon_directory(path: PathBuf, root_priority: usize, directories: &mut Vec<IconDirectory>) {
    if path.is_dir() && !directories.iter().any(|directory| directory.path == path) {
        directories.push(IconDirectory {
            path,
            root_priority,
        });
    }
}

fn child_directories(root: &Path) -> Vec<PathBuf> {
    let mut directories = fs::read_dir(root)
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|kind| kind.is_dir())
                .map(|_| entry.path())
        })
        .collect::<Vec<_>>();
    directories.sort();
    directories
}

fn icon_theme_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    roots.push(user_data_directory().join("icons"));
    if let Some(home) = env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".icons"));
    }

    for data_dir in xdg_data_dirs() {
        roots.push(data_dir.join("icons"));
    }

    roots
}

fn icon_pixmap_roots() -> Vec<PathBuf> {
    xdg_data_dirs()
        .into_iter()
        .map(|data_dir| data_dir.join("pixmaps"))
        .collect()
}

#[cfg(test)]
fn icon_file_key(path: &Path) -> Option<String> {
    let extension = path.extension()?.to_string_lossy().to_ascii_lowercase();
    if !matches!(extension.as_str(), "png" | "svg" | "xpm") {
        return None;
    }

    let in_apps = path
        .components()
        .any(|component| component.as_os_str() == "apps");
    let in_pixmaps = path
        .parent()
        .is_some_and(|parent| parent.ends_with("pixmaps"));

    if !in_apps && !in_pixmaps {
        return None;
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_ascii_lowercase())
}

fn icon_name(icon: &str) -> Option<IconName> {
    let value = normalize_field(icon)?;
    let stem = icon_stem(&value);
    let lowercase = stem.to_ascii_lowercase();
    let mut stems = vec![stem.clone()];

    if lowercase != stem {
        stems.push(lowercase);
    }

    Some(IconName {
        cache_key: stem,
        stems,
    })
}

fn icon_stem(icon: &str) -> String {
    let name = Path::new(icon)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(icon);

    for extension in [".svg", ".png", ".xpm"] {
        if name
            .get(name.len().saturating_sub(extension.len())..)
            .is_some_and(|suffix| suffix.eq_ignore_ascii_case(extension))
        {
            return name[..name.len() - extension.len()].to_string();
        }
    }

    name.to_string()
}

fn icon_score(path: &Path, root_priority: usize) -> i32 {
    let extension_score = match path
        .extension()
        .map(|extension| extension.to_string_lossy().to_ascii_lowercase())
        .as_deref()
    {
        Some("svg") => 3_000,
        Some("png") => 2_800,
        Some("xpm") => 1_000,
        _ => 0,
    };
    let render_score = match path
        .extension()
        .map(|extension| extension.to_string_lossy().to_ascii_lowercase())
        .as_deref()
    {
        Some("svg") => svg_render_score(path),
        _ => 0,
    };

    let size_score = path
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(icon_size_score)
        .max()
        .unwrap_or_default();
    let root_score = 500_i32.saturating_sub(root_priority as i32);

    extension_score + render_score + size_score + root_score
}

fn svg_render_score(path: &Path) -> i32 {
    let Some(svg) = read_svg_probe(path) else {
        return -2_400;
    };

    let mut score = 0;
    if svg.contains("<image") {
        score -= 2_400;
    }
    if svg.contains("<filter")
        || svg.contains("<fegaussianblur")
        || svg.contains("<fecomposite")
        || svg.contains("<feflood")
        || svg.contains("<feoffset")
    {
        score -= 2_400;
    }
    score
}

fn read_svg_probe(path: &Path) -> Option<String> {
    let file = fs::File::open(path).ok()?;
    let mut probe = String::new();
    file.take(SVG_PROBE_BYTES).read_to_string(&mut probe).ok()?;
    Some(probe.to_ascii_lowercase())
}

fn icon_size_score(component: &str) -> i32 {
    if component.eq_ignore_ascii_case("scalable") {
        return 512;
    }

    let size = component
        .split_once('x')
        .and_then(|(width, _)| width.parse::<i32>().ok());

    match size {
        Some(64) => 256,
        Some(48) => 224,
        Some(32) => 192,
        Some(24) => 160,
        Some(22) => 144,
        Some(16) => 128,
        Some(value) => value.clamp(0, 96),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, fs, path::Path, process};

    use super::*;

    #[test]
    fn icon_keys_use_app_icon_stems() {
        let path = Path::new("/usr/share/icons/hicolor/scalable/apps/firefox.svg");
        assert_eq!(icon_file_key(path).as_deref(), Some("firefox"));
    }

    #[test]
    fn icon_keys_ignore_non_app_theme_assets() {
        let path = Path::new("/usr/share/icons/breeze_cursors/cursors/pointer");
        assert_eq!(icon_file_key(path), None);
    }

    #[test]
    fn icon_names_keep_dotted_desktop_entry_ids() {
        let name = icon_name("org.gnome.Nautilus").expect("icon name");

        assert_eq!(name.cache_key(), "org.gnome.Nautilus");
        assert_eq!(
            name.stems(),
            &[
                String::from("org.gnome.Nautilus"),
                String::from("org.gnome.nautilus"),
            ]
        );
    }

    #[test]
    fn scalable_icons_score_above_fixed_sizes() {
        assert!(icon_size_score("scalable") > icon_size_score("64x64"));
    }

    #[test]
    fn icon_index_resolves_child_app_directories() -> Result<(), Box<dyn std::error::Error>> {
        let root = env::temp_dir().join(format!(
            "hyprbaric-icon-index-{}-child-app-directories",
            process::id()
        ));
        let icon = root.join("apps").join("scalable").join("example.svg");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(icon.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "icon path should have parent",
            )
        })?)?;
        fs::write(&icon, [])?;

        let mut icons = IconIndex {
            primary_directories: Vec::new(),
            fallback_directories: None,
            pixmap_directories: Vec::new(),
            cache: HashMap::new(),
        };
        push_icon_directory_branch(root.join("apps"), 0, &mut icons.primary_directories);

        assert_eq!(icons.resolve("example").as_deref(), Some(icon.as_path()));

        let _ = fs::remove_dir_all(&root);
        Ok(())
    }

    #[test]
    fn icon_index_preserves_desktop_entry_icon_case() -> Result<(), Box<dyn std::error::Error>> {
        let root = env::temp_dir().join(format!(
            "hyprbaric-icon-index-{}-desktop-entry-icon-case",
            process::id()
        ));
        let icon = root
            .join("apps")
            .join("scalable")
            .join("org.gnome.Nautilus.svg");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(icon.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "icon path should have parent",
            )
        })?)?;
        fs::write(&icon, [])?;

        let mut icons = IconIndex {
            primary_directories: Vec::new(),
            fallback_directories: None,
            pixmap_directories: Vec::new(),
            cache: HashMap::new(),
        };
        push_icon_directory_branch(root.join("apps"), 0, &mut icons.primary_directories);

        assert_eq!(
            icons.resolve("org.gnome.Nautilus").as_deref(),
            Some(icon.as_path())
        );

        let _ = fs::remove_dir_all(&root);
        Ok(())
    }

    #[test]
    fn icon_index_prefers_render_safe_icons() -> Result<(), Box<dyn std::error::Error>> {
        let root = env::temp_dir().join(format!(
            "hyprbaric-icon-index-{}-render-safe-icons",
            process::id()
        ));
        let fragile = root.join("apps").join("scalable").join("example.svg");
        let safe = root.join("apps").join("48x48").join("example.png");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(fragile.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fragile icon path should have parent",
            )
        })?)?;
        fs::create_dir_all(safe.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "safe icon path should have parent",
            )
        })?)?;
        fs::write(
            &fragile,
            br#"<svg><filter id="shadow" /><image href="data:image/png;base64,AA==" /></svg>"#,
        )?;
        fs::write(&safe, [])?;

        let mut icons = IconIndex {
            primary_directories: Vec::new(),
            fallback_directories: None,
            pixmap_directories: Vec::new(),
            cache: HashMap::new(),
        };
        push_icon_directory_branch(root.join("apps"), 0, &mut icons.primary_directories);

        assert_eq!(icons.resolve("example").as_deref(), Some(safe.as_path()));

        let _ = fs::remove_dir_all(&root);
        Ok(())
    }

    #[test]
    fn icon_index_resolves_non_app_theme_contexts() -> Result<(), Box<dyn std::error::Error>> {
        let root = env::temp_dir().join(format!(
            "hyprbaric-icon-index-{}-non-app-contexts",
            process::id()
        ));
        let icon = root
            .join("Theme")
            .join("48x48")
            .join("actions")
            .join("example.png");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(icon.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "icon path should have parent",
            )
        })?)?;
        fs::write(&icon, [])?;

        let mut icons = IconIndex {
            primary_directories: Vec::new(),
            fallback_directories: Some(Vec::new()),
            pixmap_directories: Vec::new(),
            cache: HashMap::new(),
        };
        if let Some(directories) = &mut icons.fallback_directories {
            collect_icon_theme_directories(&root, 0, FALLBACK_ICON_CONTEXTS, directories);
        }

        assert_eq!(icons.resolve("example").as_deref(), Some(icon.as_path()));

        let _ = fs::remove_dir_all(&root);
        Ok(())
    }
}
