//! Shortcut binding configuration and compositor projections.
//!
//! [`Configuration`] is where trigger choice lives. It starts with complete
//! defaults, accepts partial serde overrides, and turns enabled mappings into [`Spec`]
//! values for the shortcut registry.

use serde::{Deserialize, Deserializer, de};

use super::{
    domain::{Shortcut, Spec},
    hyprland,
};

/// User-configurable shortcut mappings with a complete default set.
///
/// A default [`Configuration`] can be installed with no configuration file.
/// Serde overrides change only the selected bindings while [`Shortcut`]
/// continues to own meaning, identity, and behavior.
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    app_launcher: Mapping,
    controls: Mapping,
    bar_settings: Mapping,
    session_launcher: Mapping,
    lock_session: Mapping,
    capture_region: Mapping,
    capture_window: Mapping,
    capture_full_screen: Mapping,
    color_pick: Mapping,
    toggle_recording: Mapping,
    toggle_do_not_disturb: Mapping,
    toggle_night_light: Mapping,
    toggle_caffeine: Mapping,
    volume_up: Mapping,
    volume_down: Mapping,
    toggle_mute: Mapping,
    brightness_up: Mapping,
    brightness_down: Mapping,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            app_launcher: Mapping::default_for(Shortcut::AppLauncher),
            controls: Mapping::default_for(Shortcut::Controls),
            bar_settings: Mapping::default_for(Shortcut::BarSettings),
            session_launcher: Mapping::default_for(Shortcut::SessionLauncher),
            lock_session: Mapping::default_for(Shortcut::LockSession),
            capture_region: Mapping::default_for(Shortcut::CaptureRegion),
            capture_window: Mapping::default_for(Shortcut::CaptureWindow),
            capture_full_screen: Mapping::default_for(Shortcut::CaptureFullScreen),
            color_pick: Mapping::default_for(Shortcut::ColorPick),
            toggle_recording: Mapping::default_for(Shortcut::ToggleRecording),
            toggle_do_not_disturb: Mapping::default_for(Shortcut::ToggleDoNotDisturb),
            toggle_night_light: Mapping::default_for(Shortcut::ToggleNightLight),
            toggle_caffeine: Mapping::default_for(Shortcut::ToggleCaffeine),
            volume_up: Mapping::default_for(Shortcut::VolumeUp),
            volume_down: Mapping::default_for(Shortcut::VolumeDown),
            toggle_mute: Mapping::default_for(Shortcut::ToggleMute),
            brightness_up: Mapping::default_for(Shortcut::BrightnessUp),
            brightness_down: Mapping::default_for(Shortcut::BrightnessDown),
        }
    }
}

impl Configuration {
    /// Returns the enabled shortcut mappings as installable [`Spec`] values.
    pub fn specs(&self) -> impl Iterator<Item = Spec> + '_ {
        Shortcut::ALL
            .into_iter()
            .filter_map(|shortcut| self.mapping(*shortcut).spec(*shortcut))
    }

    /// Returns the configured mapping for a shortcut.
    ///
    /// Each [`Shortcut`] has exactly one slot in [`Configuration`]. The private
    /// lookup keeps that exhaustiveness beside the closed shortcut vocabulary
    /// instead of making callers coordinate field names.
    pub(crate) fn mapping(&self, shortcut: Shortcut) -> &Mapping {
        match shortcut {
            Shortcut::AppLauncher => &self.app_launcher,
            Shortcut::Controls => &self.controls,
            Shortcut::BarSettings => &self.bar_settings,
            Shortcut::SessionLauncher => &self.session_launcher,
            Shortcut::LockSession => &self.lock_session,
            Shortcut::CaptureRegion => &self.capture_region,
            Shortcut::CaptureWindow => &self.capture_window,
            Shortcut::CaptureFullScreen => &self.capture_full_screen,
            Shortcut::ColorPick => &self.color_pick,
            Shortcut::ToggleRecording => &self.toggle_recording,
            Shortcut::ToggleDoNotDisturb => &self.toggle_do_not_disturb,
            Shortcut::ToggleNightLight => &self.toggle_night_light,
            Shortcut::ToggleCaffeine => &self.toggle_caffeine,
            Shortcut::VolumeUp => &self.volume_up,
            Shortcut::VolumeDown => &self.volume_down,
            Shortcut::ToggleMute => &self.toggle_mute,
            Shortcut::BrightnessUp => &self.brightness_up,
            Shortcut::BrightnessDown => &self.brightness_down,
        }
    }
}

