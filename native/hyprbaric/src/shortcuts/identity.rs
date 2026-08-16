//! Desktop portal identity preparation.

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use ashpd::{AppID, register_host_app_with_connection};
use tracing::instrument;

use super::Error;

pub(super) const APPLICATION_ID: &str = "com.hyprbaric.Hyprbaric";
const DESKTOP_ENTRY_FILE_NAME: &str = "com.hyprbaric.Hyprbaric.desktop";

/// Desktop-entry-backed application identity for a portal connection.
pub(super) struct Identity {
    app_id: AppID,
}

impl Identity {
    /// Prepares the desktop entry identity used by the shortcuts portal.
    #[instrument(name = "hyprbaric::shortcuts::identity::prepare", err)]
    pub(super) async fn prepare() -> Result<Self, Error> {
        let app_id = AppID::try_from(APPLICATION_ID).map_err(Error::RegisterHostApp)?;
        ensure_user_desktop_entry()?;
        Ok(Self { app_id })
    }

    /// Registers this identity on the D-Bus connection that will issue portal
    /// requests. The host registry associates an app ID with a D-Bus peer, so
    /// registering a separate, short-lived connection is ineffective.
    #[instrument(
        name = "hyprbaric::shortcuts::identity::register",
        skip(self, connection),
        err
    )]
    pub(super) async fn register(&self, connection: zbus::Connection) -> Result<(), Error> {
        register_host_app_with_connection(connection, self.app_id.clone())
            .await
            .map_err(Error::RegisterHostApp)
    }

    /// Returns the registered desktop application ID.
    pub(super) fn app_id(&self) -> &AppID {
        &self.app_id
    }
}

fn ensure_user_desktop_entry() -> Result<(), Error> {
    let path = user_applications_dir().join(DESKTOP_ENTRY_FILE_NAME);
    let executable = env::current_exe().map_err(Error::CurrentExe)?;
    let content = desktop_entry_content(&executable);

    match fs::read_to_string(&path) {
        Ok(existing) if existing == content => return Ok(()),
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(Error::ReadDesktopEntry { path, source });
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::CreateDesktopEntryDir {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(&path, content).map_err(|source| Error::WriteDesktopEntry { path, source })
}

fn user_applications_dir() -> PathBuf {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from(".local/share"))
        .join("applications")
}

fn desktop_entry_content(executable: &Path) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Hyprbaric\n\
         GenericName=Hyprland Status Bar\n\
         Comment=Flutter-based status bar tailored for Hyprland on Linux.\n\
         Exec={}\n\
         Icon=hyprbaric\n\
         Terminal=false\n\
         Categories=System;\n\
         Keywords=Hyprland;Wayland;Bar;Status;Panel;\n\
         StartupNotify=false\n",
        quote_exec_path(executable)
    )
}

fn quote_exec_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    if value
        .chars()
        .any(|character| character.is_whitespace() || matches!(character, '"' | '\\' | '$' | '`'))
    {
        let escaped = value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('$', "\\$")
            .replace('`', "\\`");
        format!("\"{escaped}\"")
    } else {
        value.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{desktop_entry_content, quote_exec_path};

    #[test]
    fn desktop_entry_points_at_current_executable() {
        let content = desktop_entry_content(Path::new("/opt/hyprbaric/hyprbaric"));

        assert!(content.contains("Name=Hyprbaric"));
        assert!(content.contains("Exec=/opt/hyprbaric/hyprbaric"));
    }

    #[test]
    fn desktop_entry_quotes_paths_with_spaces() {
        assert_eq!(
            quote_exec_path(Path::new("/opt/Hyprbaric Test/hyprbaric")),
            "\"/opt/Hyprbaric Test/hyprbaric\""
        );
    }
}
