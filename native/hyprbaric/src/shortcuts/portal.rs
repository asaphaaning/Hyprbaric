//! GlobalShortcuts portal session and activation boundary.

use std::{collections::HashMap, sync::Arc};

use ashpd::desktop::{
    Session as PortalSession,
    global_shortcuts::{GlobalShortcuts, NewShortcut},
};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tracing::instrument;
use zbus::{
    MatchRule, MessageStream,
    message::Type,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use super::{Error, Shortcut, Spec, identity::Identity};

const OBJECT_PATH: &str = "/org/freedesktop/portal/desktop";
const ACTIVATION_INTERFACE: &str = "org.freedesktop.impl.portal.GlobalShortcuts";

/// Live portal state retained while shortcuts are registered.
///
/// The session is the sole owner of its activation [`MessageStream`].
/// [`Session::bind`] and [`Session::next`] expose the two independent portal
/// operations without transferring stream ownership to the registry.
pub(super) struct Session {
    portal: GlobalShortcuts,
    session: PortalSession<GlobalShortcuts>,
    activations: Mutex<MessageStream>,
}

impl Session {
    /// Opens a typed shortcuts session for a prepared host identity.
    #[instrument(
        name = "hyprbaric::shortcuts::portal::connect",
        skip(identity),
        fields(app_id = %identity.app_id()),
        err
    )]
    pub(super) async fn connect(identity: &Identity) -> Result<Arc<Self>, Error> {
        let connection = zbus::Connection::session()
            .await
            .map_err(Error::ConnectPortalBus)?;
        identity.register(connection.clone()).await?;
        let portal = GlobalShortcuts::with_connection(connection)
            .await
            .map_err(Error::ConnectPortal)?;
        let session = portal
            .create_session(Default::default())
            .await
            .map_err(Error::CreateSession)?;
        let activations = activation_stream().await?;

        tracing::info!("Global shortcuts portal session established");

        Ok(Arc::new(Self {
            portal,
            session,
            activations: Mutex::new(activations),
        }))
    }

    /// Waits for the next implementation-side portal activation.
    pub(super) async fn next(&self) -> Option<Result<zbus::Message, zbus::Error>> {
        self.activations.lock().await.next().await
    }

    /// Registers one shortcut spec with the desktop portal.
    #[instrument(skip(self), fields(shortcut_id = %spec.shortcut()), err)]
    pub(super) async fn bind(&self, spec: &Spec) -> Result<(), Error> {
        let portal_id = spec.id().portal_id();
        let trigger = spec.binding().portal_trigger();
        let shortcut = NewShortcut::new(&portal_id, spec.description())
            .preferred_trigger(Some(trigger.as_str()));

        let request = self
            .portal
            .bind_shortcuts(&self.session, &[shortcut], None, Default::default())
            .await
            .map_err(|source| Error::BindPortalShortcut {
                shortcut: spec.shortcut(),
                source,
            })?;
        request
            .response()
            .map_err(|source| Error::BindPortalShortcut {
                shortcut: spec.shortcut(),
                source,
            })?;

        tracing::info!(shortcut = %portal_id, trigger, "Registered shortcut");
        Ok(())
    }
}

/// Parses one implementation-side activation signal into application meaning.
pub(super) fn parse_activation(message: &zbus::Message) -> Result<Option<Shortcut>, zbus::Error> {
    let (_session, shortcut_id, _timestamp, _options) =
        message
            .body()
            .deserialize::<(OwnedObjectPath, String, u64, HashMap<String, OwnedValue>)>()?;

    if let Some(shortcut) = Shortcut::from_portal_id(&shortcut_id) {
        tracing::debug!(%shortcut, shortcut_id, "Shortcut activation received");
        Ok(Some(shortcut))
    } else {
        tracing::warn!("Ignoring activation for unknown shortcut '{shortcut_id}'");
        Ok(None)
    }
}

async fn activation_stream() -> Result<MessageStream, Error> {
    let connection = zbus::Connection::session()
        .await
        .map_err(Error::ConnectActivationBus)?;
    let rule = MatchRule::builder()
        .msg_type(Type::Signal)
        .path(OBJECT_PATH)
        .map_err(Error::SubscribeActivations)?
        .interface(ACTIVATION_INTERFACE)
        .map_err(Error::SubscribeActivations)?
        .member("Activated")
        .map_err(Error::SubscribeActivations)?
        .build();

    MessageStream::for_match_rule(rule, &connection, Some(32))
        .await
        .map_err(Error::SubscribeActivations)
}
