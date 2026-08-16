//! Tray activation command boundary.

use std::time::{SystemTime, UNIX_EPOCH};

use system_tray::client::ActivateRequest;
use tracing::instrument;
use zbus::proxy;

use super::{Activation, ActivationKind, Error, MenuActivation, Outcome, Position, watcher};

#[proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
trait NotifierItemActions {
    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;
    fn secondary_activate(&self, x: i32, y: i32) -> zbus::Result<()>;
    fn x_ayatana_secondary_activate(&self, timestamp: u32) -> zbus::Result<()>;
    fn context_menu(&self, x: i32, y: i32) -> zbus::Result<()>;
}

/// Activates a tray item or opens its context menu when the item is menu-only.
#[instrument(
    skip(client),
    fields(
        item_id = %activation.item_id.as_str(),
        x = activation.position.x(),
        y = activation.position.y(),
    )
)]
pub(super) async fn activate(
    client: &watcher::ClientHandle,
    activation: Activation,
) -> Result<Outcome, Error> {
    let address = activation.item_id.as_str().to_string();
    if activation.kind == ActivationKind::ContextMenu {
        if let Some(menu) = watcher::menu(client, &address, activation.position) {
            return Ok(Outcome::Menu(menu));
        }
        open_context_menu(&address, activation.position).await?;
        return Ok(Outcome::Activated);
    }

    if watcher::item_is_menu(client, destination(&address)) {
        if let Some(menu) = watcher::menu(client, &address, activation.position) {
            return Ok(Outcome::Menu(menu));
        }
        if let Err(error) = open_context_menu(&address, activation.position).await {
            tracing::debug!(
                "Tray context menu failed for {address}: {error}; trying fallback activation"
            );
            tracing::debug!(%error, "Tray context menu failed; trying fallback activation");
            activate_fallback(&address, activation.position).await?;
        }
        return Ok(Outcome::Activated);
    }

    if let Err(error) = activate_default(&address, activation.position).await {
        if let Some(menu) = watcher::menu(client, &address, activation.position) {
            return Ok(Outcome::Menu(menu));
        }
        tracing::debug!(
            "Tray default activation failed for {address}: {error}; trying fallback activation"
        );
        tracing::debug!(%error, "Tray default activation failed; trying fallback activation");
        activate_fallback(&address, activation.position).await?;
    }

    Ok(Outcome::Activated)
}

pub(super) async fn activate_menu_item(
    client: &watcher::ClientHandle,
    activation: MenuActivation,
) -> Result<Outcome, Error> {
    let address = activation.item_id.as_str().to_string();
    let Some(menu_path) = watcher::menu_path(client, &address) else {
        return Err(Error::MenuUnavailable {
            item_id: activation.item_id,
        });
    };

    client
        .activate(ActivateRequest::MenuItem {
            address: destination(&address).to_string(),
            menu_path,
            submenu_id: activation.menu_item_id.get(),
        })
        .await
        .map_err(Error::MenuItemActivate)?;

    Ok(Outcome::Activated)
}

async fn activate_default(address: &str, position: Position) -> Result<(), Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    let proxy = item_proxy(&connection, address).await?;
    proxy
        .activate(position.x(), position.y())
        .await
        .map_err(Error::Activate)?;
    Ok(())
}

async fn activate_fallback(address: &str, position: Position) -> Result<(), Error> {
    if is_ayatana_address(address) {
        match activate_ayatana_secondary(address).await {
            Ok(()) => Ok(()),
            Err(error) => {
                tracing::debug!(
                    "Tray Ayatana secondary activation failed for {address}: {error}; trying standard secondary activation"
                );
                activate_secondary(address, position).await
            }
        }
    } else {
        match activate_secondary(address, position).await {
            Ok(()) => Ok(()),
            Err(error) => {
                tracing::debug!(
                    "Tray standard secondary activation failed for {address}: {error}; trying Ayatana secondary activation"
                );
                activate_ayatana_secondary(address).await
            }
        }
    }
}

async fn activate_ayatana_secondary(address: &str) -> Result<(), Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    let proxy = item_proxy(&connection, address).await?;
    proxy
        .x_ayatana_secondary_activate(timestamp())
        .await
        .map_err(Error::AyatanaSecondaryActivate)?;
    Ok(())
}

async fn activate_secondary(address: &str, position: Position) -> Result<(), Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    let proxy = item_proxy(&connection, address).await?;
    proxy
        .secondary_activate(position.x(), position.y())
        .await
        .map_err(Error::SecondaryActivate)?;
    Ok(())
}

async fn open_context_menu(address: &str, position: Position) -> Result<(), Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectSessionBus)?;
    let proxy = item_proxy(&connection, address).await?;
    proxy
        .context_menu(position.x(), position.y())
        .await
        .map_err(Error::ContextMenu)?;
    Ok(())
}

async fn item_proxy<'a>(
    connection: &'a zbus::Connection,
    address: &'a str,
) -> Result<NotifierItemActionsProxy<'a>, Error> {
    let (destination, path) = split_address(address);
    let proxy = NotifierItemActionsProxy::builder(connection)
        .destination(destination)
        .map_err(Error::CreateItemProxy)?
        .path(path)
        .map_err(Error::CreateItemProxy)?
        .build()
        .await
        .map_err(Error::CreateItemProxy)?;
    Ok(proxy)
}

fn destination(address: &str) -> &str {
    split_address(address).0
}

fn split_address(address: &str) -> (&str, &str) {
    address
        .split_once('/')
        .map_or((address, "/StatusNotifierItem"), |(destination, path)| {
            (destination, &address[destination.len()..][..path.len() + 1])
        })
}

fn is_ayatana_address(address: &str) -> bool {
    split_address(address)
        .1
        .starts_with("/org/ayatana/NotificationItem/")
}

fn timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{destination, is_ayatana_address, split_address};

    #[test]
    fn destination_uses_unique_bus_from_full_address() {
        assert_eq!(
            destination(":1.42/org/ayatana/NotificationItem/indicator_solaar"),
            ":1.42"
        );
    }

    #[test]
    fn split_address_preserves_non_default_path() {
        assert_eq!(
            split_address(":1.42/org/ayatana/NotificationItem/indicator_solaar"),
            (":1.42", "/org/ayatana/NotificationItem/indicator_solaar")
        );
    }

    #[test]
    fn split_address_defaults_to_status_notifier_path() {
        assert_eq!(split_address(":1.42"), (":1.42", "/StatusNotifierItem"));
    }

    #[test]
    fn ayatana_address_detects_appindicator_paths() {
        assert!(is_ayatana_address(
            ":1.42/org/ayatana/NotificationItem/indicator_solaar"
        ));
        assert!(!is_ayatana_address(":1.42/StatusNotifierItem"));
    }
}
