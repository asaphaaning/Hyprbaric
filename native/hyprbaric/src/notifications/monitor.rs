//! Freedesktop notification monitor boundary.

use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dbus::{
    Message, MessageType,
    arg::{PropMap, RefArg},
    blocking::Connection,
    channel::MatchingReceiver,
    message::MatchRule,
};
use tracing::instrument;

use super::{
    Error,
    domain::{Event, NotificationId, Pending, Urgency},
};

const DBUS_BUS: &str = "org.freedesktop.DBus";
const DBUS_PATH: &str = "/org/freedesktop/DBus";
const NOTIFICATIONS_INTERFACE: &str = "org.freedesktop.Notifications";
const NOTIFICATIONS_PATH: &str = "/org/freedesktop/Notifications";
const PROCESS_TIMEOUT: Duration = Duration::from_secs(5);

/// Starts the D-Bus monitor thread.
#[instrument(skip_all)]
pub(super) fn spawn(on_event: impl Fn(Event) + Send + Sync + 'static) -> Result<(), Error> {
    let connection = Connection::new_session().map_err(Error::ConnectSessionBus)?;
    become_monitor(&connection)?;

    let on_event = Arc::new(on_event);
    connection.start_receive(
        MatchRule::new().static_clone(),
        Box::new(move |message, _| {
            if let Some(event) = parse(&message) {
                on_event(event);
            }
            true
        }),
    );

    thread::Builder::new()
        .name("hyprbaric-notifications".to_string())
        .spawn(move || run(connection))
        .map_err(Error::SpawnWatcher)?;

    Ok(())
}

#[instrument(skip_all)]
fn become_monitor(connection: &Connection) -> Result<(), Error> {
    let proxy = connection.with_proxy(DBUS_BUS, DBUS_PATH, PROCESS_TIMEOUT);
    proxy
        .method_call(
            "org.freedesktop.DBus.Monitoring",
            "BecomeMonitor",
            (vec![MatchRule::new().match_str()], 0u32),
        )
        .map_err(Error::BecomeMonitor)
}

#[instrument(skip_all)]
fn run(connection: Connection) {
    loop {
        if let Err(error) = connection.process(PROCESS_TIMEOUT) {
            tracing::warn!("Notification monitor failed: {error}");
        }
    }
}

fn parse(message: &Message) -> Option<Event> {
    match message.msg_type() {
        MessageType::MethodCall => parse_notify(message),
        MessageType::MethodReturn => parse_notify_return(message),
        MessageType::Signal => parse_signal(message),
        MessageType::Error => None,
    }
}

fn parse_notify(message: &Message) -> Option<Event> {
    if message.path().as_deref() != Some(NOTIFICATIONS_PATH)
        || message.interface().as_deref() != Some(NOTIFICATIONS_INTERFACE)
        || message.member().as_deref() != Some("Notify")
    {
        return None;
    }

    let serial = message.get_serial()?;
    let (app_name, replaces_id, _app_icon, summary, body, _actions, hints, _expire_timeout): (
        String,
        u32,
        String,
        String,
        String,
        Vec<String>,
        PropMap,
        i32,
    ) = message.read_all().ok()?;

    Some(Event::NotifyRequested {
        serial,
        pending: Pending::new(
            app_name,
            &summary,
            &body,
            (replaces_id != 0).then(|| NotificationId::new(replaces_id)),
            now_unix_ms(),
            urgency_from_hints(&hints),
        ),
    })
}

fn parse_notify_return(message: &Message) -> Option<Event> {
    Some(Event::NotifyAssigned {
        reply_serial: message.get_reply_serial()?,
        id: NotificationId::new(message.get1::<u32>()?),
    })
}

fn parse_signal(message: &Message) -> Option<Event> {
    if message.path().as_deref() == Some(NOTIFICATIONS_PATH)
        && message.interface().as_deref() == Some(NOTIFICATIONS_INTERFACE)
        && message.member().as_deref() == Some("NotificationClosed")
    {
        let (notification_id, _) = message.get2::<u32, u32>();
        return notification_id.map(|id| Event::Closed(NotificationId::new(id)));
    }

    if message.path().as_deref() == Some(DBUS_PATH)
        && message.interface().as_deref() == Some(DBUS_BUS)
        && message.member().as_deref() == Some("NameOwnerChanged")
    {
        let (bus_name, _old_owner, new_owner) = message.get3::<String, String, String>();
        if bus_name.as_deref() == Some(NOTIFICATIONS_INTERFACE) {
            return Some(if new_owner.as_deref().is_some_and(str::is_empty) {
                Event::DaemonLost
            } else {
                Event::DaemonAvailable
            });
        }
    }

    None
}

fn urgency_from_hints(hints: &PropMap) -> Urgency {
    hints
        .get("urgency")
        .and_then(|variant| refarg_u8(&*variant.0))
        .map(Urgency::from_hint)
        .unwrap_or_default()
}

fn refarg_u8(value: &dyn RefArg) -> Option<u8> {
    value
        .as_u64()
        .and_then(|value| u8::try_from(value).ok())
        .or_else(|| value.as_i64().and_then(|value| u8::try_from(value).ok()))
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