/// The configured state for one [`Shortcut`] mapping.
///
/// A binding-shaped mapping implies [`Mapping::Bound`]. The only state users
/// need to spell explicitly is [`Mapping::Disabled`].
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub(crate) enum Mapping {
    /// The shortcut should be installed with the configured binding.
    Bound(Binding),
    /// The shortcut stays in the application vocabulary but is not installed.
    Disabled(Disabled),
}

impl Mapping {
    /// Creates a bound mapping for one of the built-in defaults.
    fn bound(binding: Binding) -> Self {
        Self::Bound(binding)
    }

    /// Returns the built-in mapping for a shortcut.
    pub(crate) fn default_for(shortcut: Shortcut) -> Self {
        Self::bound(Binding::default_for(shortcut))
    }

    /// Projects this mapping into an installable [`Spec`], if it is enabled.
    pub(crate) fn spec(&self, shortcut: Shortcut) -> Option<Spec> {
        match self {
            Self::Bound(binding) => Some(Spec::new(shortcut, binding.clone())),
            Self::Disabled(_) => None,
        }
    }

    /// Returns the bound trigger, if this mapping is enabled.
    pub(crate) fn binding(&self) -> Option<&Binding> {
        match self {
            Self::Bound(binding) => Some(binding),
            Self::Disabled(_) => None,
        }
    }

    /// Returns whether this mapping disables its shortcut.
    pub(crate) const fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled(_))
    }
}

/// The explicit opt-out state for a shortcut mapping.
///
/// Bound mappings are represented directly by [`Binding`]. This marker keeps
/// the disabled config form narrow:
///
/// ```toml
/// [shortcuts.controls]
/// state = "disabled"
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct Disabled {
    /// The explicit disabled state requested by user configuration.
    pub(crate) state: DisabledState,
}

/// The only explicit state currently carried by [`Disabled`].
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DisabledState {
    /// The shortcut should not be installed.
    Disabled,
}

/// A configured trigger chord and activation phase.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct Binding {
    #[serde(default)]
    phase: Phase,
    #[serde(flatten)]
    chord: Chord,
}

impl Binding {
    /// Creates the ordinary press binding used by default shortcuts.
    fn press(chord: Chord) -> Self {
        Self {
            phase: Phase::Press,
            chord,
        }
    }

    /// Creates a release binding used for modifier-only default shortcuts.
    fn release(chord: Chord) -> Self {
        Self {
            phase: Phase::Release,
            chord,
        }
    }

    /// Returns the default binding for a shortcut.
    pub(crate) fn default_for(shortcut: Shortcut) -> Self {
        match shortcut {
            Shortcut::AppLauncher => Self::release(Chord::logo("Super_L")),
            Shortcut::Controls => Self::press(Chord::logo("S")),
            Shortcut::BarSettings => Self::press(Chord::logo_shift("C")),
            Shortcut::SessionLauncher => Self::press(Chord::logo("ESCAPE")),
            Shortcut::LockSession => Self::press(Chord::logo("L")),
            Shortcut::CaptureRegion => Self::press(Chord::logo_shift("S")),
            Shortcut::CaptureWindow => Self::press(Chord::logo("Print")),
            Shortcut::CaptureFullScreen => Self::press(Chord::key("Print")),
            Shortcut::ColorPick => Self::press(Chord::logo_shift("P")),
            Shortcut::ToggleRecording => Self::press(Chord::logo_shift("R")),
            Shortcut::ToggleDoNotDisturb => Self::press(Chord::logo_shift("D")),
            Shortcut::ToggleNightLight => Self::press(Chord::logo_shift("N")),
            Shortcut::ToggleCaffeine => Self::press(Chord::logo_shift("F12")),
            Shortcut::VolumeUp => Self::press(Chord::key("XF86AudioRaiseVolume")),
            Shortcut::VolumeDown => Self::press(Chord::key("XF86AudioLowerVolume")),
            Shortcut::ToggleMute => Self::press(Chord::key("XF86AudioMute")),
            Shortcut::BrightnessUp => Self::press(Chord::key("XF86MonBrightnessUp")),
            Shortcut::BrightnessDown => Self::press(Chord::key("XF86MonBrightnessDown")),
        }
    }

    /// Creates a user-selected binding from already typed pieces.
    pub(crate) fn from_parts(
        phase: Phase,
        modifiers: impl IntoIterator<Item = Modifier>,
        key: &str,
    ) -> Result<Self, InvalidBinding> {
        Ok(Self {
            phase,
            chord: Chord {
                modifiers: modifiers
                    .into_iter()
                    .fold(Modifiers::default(), Modifiers::with),
                key: Key::parse(key)?,
            },
        })
    }

