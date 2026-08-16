//! login1 inhibitor boundary.

use tracing::instrument;
use zbus::zvariant::OwnedFd;
use zbus::{Connection, proxy};

use super::Error;

const INHIBIT_WHAT: &str = "idle:sleep";
const INHIBIT_WHO: &str = "hyprbaric";
const INHIBIT_WHY: &str = "Caffeine mode enabled";
const INHIBIT_MODE: &str = "block";

#[proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LoginManager {
    fn inhibit(&self, what: &str, who: &str, why: &str, mode: &str) -> zbus::Result<OwnedFd>;
}

/// Held login1 inhibitor. Dropping it releases Caffeine.
pub struct Guard {
    _fd: OwnedFd,
}

/// Probes whether login1 is reachable.
#[instrument]
pub async fn probe() -> Result<(), Error> {
    let connection = system_connection().await?;
    LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    Ok(())
}

/// Acquires an idle/sleep inhibitor.
#[instrument]
pub async fn inhibit() -> Result<Guard, Error> {
    let connection = system_connection().await?;
    let proxy = LoginManagerProxy::new(&connection)
        .await
        .map_err(Error::CreateManagerProxy)?;
    let fd = proxy
        .inhibit(INHIBIT_WHAT, INHIBIT_WHO, INHIBIT_WHY, INHIBIT_MODE)
        .await
        .map_err(Error::Inhibit)?;

    Ok(Guard { _fd: fd })
}

async fn system_connection() -> Result<Connection, Error> {
    Connection::system().await.map_err(Error::ConnectSystemBus)
}
