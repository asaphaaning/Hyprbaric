//! RINF projections for tray state and commands.

use crate::signals;

use super::{
    Activation, ActivationKind, Icon, Item, ItemId, Menu, MenuActivation, MenuItem, MenuItemId,
    MenuItemKind, Position, Snapshot, Status,
};

impl From<signals::TrayActivateRequest> for Activation {
    fn from(request: signals::TrayActivateRequest) -> Self {
        Self {
            item_id: ItemId::new(request.id),
            position: Position::new(request.x, request.y),
            kind: request.kind.into(),
        }
    }
}

impl From<signals::TrayActivationKind> for ActivationKind {
    fn from(kind: signals::TrayActivationKind) -> Self {
        match kind {
            signals::TrayActivationKind::Primary => Self::Primary,
            signals::TrayActivationKind::ContextMenu => Self::ContextMenu,
        }
    }
}

impl From<signals::TrayMenuItemActivateRequest> for MenuActivation {
    fn from(request: signals::TrayMenuItemActivateRequest) -> Self {
        Self {
            item_id: ItemId::new(request.item_id),
            menu_item_id: MenuItemId::new(request.menu_item_id),
        }
    }
}

impl From<&Snapshot> for signals::TrayStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            items: snapshot.items.iter().map(Into::into).collect(),
            message: snapshot.message.clone(),
        }
    }
}

impl From<&Menu> for signals::TrayMenuStatus {
    fn from(menu: &Menu) -> Self {
        Self {
            item_id: menu.item_id.as_str().to_string(),
            x: menu.position.x(),
            y: menu.position.y(),
            items: menu.items.iter().map(Into::into).collect(),
        }
    }
}

impl From<&MenuItem> for signals::TrayMenuItem {
    fn from(item: &MenuItem) -> Self {
        Self {
            id: item.id.get(),
            label: item.label.clone(),
            enabled: item.enabled,
            kind: item.kind.into(),
            depth: item.depth,
        }
    }
}

impl From<MenuItemKind> for signals::TrayMenuItemKind {
    fn from(kind: MenuItemKind) -> Self {
        match kind {
            MenuItemKind::Standard => Self::Standard,
            MenuItemKind::Separator => Self::Separator,
        }
    }
}

impl From<&Item> for signals::TrayItem {
    fn from(item: &Item) -> Self {
        Self {
            id: item.id.as_str().to_owned(),
            title: item.title.clone(),
            description: item.description.clone(),
            status: item.status.into(),
            icon: (&item.icon).into(),
        }
    }
}

impl From<Status> for signals::TrayItemStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::Unknown => Self::Unknown,
            Status::Passive => Self::Passive,
            Status::Active => Self::Active,
            Status::NeedsAttention => Self::NeedsAttention,
        }
    }
}