    /// Formats this binding for the desktop portal trigger vocabulary.
    pub(super) fn portal_trigger(&self) -> String {
        self.chord.portal_trigger()
    }

    /// Formats this binding as a Hyprland `global` bind.
    ///
    /// Hyprland wants a dispatcher argument prefixed for the `global`
    /// dispatcher, while [`Binding`] only owns key and phase information.
    pub(super) fn hypr_bind(&self, global_id: &str) -> hyprland::Bind {
        hyprland::Bind {
            keyword: self.phase.keyword(),
            value: self.chord.hypr_bind_value("global", global_id),
        }
    }

    /// Formats this binding as a Hyprland `exec` bind.
    pub(super) fn hypr_exec_bind(&self, command: &str) -> hyprland::Bind {
        hyprland::Bind {
            keyword: self.phase.keyword(),
            value: self.chord.hypr_bind_value("exec", command),
        }
    }

    /// Returns the Hyprland chord used to compare compositor bind state.
    pub(super) fn hypr_chord(&self) -> hyprland::Chord {
        self.chord.hypr_chord()
    }

    /// Returns the bind phase used by compositor reconciliation.
    pub(super) fn phase(&self) -> Phase {
        self.phase
    }

    /// Returns the user-facing trigger string.
    pub(crate) fn display(&self) -> String {
        self.chord.display()
    }

    /// Returns the configured phase.
    pub(crate) fn configured_phase(&self) -> Phase {
        self.phase
    }

    /// Returns modifiers in stable settings-display order.
    pub(crate) fn modifiers(&self) -> Vec<Modifier> {
        self.chord.modifiers.ordered()
    }

    /// Returns the normalized key label.
    pub(crate) fn key(&self) -> String {
        self.chord.key.label()
    }

    /// Returns whether this binding models the standalone logo key.
    pub(super) fn is_standalone_logo_key(&self) -> bool {
        self.chord.is_standalone_logo_key()
    }
}

/// The Hyprland bind phase for a [`Binding`].
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Phase {
    /// Activates when a chord is pressed.
    #[default]
    Press,
    /// Activates when a chord is released.
    Release,
}

impl Phase {
    /// Returns the Hyprland bind keyword for this phase.
    fn keyword(self) -> &'static str {
        match self {
            Self::Press => "bind",
            Self::Release => "bindr",
        }
    }

    /// Rebuilds a phase from Hyprland's `release` field.
    pub(super) fn from_release(release: bool) -> Self {
        if release { Self::Release } else { Self::Press }
    }
}

/// A shortcut key with its modifier set.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct Chord {
    #[serde(default)]
    modifiers: Modifiers,
    key: Key,
}

impl Chord {
    /// Creates a key-only chord for hardware utility defaults.
    fn key(key: &str) -> Self {
        Self {
            modifiers: Modifiers::default(),
            key: Key::named(key),
        }
    }

    /// Creates a logo chord for default configuration.
    fn logo(key: &str) -> Self {
        Self {
            modifiers: Modifiers::from([Modifier::Logo]),
            key: Key::named(key),
        }
    }

    /// Creates a logo-shift chord for default configuration.
    fn logo_shift(key: &str) -> Self {
        Self {
            modifiers: Modifiers::from([Modifier::Logo, Modifier::Shift]),
            key: Key::named(key),
        }
    }

    /// Formats this chord for a portal preferred trigger.
    fn portal_trigger(&self) -> String {
        let mut parts = self.modifiers.labels(LabelStyle::Portal);
        parts.push(self.key.label());
        parts.join("+")
    }

    /// Formats this chord for user-facing settings and status surfaces.
    fn display(&self) -> String {
        if self.is_standalone_logo_key() {
            return "Super".to_string();
        }

        self.portal_trigger()
    }

    /// Returns whether this chord models the standalone logo key.
    fn is_standalone_logo_key(&self) -> bool {
        self.modifiers == Modifiers::from([Modifier::Logo]) && self.key.label() == "Super_L"
    }

    /// Projects this chord into the shape reported by Hyprland.
    fn hypr_chord(&self) -> hyprland::Chord {
        hyprland::Chord {
            modmask: self.modifiers.mask(),
            key: self.key.label(),
        }
    }

