//! Shortcut meaning and activation effects.
//!
//! This module owns the closed shortcut vocabulary. Portal identifiers,
//! configured bindings, and runtime events all collapse back into [`Shortcut`]
//! before the rest of the native runtime decides which [`Action`] to perform.

use std::fmt;

use crate::{screenshot, session};

use super::binding::Binding;

/// A shortcut activation emitted by the shortcuts [`super::Registry`].
#[derive(Clone, Debug)]
pub struct Event {
    /// The application shortcut activated by the portal.
    pub shortcut: Shortcut,
}

/// An enabled [`Shortcut`] paired with its configured binding.
///
/// A [`Spec`] is the installable projection of shortcut configuration. It keeps
/// shortcut meaning and trigger data together until the portal and compositor
/// boundaries need their own string formats.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spec {
    shortcut: Shortcut,
    binding: Binding,
}

impl Spec {
    /// Creates an installable [`Spec`] from application meaning and a binding.
    pub(super) fn new(shortcut: Shortcut, binding: Binding) -> Self {
        Self { shortcut, binding }
    }

    /// Returns the application shortcut represented by this spec.
    pub fn shortcut(&self) -> Shortcut {
        self.shortcut
    }

    /// Returns the stable external identifier for this spec.
    pub fn id(&self) -> Id {
        self.shortcut.id()
    }

    /// Returns the portal-facing description for this spec.
    pub fn description(&self) -> &'static str {
        self.shortcut.description()
    }

    /// Returns the configured binding carried by this spec.
    pub(super) fn binding(&self) -> &Binding {
        &self.binding
    }
}

/// The shortcuts Hyprbaric knows how to install and handle.
///
/// A [`Shortcut`] owns stable identity and behavior. User configuration may
/// select a different binding or disable a shortcut, but it does not invent new
/// shortcut meanings at the transport boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shortcut {
    /// Opens or closes the application launcher.
    AppLauncher,
    /// Opens or closes the shared controls widget.
    Controls,
    /// Opens the bar settings surface.
    BarSettings,
    /// Opens or closes the session launcher.
    SessionLauncher,
    /// Locks the current session.
    LockSession,
    /// Starts an interactive region screenshot.
    CaptureRegion,
    /// Captures the currently focused window.
    CaptureWindow,
    /// Captures the full screen.
    CaptureFullScreen,
    /// Starts an interactive screen color pick.
    ColorPick,
    /// Toggles interactive region screen recording.
    ToggleRecording,
    /// Toggles notification do-not-disturb mode.
    ToggleDoNotDisturb,
    /// Toggles the night light filter.
    ToggleNightLight,
    /// Toggles the Caffeine sleep inhibitor.
    ToggleCaffeine,
    /// Raises the output volume.
    VolumeUp,
    /// Lowers the output volume.
    VolumeDown,
    /// Toggles output mute.
    ToggleMute,
    /// Raises display brightness.
    BrightnessUp,
    /// Lowers display brightness.
    BrightnessDown,
}

