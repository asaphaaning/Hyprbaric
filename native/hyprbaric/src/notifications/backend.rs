//! Notification-daemon command boundary.

use tracing::instrument;
use zbus::proxy;

use super::{Error, NotificationId};

#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn close_notification(&self, id: u32) -> zbus::Result<()>;
}

/// Requests that the active notification daemon close one notification.
#[instrument(fields(notification_id = id.as_u32()), err)]
pub(super) async fn close(id: NotificationId) -> Result<(), Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectSessionBusZbus)?;
    let proxy = NotificationsProxy::new(&connection)
        .await
        .map_err(Error::CreateNotificationsProxy)?;
    proxy
        .close_notification(id.as_u32())
        .await
        .map_err(Error::CloseNotification)
}
