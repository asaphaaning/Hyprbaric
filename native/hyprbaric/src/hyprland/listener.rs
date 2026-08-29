//! Hyprland event-listener boundary.

use hyprland::{
    data::{Client, Workspace},
    event_listener::{
        AsyncEventListener, WindowEventData, WindowTitleEventData, WorkspaceEventData,
    },
    prelude::*,
    shared::WorkspaceType,
};
use tokio::sync::broadcast;
use tracing::instrument;

use super::{Error, FocusedWindowSnapshot, WorkspaceSnapshot, workspace_occupancy};

/// Runs compositor observation until Hyprland closes the event listener.
#[instrument(name = "hyprbaric::hyprland::listener::run", skip_all, fields(%hostname), err)]
pub(super) async fn run(
    workspace_sender: broadcast::Sender<WorkspaceSnapshot>,
    focused_window_sender: broadcast::Sender<FocusedWindowSnapshot>,
    hostname: String,
) -> Result<(), Error> {
    let mut listener = AsyncEventListener::new();
    let workspace_sender_clone = workspace_sender.clone();
    listener.add_workspace_changed_handler(move |event: WorkspaceEventData| {
        let sender = workspace_sender_clone.clone();
        Box::pin(async move {
            let (name, is_special) = workspace_name(event.name);
            refresh_workspace(sender, event.id, name, is_special).await;
        })
    });

    let occupancy_sender = workspace_sender.clone();
    listener.add_window_opened_handler(move |_| {
        let sender = occupancy_sender.clone();
        Box::pin(async move { refresh_active_workspace(sender).await })
    });

    let occupancy_sender = workspace_sender.clone();
    listener.add_window_closed_handler(move |_| {
        let sender = occupancy_sender.clone();
        Box::pin(async move { refresh_active_workspace(sender).await })
    });

    listener.add_window_moved_handler(move |_| {
        let sender = workspace_sender.clone();
        Box::pin(async move { refresh_active_workspace(sender).await })
    });

    let active_window_hostname = hostname.clone();
    let focused_window_sender_clone = focused_window_sender.clone();
    listener.add_active_window_changed_handler(move |event: Option<WindowEventData>| {
        let sender = focused_window_sender_clone.clone();
        let event_hostname = active_window_hostname.clone();
        Box::pin(async move {
            if let Some(snapshot) = refresh_focused_window(event.as_ref(), &event_hostname).await {
                drop(sender.send(snapshot));
            }
        })
    });

    let title_change_hostname = hostname;
    listener.add_window_title_changed_handler(move |_event: WindowTitleEventData| {
        let sender = focused_window_sender.clone();
        let event_hostname = title_change_hostname.clone();
        Box::pin(async move {
            if let Some(snapshot) = refresh_focused_window(None, &event_hostname).await {
                drop(sender.send(snapshot));
            }
        })
    });

    listener
        .start_listener_async()
        .await
        .map_err(Error::Listener)
}

async fn refresh_active_workspace(sender: broadcast::Sender<WorkspaceSnapshot>) {
    let workspace = match Workspace::get_active_async().await {
        Ok(workspace) => workspace,
        Err(error) => {
            tracing::warn!(?error, "Failed to refresh active Hyprland workspace");
            return;
        }
    };
    let is_special = workspace.id < 0;

    refresh_workspace(sender, workspace.id, workspace.name, is_special).await;
}

async fn refresh_workspace(
    sender: broadcast::Sender<WorkspaceSnapshot>,
    id: i32,
    name: String,
    is_special: bool,
) {
    match workspace_occupancy().await {
        Ok(occupancy) => {
            drop(sender.send(WorkspaceSnapshot::new(id, name, is_special, occupancy)));
        }
        Err(error) => tracing::warn!(?error, "Failed to refresh Hyprland workspace occupancy"),
    }
}

async fn refresh_focused_window(
    event: Option<&WindowEventData>,
    hostname: &str,
) -> Option<FocusedWindowSnapshot> {
    match Client::get_active_async().await {
        Ok(client) => Some(focused_window(client.as_ref(), hostname)),
        Err(error) => {
            tracing::warn!(?error, "Failed to refresh active client from Hyprland");
            event.map(|value| {
                FocusedWindowSnapshot::new(Some(&value.class), Some(&value.title), hostname)
            })
        }
    }
}

fn focused_window(client: Option<&Client>, hostname: &str) -> FocusedWindowSnapshot {
    FocusedWindowSnapshot::new(
        client.map(|value| value.class.as_str()),
        client.map(|value| value.title.as_str()),
        hostname,
    )
}

fn workspace_name(ty: WorkspaceType) -> (String, bool) {
    match ty {
        WorkspaceType::Regular(name) => (name, false),
        WorkspaceType::Special(Some(name)) => (name, true),
        WorkspaceType::Special(None) => ("special".to_owned(), true),
    }
}