    /// Formats this chord for a Hyprland bind value.
    fn hypr_bind_value(&self, dispatcher: &str, argument: &str) -> String {
        let mods = self.modifiers.labels(LabelStyle::Hyprland).join(" ");
        if mods.is_empty() {
            format!(", {}, {dispatcher}, {argument}", self.key.label())
        } else {
            format!("{mods}, {}, {dispatcher}, {argument}", self.key.label())
        }
    }
}

/// The modifier bits carried by a configured [`Chord`].
///
/// Internally the set is compact. Boundaries ask for ordered labels or the
/// Hyprland mask instead of depending on serde's array representation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Modifiers(u8);

impl<const N: usize> From<[Modifier; N]> for Modifiers {
    /// Builds a modifier set from distinct default configuration pieces.
    fn from(value: [Modifier; N]) -> Self {
        value.into_iter().fold(Self::default(), Self::with)
    }
}

impl Modifiers {
    /// Returns a copy with the modifier bit enabled.
    pub(crate) fn with(mut self, modifier: Modifier) -> Self {
        self.0 |= modifier.mask();
        self
    }

    /// Returns the bitmask used by Hyprland bind snapshots.
    fn mask(&self) -> u32 {
        self.0.into()
    }

    /// Returns stable modifier labels for a boundary syntax.
    fn labels(&self, style: LabelStyle) -> Vec<String> {
        Modifier::ORDERED
            .into_iter()
            .filter(|modifier| self.0 & modifier.mask() != 0)
            .map(|modifier| modifier.label(style).to_string())
            .collect()
    }

    /// Returns typed modifiers in stable order.
    fn ordered(&self) -> Vec<Modifier> {
        Modifier::ORDERED
            .into_iter()
            .filter(|modifier| self.0 & modifier.mask() != 0)
            .collect()
    }
}

impl<'de> Deserialize<'de> for Modifiers {
    /// Deserializes user modifier arrays into the compact set representation.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<Modifier>::deserialize(deserializer)?
            .into_iter()
            .fold(Self::default(), Self::with))
    }
}

/// A supported shortcut modifier.
///
/// The serde aliases keep user configuration familiar, while rendering still
/// chooses the spelling required by a specific [`LabelStyle`].
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Modifier {
    /// The compositor logo key, also accepted as `super` and `meta`.
    #[serde(alias = "super", alias = "meta")]
    Logo,
    /// The control key, also accepted as `control`.
    #[serde(alias = "control")]
    Ctrl,
    /// The shift key.
    Shift,
    /// The alt key.
    Alt,
    /// The num-lock modifier.
    Num,
}

impl Modifier {
    /// Stable label order used when formatting chords.
    pub(crate) const ORDERED: [Self; 5] =
        [Self::Logo, Self::Ctrl, Self::Shift, Self::Alt, Self::Num];

    /// Returns the union of modifier bits Hyprbaric can format.
    fn supported_mask() -> u32 {
        Self::ORDERED
            .into_iter()
            .fold(0, |mask, modifier| mask | u32::from(modifier.mask()))
    }

    /// Returns the Hyprland bit associated with this modifier.
    const fn mask(self) -> u8 {
        match self {
            Self::Logo => 64,
            Self::Ctrl => 4,
            Self::Shift => 1,
            Self::Alt => 8,
            Self::Num => 16,
        }
    }

    /// Returns the label expected by one boundary syntax.
    const fn label(self, style: LabelStyle) -> &'static str {
        match (self, style) {
            (Self::Logo, LabelStyle::Portal) => "LOGO",
            (Self::Logo, LabelStyle::Hyprland) => "SUPER",
            (Self::Ctrl, _) => "CTRL",
            (Self::Shift, _) => "SHIFT",
            (Self::Alt, _) => "ALT",
            (Self::Num, _) => "NUM",
        }
    }
}

/// The modifier spelling required by a shortcut boundary.
#[derive(Clone, Copy)]
enum LabelStyle {
    /// Portal trigger spelling.
    Portal,
    /// Hyprland bind spelling.
    Hyprland,
}

/// A normalized key label for a [`Chord`].
///
/// Keys enter through user configuration or default constructors. This wrapper
/// rejects separators and normalizes case before a key can leak into portal or
/// Hyprland command strings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Key(String);

impl Key {
    /// Normalizes a known-good key label.
    fn named(value: &str) -> Self {
        Self(normalize_key_label(value))
    }