impl Shortcut {
    /// Every shortcut currently understood by Hyprbaric.
    ///
    /// [`Configuration`](super::Configuration) walks this vocabulary to
    /// preserve default bindings for shortcuts absent from a user override.
    pub const ALL: &'static [Self] = &[
        Self::AppLauncher,
        Self::Controls,
        Self::BarSettings,
        Self::SessionLauncher,
        Self::LockSession,
        Self::CaptureRegion,
        Self::CaptureWindow,
        Self::CaptureFullScreen,
        Self::ColorPick,
        Self::ToggleRecording,
        Self::ToggleDoNotDisturb,
        Self::ToggleNightLight,
        Self::ToggleCaffeine,
        Self::VolumeUp,
        Self::VolumeDown,
        Self::ToggleMute,
        Self::BrightnessUp,
        Self::BrightnessDown,
    ];

    /// Returns the stable identifier owned by this shortcut.
    pub const fn id(self) -> Id {
        match self {
            Self::AppLauncher => Id::new("open_app_launcher"),
            Self::Controls => Id::new("toggle_controls"),
            Self::BarSettings => Id::new("open_bar_settings"),
            Self::SessionLauncher => Id::new("open_session_launcher"),
            Self::LockSession => Id::new("lock_session"),
            Self::CaptureRegion => Id::new("capture_region"),
            Self::CaptureWindow => Id::new("capture_window"),
            Self::CaptureFullScreen => Id::new("capture_full_screen"),
            Self::ColorPick => Id::new("color_pick"),
            Self::ToggleRecording => Id::new("toggle_recording"),
            Self::ToggleDoNotDisturb => Id::new("toggle_do_not_disturb"),
            Self::ToggleNightLight => Id::new("toggle_night_light"),
            Self::ToggleCaffeine => Id::new("toggle_caffeine"),
            Self::VolumeUp => Id::new("volume_up"),
            Self::VolumeDown => Id::new("volume_down"),
            Self::ToggleMute => Id::new("toggle_mute"),
            Self::BrightnessUp => Id::new("brightness_up"),
            Self::BrightnessDown => Id::new("brightness_down"),
        }
    }

    /// Returns the serde field name used under the top-level `shortcuts` table.
    ///
    /// This is deliberately distinct from [`Self::id`]. Portal identifiers
    /// describe runtime effects, while config keys must stay aligned with
    /// [`super::Configuration`]'s serde field names.
    pub const fn config_key(self) -> &'static str {
        match self {
            Self::AppLauncher => "app_launcher",
            Self::Controls => "controls",
            Self::BarSettings => "bar_settings",
            Self::SessionLauncher => "session_launcher",
            Self::LockSession => "lock_session",
            Self::CaptureRegion => "capture_region",
            Self::CaptureWindow => "capture_window",
            Self::CaptureFullScreen => "capture_full_screen",
            Self::ColorPick => "color_pick",
            Self::ToggleRecording => "toggle_recording",
            Self::ToggleDoNotDisturb => "toggle_do_not_disturb",
            Self::ToggleNightLight => "toggle_night_light",
            Self::ToggleCaffeine => "toggle_caffeine",
            Self::VolumeUp => "volume_up",
            Self::VolumeDown => "volume_down",
            Self::ToggleMute => "toggle_mute",
            Self::BrightnessUp => "brightness_up",
            Self::BrightnessDown => "brightness_down",
        }
    }

    /// Parses a portal identifier back into Hyprbaric's shortcut vocabulary.
    ///
    /// Identifiers outside [`Id::DOMAIN`] are intentionally ignored.
    pub fn from_portal_id(portal_id: &str) -> Option<Self> {
        let portal_id = portal_id
            .rsplit_once(':')
            .map_or(portal_id, |(_, shortcut_id)| shortcut_id);
        let suffix = portal_id.strip_prefix(Id::DOMAIN)?.strip_prefix('.')?;
        match suffix {
            "open_app_launcher" => Some(Self::AppLauncher),
            "toggle_controls" => Some(Self::Controls),
            "open_bar_settings" => Some(Self::BarSettings),
            "open_session_launcher" => Some(Self::SessionLauncher),
            "lock_session" => Some(Self::LockSession),
            "capture_region" => Some(Self::CaptureRegion),
            "capture_window" => Some(Self::CaptureWindow),
            "capture_full_screen" => Some(Self::CaptureFullScreen),
            "color_pick" => Some(Self::ColorPick),
            "toggle_recording" => Some(Self::ToggleRecording),
            "toggle_do_not_disturb" => Some(Self::ToggleDoNotDisturb),
            "toggle_night_light" => Some(Self::ToggleNightLight),
            "toggle_caffeine" => Some(Self::ToggleCaffeine),
            "volume_up" => Some(Self::VolumeUp),
            "volume_down" => Some(Self::VolumeDown),
            "toggle_mute" => Some(Self::ToggleMute),
            "brightness_up" => Some(Self::BrightnessUp),
            "brightness_down" => Some(Self::BrightnessDown),
            _ => None,
        }
    }

    /// Returns the description advertised to the shortcuts portal.
    pub const fn description(self) -> &'static str {
        match self {
            Self::AppLauncher => "Open the Hyprbaric app launcher",
            Self::Controls => "Toggle Hyprbaric controls",
            Self::BarSettings => "Open Hyprbaric bar settings",
            Self::SessionLauncher => "Open the Hyprbaric session launcher",
            Self::LockSession => "Lock the current session",
            Self::CaptureRegion => "Capture a screenshot region",
            Self::CaptureWindow => "Capture the focused window",
            Self::CaptureFullScreen => "Capture the full screen",
            Self::ColorPick => "Pick a screen color",
            Self::ToggleRecording => "Toggle screen recording",
            Self::ToggleDoNotDisturb => "Toggle notification do-not-disturb mode",
            Self::ToggleNightLight => "Toggle Night light",
            Self::ToggleCaffeine => "Toggle Caffeine mode",
            Self::VolumeUp => "Raise output volume",
            Self::VolumeDown => "Lower output volume",
            Self::ToggleMute => "Toggle output mute",
            Self::BrightnessUp => "Raise display brightness",
            Self::BrightnessDown => "Lower display brightness",
        }
    }

    /// Returns the user-facing label used by settings surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AppLauncher => "App launcher",
            Self::Controls => "Controls",
            Self::BarSettings => "Bar settings",
            Self::SessionLauncher => "Session launcher",
            Self::LockSession => "Lock session",
            Self::CaptureRegion => "Screenshot region",
            Self::CaptureWindow => "Screenshot window",
            Self::CaptureFullScreen => "Screenshot full screen",
            Self::ColorPick => "Color picker",
            Self::ToggleRecording => "Toggle recording",
            Self::ToggleDoNotDisturb => "Toggle DND",
            Self::ToggleNightLight => "Toggle Night light",
            Self::ToggleCaffeine => "Toggle Caffeine",
            Self::VolumeUp => "Volume up",
            Self::VolumeDown => "Volume down",
            Self::ToggleMute => "Toggle mute",
            Self::BrightnessUp => "Brightness up",
            Self::BrightnessDown => "Brightness down",
        }
    }

    /// Returns the settings category for this shortcut.
    pub const fn category(self) -> Category {
        match self {
            Self::AppLauncher | Self::Controls | Self::BarSettings => Category::Bar,
            Self::ToggleDoNotDisturb => Category::Bar,
            Self::SessionLauncher | Self::LockSession | Self::ToggleCaffeine => Category::Session,
            Self::CaptureRegion
            | Self::CaptureWindow
            | Self::CaptureFullScreen
            | Self::ColorPick
            | Self::ToggleRecording => Category::Capture,
            Self::VolumeUp | Self::VolumeDown | Self::ToggleMute => Category::Audio,
            Self::BrightnessUp | Self::BrightnessDown | Self::ToggleNightLight => Category::Display,
        }
    }

    /// Returns the native effect produced by this shortcut.
    pub const fn action(self) -> Action {
        match self {
            Self::AppLauncher => Action::Ui(UiAction::ToggleAppLauncher),
            Self::Controls => Action::Ui(UiAction::ToggleControls),
            Self::BarSettings => Action::Ui(UiAction::OpenBarSettings),
            Self::SessionLauncher => Action::Ui(UiAction::ToggleSessionLauncher),
            Self::LockSession => Action::Session(session::Action::Lock),
            Self::CaptureRegion => Action::Screenshot(screenshot::Mode::Region),
            Self::CaptureWindow => Action::Screenshot(screenshot::Mode::Window),
            Self::CaptureFullScreen => Action::Screenshot(screenshot::Mode::FullScreen),
            Self::ColorPick => Action::Ui(UiAction::ColorPick),
            Self::ToggleRecording => Action::Ui(UiAction::ToggleRecording),
            Self::ToggleDoNotDisturb => Action::Ui(UiAction::ToggleDoNotDisturb),
            Self::ToggleNightLight => Action::Ui(UiAction::ToggleNightLight),
            Self::ToggleCaffeine => Action::Ui(UiAction::ToggleCaffeine),
            Self::VolumeUp => Action::Ui(UiAction::VolumeUp),
            Self::VolumeDown => Action::Ui(UiAction::VolumeDown),
            Self::ToggleMute => Action::Ui(UiAction::ToggleMute),
            Self::BrightnessUp => Action::Ui(UiAction::BrightnessUp),
            Self::BrightnessDown => Action::Ui(UiAction::BrightnessDown),
        }
    }
}

