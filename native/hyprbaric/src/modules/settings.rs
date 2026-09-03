//! Module visibility settings persistence.

use toml_edit::{DocumentMut, Item, Table, value};
use tracing::instrument;

use crate::config;

use super::{Command, Configuration, Error, Module};

const TABLE: &str = "modules";

#[instrument(skip(command), err)]
pub fn save(command: &Command, current: Configuration) -> Result<Configuration, Error> {
    let next = current.apply(command);
    config::edit(|document| write_modules(document, next))?;

    Ok(next)
}

fn write_modules(document: &mut DocumentMut, config: Configuration) {
    if !document.as_table().contains_key(TABLE) {
        document[TABLE] = Item::Table(Table::new());
    }
    let table = document[TABLE]
        .as_table_mut()
        .expect("modules item should be a table");

    for module in Module::ALL {
        let module_table = module_table(table, module);
        module_table["enabled"] = value(config.enabled(module));
    }
}

fn module_table(table: &mut Table, module: Module) -> &mut Table {
    let key = module.config_key();
    if !table.get(key).is_some_and(Item::is_table) {
        table[key] = Item::Table(Table::new());
    }
    table[key]
        .as_table_mut()
        .expect("module item should be a table")
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use crate::modules::{Command, Configuration, Module};

    use super::save;

    struct EnvGuard {
        key: &'static str,
        previous: Option<std::ffi::OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &PathBuf) -> Self {
            let previous = env::var_os(key);
            unsafe {
                env::set_var(key, value);
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(value) => env::set_var(self.key, value),
                    None => env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    fn save_patches_modules_without_removing_existing_config() {
        let _environment = crate::config::environment_lock();
        let root = env::temp_dir().join(format!("hyprbaric-modules-test-{}", std::process::id()));
        let _guard = EnvGuard::set("XDG_CONFIG_HOME", &root);
        let config_path = root.join("hyprbaric/config.toml");
        fs::create_dir_all(config_path.parent().expect("config should have parent"))
            .expect("config directory should be created");
        fs::write(&config_path, "[night_light]\ntemperature = 3500\n")
            .expect("fixture config should be written");

        let next = save(
            &Command::SetEnabled {
                module: Module::SystemTray,
                enabled: false,
            },
            Configuration::default(),
        )
        .expect("module config should save");

        let source = fs::read_to_string(&config_path).expect("config should be readable");
        assert!(source.contains("[night_light]"));
        assert!(source.contains("[modules.active_window_title]"));
        assert!(source.contains("[modules.system_tray]"));
        assert!(source.contains("[modules.notifications]"));
        assert!(source.contains("[modules.audio_display]"));
        assert!(source.contains("enabled = false"));
        assert!(!next.enabled(Module::SystemTray));

        fs::remove_dir_all(root).expect("fixture config should be removed");
    }
}