impl From<&Icon> for signals::TrayIcon {
    fn from(icon: &Icon) -> Self {
        match icon {
            Icon::None => Self {
                kind: signals::TrayIconKind::None,
                path: None,
                png_bytes: None,
                symbolic: false,
            },
            Icon::Theme { path, symbolic } => Self {
                kind: signals::TrayIconKind::ThemePath,
                path: Some(path.display().to_string()),
                png_bytes: None,
                symbolic: *symbolic,
            },
            Icon::Png { bytes, symbolic } => Self {
                kind: signals::TrayIconKind::PngBytes,
                path: None,
                png_bytes: Some(bytes.clone()),
                symbolic: *symbolic,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        signals,
        tray::{
            Activation, ActivationKind, Icon, Item, ItemId, Menu, MenuActivation, MenuItem,
            MenuItemId, MenuItemKind, Position, Snapshot, Status,
        },
    };

    #[test]
    fn activation_request_projects_to_domain_command() {
        let activation = Activation::from(signals::TrayActivateRequest {
            id: "org.example.Item".to_string(),
            x: 42,
            y: 64,
            kind: signals::TrayActivationKind::Primary,
        });

        assert_eq!(activation.item_id, ItemId::new("org.example.Item"));
        assert_eq!(activation.position, Position::new(42, 64));
        assert_eq!(activation.kind, ActivationKind::Primary);
    }

    #[test]
    fn context_menu_activation_request_projects_to_domain_command() {
        let activation = Activation::from(signals::TrayActivateRequest {
            id: "org.example.Item".to_string(),
            x: 42,
            y: 64,
            kind: signals::TrayActivationKind::ContextMenu,
        });

        assert_eq!(activation.kind, ActivationKind::ContextMenu);
    }

    #[test]
    fn menu_item_activation_request_projects_to_domain_command() {
        let activation = MenuActivation::from(signals::TrayMenuItemActivateRequest {
            item_id: "org.example.Item".to_string(),
            menu_item_id: 7,
        });

        assert_eq!(activation.item_id, ItemId::new("org.example.Item"));
        assert_eq!(activation.menu_item_id, MenuItemId::new(7));
    }

    #[test]
    fn status_projects_to_signal_vocabulary() {
        assert_eq!(
            signals::TrayItemStatus::from(Status::Unknown),
            signals::TrayItemStatus::Unknown
        );
        assert_eq!(
            signals::TrayItemStatus::from(Status::Passive),
            signals::TrayItemStatus::Passive
        );
        assert_eq!(
            signals::TrayItemStatus::from(Status::Active),
            signals::TrayItemStatus::Active
        );
        assert_eq!(
            signals::TrayItemStatus::from(Status::NeedsAttention),
            signals::TrayItemStatus::NeedsAttention
        );
    }

    #[test]
    fn icon_projection_preserves_variants() {
        let none = signals::TrayIcon::from(&Icon::None);
        assert_eq!(none.kind, signals::TrayIconKind::None);
        assert_eq!(none.path, None);
        assert_eq!(none.png_bytes, None);
        assert!(!none.symbolic);

        let theme = signals::TrayIcon::from(&Icon::Theme {
            path: PathBuf::from("/tmp/icon.svg"),
            symbolic: true,
        });
        assert_eq!(theme.kind, signals::TrayIconKind::ThemePath);
        assert_eq!(theme.path.as_deref(), Some("/tmp/icon.svg"));
        assert!(theme.symbolic);

        let png = signals::TrayIcon::from(&Icon::Png {
            bytes: vec![1, 2, 3],
            symbolic: false,
        });
        assert_eq!(png.kind, signals::TrayIconKind::PngBytes);
        assert_eq!(png.png_bytes.as_deref(), Some(&[1, 2, 3][..]));
        assert!(!png.symbolic);
    }

    #[test]
    fn snapshot_projection_preserves_items_and_message() {
        let snapshot = Snapshot {
            items: vec![Item {
                id: ItemId::new("item"),
                title: "Item".to_string(),
                description: Some("Description".to_string()),
                status: Status::Active,
                icon: Icon::None,
            }],
            message: Some("unavailable".to_string()),
        };

        let signal = signals::TrayStatus::from(&snapshot);

        assert_eq!(signal.message.as_deref(), Some("unavailable"));
        assert_eq!(signal.items[0].id, "item");
        assert_eq!(signal.items[0].title, "Item");
        assert_eq!(signal.items[0].description.as_deref(), Some("Description"));
        assert_eq!(signal.items[0].status, signals::TrayItemStatus::Active);
    }

    #[test]
    fn menu_projection_preserves_rows_and_position() {
        let menu = Menu {
            item_id: ItemId::new("item"),
            position: Position::new(42, 64),
            items: vec![
                MenuItem {
                    id: MenuItemId::new(4),
                    label: "About".to_string(),
                    enabled: true,
                    kind: MenuItemKind::Standard,
                    depth: 0,
                },
                MenuItem {
                    id: MenuItemId::new(5),
                    label: String::new(),
                    enabled: false,
                    kind: MenuItemKind::Separator,
                    depth: 1,
                },
            ],
        };

        let signal = signals::TrayMenuStatus::from(&menu);

        assert_eq!(signal.item_id, "item");
        assert_eq!(signal.x, 42);
        assert_eq!(signal.y, 64);
        assert_eq!(signal.items[0].id, 4);
        assert_eq!(signal.items[0].label, "About");
        assert!(signal.items[0].enabled);
        assert_eq!(signal.items[0].kind, signals::TrayMenuItemKind::Standard);
        assert_eq!(signal.items[1].kind, signals::TrayMenuItemKind::Separator);
        assert_eq!(signal.items[1].depth, 1);
    }
}