/// Shortcut grouping used by the settings panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Category {
    /// Bar-owned popup and modal actions.
    Bar,
    /// Session and power-flow actions.
    Session,
    /// Capture actions.
    Capture,
    /// Audio controls.
    Audio,
    /// Display controls.
    Display,
}

impl fmt::Display for Shortcut {
    /// Formats this shortcut as its fully-qualified portal identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id().portal_id())
    }
}

/// The effect produced by an activated [`Shortcut`].
///
/// UI actions cross back into Flutter. Session and screenshot actions stay in
/// the Rust runtime, where their platform dependencies already live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// A Flutter-owned bar action.
    Ui(UiAction),
    /// A native session action.
    Session(session::Action),
    /// A native screenshot capture action.
    Screenshot(screenshot::Mode),
}

/// A shortcut effect handled by the Flutter bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiAction {
    /// Opens or closes the app launcher.
    ToggleAppLauncher,
    /// Opens or closes the controls widget.
    ToggleControls,
    /// Opens the bar settings surface.
    OpenBarSettings,
    /// Opens or closes the session launcher.
    ToggleSessionLauncher,
    /// Starts an interactive screen color pick through Flutter chrome cleanup.
    ColorPick,
    /// Toggles interactive screen recording through Flutter chrome cleanup.
    ToggleRecording,
    /// Toggles notification do-not-disturb mode through Flutter state.
    ToggleDoNotDisturb,
    /// Toggles Night light through Flutter state.
    ToggleNightLight,
    /// Toggles Caffeine mode through Flutter state.
    ToggleCaffeine,
    /// Raises the output volume through Flutter's audio controller.
    VolumeUp,
    /// Lowers the output volume through Flutter's audio controller.
    VolumeDown,
    /// Toggles output mute through Flutter's audio controller.
    ToggleMute,
    /// Raises display brightness through Flutter's audio/display controller.
    BrightnessUp,
    /// Lowers display brightness through Flutter's audio/display controller.
    BrightnessDown,
}

/// A stable shortcut identifier with the Hyprbaric portal namespace attached.
///
/// The suffix stays small and typed beside [`Shortcut`]. The fully qualified
/// identifier is only materialized at portal and Hyprland boundaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(&'static str);

impl Id {
    /// The reverse-domain namespace reserved for Hyprbaric shortcuts.
    pub const DOMAIN: &'static str = "com.shortcut.hyprbaric";

    /// Creates an identifier from the suffix owned by a [`Shortcut`].
    const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Formats this identifier for the global shortcuts portal.
    pub fn portal_id(self) -> String {
        format!("{}.{}", Self::DOMAIN, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Shortcut;

    #[test]
    fn portal_ids_round_trip_to_shortcuts() {
        for shortcut in Shortcut::ALL {
            let portal_id = shortcut.id().portal_id();
            assert_eq!(Shortcut::from_portal_id(&portal_id), Some(*shortcut));
        }
    }

    #[test]
    fn app_scoped_portal_ids_round_trip_to_shortcuts() {
        for shortcut in Shortcut::ALL {
            let portal_id = format!("com.hyprbaric.Hyprbaric:{}", shortcut.id().portal_id());
            assert_eq!(Shortcut::from_portal_id(&portal_id), Some(*shortcut));
        }
    }
}
