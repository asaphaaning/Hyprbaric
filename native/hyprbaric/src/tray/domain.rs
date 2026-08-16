//! Tray domain vocabulary.

use std::{collections::HashSet, fmt, path::PathBuf};

/// UI-facing tray snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// Visible tray items in stable display order.
    pub items: Vec<Item>,
    /// Optional unavailable or failure copy.
    pub message: Option<String>,
}

/// One visible tray item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    /// Stable item address from the tray backend.
    pub id: ItemId,
    /// Display title.
    pub title: String,
    /// Optional tooltip description.
    pub description: Option<String>,
    /// Notifier status.
    pub status: Status,
    /// Resolved icon payload.
    pub icon: Icon,
}

/// A tray activation command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activation {
    /// Item to activate.
    pub item_id: ItemId,
    /// Screen-space click position.
    pub position: Position,
    /// Requested activation behavior.
    pub kind: ActivationKind,
}

/// Tray item activation behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ActivationKind {
    /// Primary click/default behavior.
    #[default]
    Primary,
    /// Context menu/right-click behavior.
    ContextMenu,
}

/// A tray menu item activation command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuActivation {
    /// Item that owns the menu.
    pub item_id: ItemId,
    /// DBusMenu row to activate.
    pub menu_item_id: MenuItemId,
}

/// A visible tray menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Menu {
    /// Item that owns this menu.
    pub item_id: ItemId,
    /// Screen-space position that requested the menu.
    pub position: Position,
    /// Menu rows, flattened for the UI.
    pub items: Vec<MenuItem>,
}

/// Successful tray command outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Activated,
    Menu(Menu),
}

/// One visible tray menu row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItem {
    /// DBusMenu item identifier.
    pub id: MenuItemId,
    /// Row text.
    pub label: String,
    /// Whether clicking the row is allowed.
    pub enabled: bool,
    /// Row kind.
    pub kind: MenuItemKind,
    /// Indentation depth for nested menus.
    pub depth: u8,
}

/// A tray item identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(String);

/// A DBusMenu row identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MenuItemId(i32);

/// A screen-space click position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

/// A resolved tray icon.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    /// No icon was available.
    None,
    /// A theme icon resolved to a filesystem path.
    Theme { path: PathBuf, symbolic: bool },
    /// Pixmap bytes encoded as PNG.
    Png { bytes: Vec<u8>, symbolic: bool },
}

/// StatusNotifier item status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Status {
    /// Unknown or missing status.
    #[default]
    Unknown,
    /// The item is passive.
    Passive,
    /// The item is active.
    Active,
    /// The item needs attention.
    NeedsAttention,
}

/// Tray menu row kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MenuItemKind {
    /// A clickable row.
    #[default]
    Standard,
    /// A visual separator.
    Separator,
}

/// Stable tray item order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Order {
    ids: Vec<ItemId>,
}

/// One ordering event from the watcher boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum OrderEvent {
    /// A new item appeared.
    Add(ItemId),
    /// An item disappeared.
    Remove(ItemId),
    /// An existing item changed without changing order.
    Update,
}

impl Snapshot {
    /// Creates an unavailable tray snapshot.
    pub(super) fn unavailable(message: impl Into<String>) -> Self {
        Self {
            items: Vec::new(),
            message: Some(message.into()),
        }
    }
}

impl ItemId {
    /// Creates a tray item ID.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the raw item address.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl MenuItemId {
    /// Creates a menu item ID.
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the raw DBusMenu row ID.
    pub const fn get(self) -> i32 {
        self.0
    }
}

impl Position {
    /// Creates a screen-space position.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the x coordinate.
    pub const fn x(self) -> i32 {
        self.x
    }

    /// Returns the y coordinate.
    pub const fn y(self) -> i32 {
        self.y
    }
}

impl Order {
    /// Creates an order from a sorted address list.
    pub(super) fn sorted(mut ids: Vec<ItemId>) -> Self {
        ids.sort_by(|left, right| left.as_str().cmp(right.as_str()));
        Self { ids }
    }

    /// Returns ordered addresses.
    pub(super) fn as_slice(&self) -> &[ItemId] {
        &self.ids
    }

    /// Applies an incremental order event.
    pub(super) fn apply(&mut self, event: OrderEvent) {
        match event {
            OrderEvent::Add(id) => {
                if !self.ids.contains(&id) {
                    self.ids.push(id);
                }
            }
            OrderEvent::Remove(id) => self.ids.retain(|value| value != &id),
            OrderEvent::Update => {}
        }
    }

    /// Reconciles order with a current address set.
    pub(super) fn sync(&mut self, mut ids: Vec<ItemId>) {
        let known = ids.iter().cloned().collect::<HashSet<_>>();
        self.ids.retain(|value| known.contains(value));
        ids.sort_by(|left, right| left.as_str().cmp(right.as_str()));
        for id in ids {
            if !self.ids.contains(&id) {
                self.ids.push(id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ItemId, Order, OrderEvent};

    #[test]
    fn order_retains_known_items_and_appends_new_ones() {
        let mut order = Order::sorted(vec![ItemId::new("b"), ItemId::new("a")]);
        order.sync(vec![ItemId::new("c"), ItemId::new("a"), ItemId::new("b")]);
        assert_eq!(
            order
                .as_slice()
                .iter()
                .map(ItemId::as_str)
                .collect::<Vec<_>>(),
            ["a", "b", "c"]
        );
    }

    #[test]
    fn order_applies_add_remove_and_update_events() {
        let mut order = Order::sorted(vec![ItemId::new("b"), ItemId::new("a")]);
        order.apply(OrderEvent::Add(ItemId::new("c")));
        order.apply(OrderEvent::Add(ItemId::new("a")));
        order.apply(OrderEvent::Update);
        order.apply(OrderEvent::Remove(ItemId::new("b")));
        assert_eq!(
            order
                .as_slice()
                .iter()
                .map(ItemId::as_str)
                .collect::<Vec<_>>(),
            ["a", "c"]
        );
    }
}