    /// Parses and validates a key label from UI or configuration input.
    fn parse(value: &str) -> Result<Self, InvalidBinding> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(InvalidBinding::EmptyKey);
        }
        if trimmed.contains([',', '+', '<', '>']) {
            return Err(InvalidBinding::SeparatorInKey);
        }
        Ok(Self::named(trimmed))
    }

    /// Returns the normalized label for boundary formatting.
    fn label(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvalidBinding {
    #[error("shortcut key cannot be empty")]
    EmptyKey,
    #[error("shortcut key cannot contain bind separators")]
    SeparatorInKey,
}

pub(super) fn normalize_key_label(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with("XF86") {
        trimmed.to_owned()
    } else if trimmed.eq_ignore_ascii_case("Super_L") {
        "Super_L".to_string()
    } else if trimmed.eq_ignore_ascii_case("Super_R") {
        "Super_R".to_string()
    } else if trimmed.eq_ignore_ascii_case("Print") || trimmed.eq_ignore_ascii_case("PrintScreen") {
        "Print".to_string()
    } else {
        trimmed.to_ascii_uppercase()
    }
}

impl<'de> Deserialize<'de> for Key {
    /// Deserializes and validates a user key label.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Key::parse(&value).map_err(de::Error::custom)
    }
}

/// Formats a Hyprland modifier mask for an `unbind` command.
///
/// Unknown bits are rejected so reconciliation never guesses at a chord owned
/// by another compositor binding.
pub(super) fn format_hyprland_modmask(modmask: u32) -> Option<String> {
    if modmask & !Modifier::supported_mask() != 0 {
        return None;
    }

    let modifiers = Modifiers(u8::try_from(modmask).ok()?);
    Some(modifiers.labels(LabelStyle::Hyprland).join(" "))
}

#[cfg(test)]
mod tests {
    use super::{Binding, Chord, Configuration, Disabled, DisabledState, Mapping, Phase};
    use crate::shortcuts::Shortcut;

    #[test]
    fn defaults_emit_a_bound_spec_for_every_shortcut() {
        assert_eq!(
            Configuration::default().specs().count(),
            Shortcut::ALL.len()
        );
    }

    #[test]
    fn partial_config_override_keeps_default_shortcuts() {
        let config = toml::from_str::<Configuration>(
            r#"
            [controls]
            state = "disabled"

            [capture_region]
            phase = "release"
            modifiers = ["logo", "shift"]
            key = "r"
            "#,
        )
        .expect("shortcut config should deserialize");
        let capture_region = config
            .mapping(Shortcut::CaptureRegion)
            .spec(Shortcut::CaptureRegion)
            .expect("overridden capture region should remain bound");

        assert_eq!(config.specs().count(), Shortcut::ALL.len() - 1);
        assert_eq!(capture_region.binding().phase, Phase::Release);
        assert_eq!(capture_region.binding().portal_trigger(), "LOGO+SHIFT+R");
        assert!(
            config
                .specs()
                .any(|spec| spec.shortcut() == Shortcut::AppLauncher)
        );
    }

    #[test]
    fn disabled_mapping_omits_a_spec() {
        let config = Configuration {
            controls: Mapping::Disabled(Disabled {
                state: DisabledState::Disabled,
            }),
            ..Configuration::default()
        };

        assert!(
            !config
                .specs()
                .any(|spec| spec.shortcut() == Shortcut::Controls)
        );
    }

    #[test]
    fn defaults_include_hardware_utility_keys() {
        let config = Configuration::default();
        let specs = config.specs().collect::<Vec<_>>();

        let volume_up = specs
            .iter()
            .find(|spec| spec.shortcut() == Shortcut::VolumeUp)
            .expect("volume up should be installed by default");
        let brightness_down = specs
            .iter()
            .find(|spec| spec.shortcut() == Shortcut::BrightnessDown)
            .expect("brightness down should be installed by default");

        assert_eq!(volume_up.binding().portal_trigger(), "XF86AudioRaiseVolume");
        assert_eq!(
            volume_up.binding().hypr_bind(":volume").value,
            ", XF86AudioRaiseVolume, global, :volume"
        );
        assert_eq!(
            brightness_down.binding().portal_trigger(),
            "XF86MonBrightnessDown"
        );
    }

    #[test]
    fn color_pick_defaults_to_logo_shift_p() {
        let config = Configuration::default();
        let color_pick = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::ColorPick)
            .expect("color picker should be installed by default");

