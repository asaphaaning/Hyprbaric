//! Setup-guide persistence.

use toml_edit::{DocumentMut, Item, Table, value};
use tracing::instrument;

use crate::config;

use super::{Configuration, Error};

const TABLE: &str = "setup";

/// Persists an acknowledged setup journey.
#[instrument(
    name = "hyprbaric::setup::settings::complete",
    skip(configuration),
    err
)]
pub(super) fn complete(configuration: &Configuration) -> Result<Configuration, Error> {
    let completed = configuration.completed();
    config::edit(|document| write(document, &completed))?;

    Ok(completed)
}

fn write(document: &mut DocumentMut, configuration: &Configuration) {
    if !document.as_table().contains_key(TABLE) {
        document[TABLE] = Item::Table(Table::new());
    }

    let Some(table) = document[TABLE].as_table_mut() else {
        return;
    };
    table["startup"] = value(configuration.startup().as_str());
    table["completed"] = value(configuration.completion().is_complete());
}

#[cfg(test)]
mod tests {
    use toml_edit::DocumentMut;

    use super::write;
    use crate::setup::{Configuration, Status};

    #[test]
    fn completion_preserves_never_policy() {
        let configuration =
            toml::from_str::<Configuration>("startup = \"never\"\ncompleted = false\n")
                .expect("setup fixture should parse")
                .completed();
        let mut document = DocumentMut::new();

        write(&mut document, &configuration);

        assert!(document.to_string().contains("startup = \"never\""));
        assert!(document.to_string().contains("completed = true"));
        assert_eq!(configuration.status(), Status::Disabled);
    }
}
