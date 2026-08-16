//! login1 session and power action boundary.

use tracing::instrument;
use zbus::zvariant::OwnedObjectPath;
use zbus::{Connection, proxy};

use super::{Error, domain};

const LOGIN1_INTERACTIVE_AUTH: bool = true;

#[proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LoginManager {
    fn get_session_by_pid(&self, pid: u32) -> zbus::Result<OwnedObjectPath>;
    fn suspend(&self, interactive: bool) -> zbus::Result<()>;
    fn power_off(&self, interactive: bool) -> zbus::Result<()>;
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;
    fn can_reboot_to_firmware_setup(&self) -> zbus::Result<String>;
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()>;
}

#[proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
trait LoginSession {
    fn lock(&self) -> zbus::Result<()>;
}

/// Locks the current login1 session.
#[instrument]
pub(super) async fn lock_current_session() -> Result<(), Error> {
    let connection = system_connection().await?;
    let manager = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    let session_path: OwnedObjectPath = manager
        .get_session_by_pid(std::process::id())
        .await
        .map_err(Error::ResolveSession)?;
    let session = LoginSessionProxy::builder(&connection)
        .path(session_path)
        .map_err(Error::CreateSessionProxy)?
        .build()
        .await
        .map_err(Error::CreateSessionProxy)?;
    session.lock().await.map_err(Error::LockSession)
}

/// Suspends the system.
#[instrument]
pub(super) async fn suspend() -> Result<(), Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    proxy
        .suspend(LOGIN1_INTERACTIVE_AUTH)
        .await
        .map_err(Error::Suspend)
}

/// Reboots the system.
#[instrument]
pub(super) async fn reboot() -> Result<(), Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    proxy
        .reboot(LOGIN1_INTERACTIVE_AUTH)
        .await
        .map_err(Error::Reboot)
}

/// Powers off the system.
#[instrument]
pub(super) async fn power_off() -> Result<(), Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    proxy
        .power_off(LOGIN1_INTERACTIVE_AUTH)
        .await
        .map_err(Error::PowerOff)
}

/// Enables firmware setup and reboots.
#[instrument]
pub(super) async fn reboot_to_firmware() -> Result<(), Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    proxy
        .set_reboot_to_firmware_setup(true)
        .await
        .map_err(Error::SetFirmwareSetup)?;
    proxy
        .reboot(LOGIN1_INTERACTIVE_AUTH)
        .await
        .map_err(Error::Reboot)?;
    Ok(())
}

/// Detects whether reboot-to-firmware is supported.
#[instrument]
pub(super) async fn firmware_reboot_supported() -> Result<bool, Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    let result = proxy
        .can_reboot_to_firmware_setup()
        .await
        .map_err(Error::ProbeFirmwareSetup)?;
    Ok(domain::firmware_setup_supported(&result))
}

async fn system_connection() -> Result<Connection, Error> {
    Connection::system().await.map_err(Error::ConnectSystemBus)
}