        assert_eq!(color_pick.binding().portal_trigger(), "LOGO+SHIFT+P");
        assert_eq!(color_pick.binding().display(), "LOGO+SHIFT+P");
        assert_eq!(
            color_pick.binding().hypr_bind(":color_pick").value,
            "SUPER SHIFT, P, global, :color_pick"
        );
    }

    #[test]
    fn screenshot_modes_default_to_print_keys() {
        let config = Configuration::default();
        let capture_window = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::CaptureWindow)
            .expect("window screenshot should be installed by default");
        let capture_full_screen = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::CaptureFullScreen)
            .expect("full screenshot should be installed by default");

        assert_eq!(capture_window.binding().portal_trigger(), "LOGO+Print");
        assert_eq!(
            capture_window.binding().hypr_bind(":capture_window").value,
            "SUPER, Print, global, :capture_window"
        );
        assert_eq!(capture_full_screen.binding().portal_trigger(), "Print");
        assert_eq!(
            capture_full_screen
                .binding()
                .hypr_bind(":capture_full_screen")
                .value,
            ", Print, global, :capture_full_screen"
        );
    }

    #[test]
    fn toggle_recording_defaults_to_logo_shift_r() {
        let config = Configuration::default();
        let recording = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::ToggleRecording)
            .expect("recording should be installed by default");

        assert_eq!(recording.binding().portal_trigger(), "LOGO+SHIFT+R");
        assert_eq!(recording.binding().display(), "LOGO+SHIFT+R");
        assert_eq!(
            recording.binding().hypr_bind(":toggle_recording").value,
            "SUPER SHIFT, R, global, :toggle_recording"
        );
    }

    #[test]
    fn control_toggles_default_to_logo_shift_keys() {
        let config = Configuration::default();
        let dnd = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::ToggleDoNotDisturb)
            .expect("DND toggle should be installed by default");
        let night_light = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::ToggleNightLight)
            .expect("Night light toggle should be installed by default");
        let caffeine = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::ToggleCaffeine)
            .expect("Caffeine toggle should be installed by default");

        assert_eq!(dnd.binding().portal_trigger(), "LOGO+SHIFT+D");
        assert_eq!(night_light.binding().portal_trigger(), "LOGO+SHIFT+N");
        assert_eq!(caffeine.binding().portal_trigger(), "LOGO+SHIFT+F12");
    }

    #[test]
    fn app_launcher_defaults_to_standalone_super_release() {
        let config = Configuration::default();
        let app_launcher = config
            .specs()
            .find(|spec| spec.shortcut() == Shortcut::AppLauncher)
            .expect("app launcher should be installed by default");

        assert_eq!(app_launcher.binding().configured_phase(), Phase::Release);
        assert_eq!(app_launcher.binding().display(), "Super");
        assert_eq!(app_launcher.binding().portal_trigger(), "LOGO+Super_L");
        assert_eq!(
            app_launcher
                .binding()
                .hypr_exec_bind(
                    "hyprctl dispatch \"hl.dsp.global(':com.shortcut.hyprbaric.open_app_launcher')\" || hyprctl dispatch global :com.shortcut.hyprbaric.open_app_launcher",
                )
                .value,
            "SUPER, Super_L, exec, hyprctl dispatch \"hl.dsp.global(':com.shortcut.hyprbaric.open_app_launcher')\" || hyprctl dispatch global :com.shortcut.hyprbaric.open_app_launcher"
        );
        assert_eq!(
            app_launcher
                .binding()
                .hypr_exec_bind(
                    "hyprctl dispatch \"hl.dsp.global(':com.shortcut.hyprbaric.open_app_launcher')\" || hyprctl dispatch global :com.shortcut.hyprbaric.open_app_launcher",
                )
                .keyword,
            "bindr"
        );
    }

    #[test]
    fn disabling_a_hardware_mapping_omits_only_that_spec() {
        let config = toml::from_str::<Configuration>(
            r#"
            [toggle_mute]
            state = "disabled"
            "#,
        )
        .expect("shortcut config should deserialize");

        assert_eq!(config.specs().count(), Shortcut::ALL.len() - 1);
        assert!(
            !config
                .specs()
                .any(|spec| spec.shortcut() == Shortcut::ToggleMute)
        );
        assert!(
            config
                .specs()
                .any(|spec| spec.shortcut() == Shortcut::VolumeUp)
        );
    }

    #[test]
    fn builds_press_and_release_bind_keywords() {
        let press = Binding::press(Chord::logo("S")).hypr_bind(":press");
        let release = Binding {
            phase: Phase::Release,
            chord: Chord::logo("S"),
        }
        .hypr_bind(":release");

        assert_eq!(press.keyword, "bind");
        assert_eq!(release.keyword, "bindr");
    }
}
