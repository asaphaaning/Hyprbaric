//! RINF projections for shortcut settings.

use crate::signals;

use super::{
    Shortcut,
    binding::{self, Binding},
    domain, settings,
};

impl From<Shortcut> for signals::ShortcutSettingId {
    fn from(shortcut: Shortcut) -> Self {
        match shortcut {
            Shortcut::AppLauncher => Self::AppLauncher,
            Shortcut::Controls => Self::Controls,
            Shortcut::BarSettings => Self::BarSettings,
            Shortcut::SessionLauncher => Self::SessionLauncher,
            Shortcut::LockSession => Self::LockSession,
            Shortcut::CaptureRegion => Self::CaptureRegion,
            Shortcut::CaptureWindow => Self::CaptureWindow,
            Shortcut::CaptureFullScreen => Self::CaptureFullScreen,
            Shortcut::ColorPick => Self::ColorPick,
            Shortcut::ToggleRecording => Self::ToggleRecording,
            Shortcut::ToggleDoNotDisturb => Self::ToggleDoNotDisturb,
            Shortcut::ToggleNightLight => Self::ToggleNightLight,
            Shortcut::ToggleCaffeine => Self::ToggleCaffeine,
            Shortcut::VolumeUp => Self::VolumeUp,
            Shortcut::VolumeDown => Self::VolumeDown,
            Shortcut::ToggleMute => Self::ToggleMute,
            Shortcut::BrightnessUp => Self::BrightnessUp,
            Shortcut::BrightnessDown => Self::BrightnessDown,
        }
    }
}

impl From<signals::ShortcutSettingId> for Shortcut {
    fn from(shortcut: signals::ShortcutSettingId) -> Self {
        match shortcut {
            signals::ShortcutSettingId::AppLauncher => Self::AppLauncher,
            signals::ShortcutSettingId::Controls => Self::Controls,
            signals::ShortcutSettingId::BarSettings => Self::BarSettings,
            signals::ShortcutSettingId::SessionLauncher => Self::SessionLauncher,
            signals::ShortcutSettingId::LockSession => Self::LockSession,
            signals::ShortcutSettingId::CaptureRegion => Self::CaptureRegion,
            signals::ShortcutSettingId::CaptureWindow => Self::CaptureWindow,
            signals::ShortcutSettingId::CaptureFullScreen => Self::CaptureFullScreen,
            signals::ShortcutSettingId::ColorPick => Self::ColorPick,
            signals::ShortcutSettingId::ToggleRecording => Self::ToggleRecording,
            signals::ShortcutSettingId::ToggleDoNotDisturb => Self::ToggleDoNotDisturb,
            signals::ShortcutSettingId::ToggleNightLight => Self::ToggleNightLight,
            signals::ShortcutSettingId::ToggleCaffeine => Self::ToggleCaffeine,
            signals::ShortcutSettingId::VolumeUp => Self::VolumeUp,
            signals::ShortcutSettingId::VolumeDown => Self::VolumeDown,
            signals::ShortcutSettingId::ToggleMute => Self::ToggleMute,
            signals::ShortcutSettingId::BrightnessUp => Self::BrightnessUp,
            signals::ShortcutSettingId::BrightnessDown => Self::BrightnessDown,
        }
    }
}

impl From<domain::Category> for signals::ShortcutSettingCategory {
    fn from(category: domain::Category) -> Self {
        match category {
            domain::Category::Bar => Self::Bar,
            domain::Category::Session => Self::Session,
            domain::Category::Capture => Self::Capture,
            domain::Category::Audio => Self::Audio,
            domain::Category::Display => Self::Display,
        }
    }
}

impl From<binding::Phase> for signals::ShortcutBindingPhase {
    fn from(phase: binding::Phase) -> Self {
        match phase {
            binding::Phase::Press => Self::Press,
            binding::Phase::Release => Self::Release,
        }
    }
}

impl From<signals::ShortcutBindingPhase> for binding::Phase {
    fn from(phase: signals::ShortcutBindingPhase) -> Self {
        match phase {
            signals::ShortcutBindingPhase::Press => Self::Press,
            signals::ShortcutBindingPhase::Release => Self::Release,
        }
    }
}

impl From<binding::Modifier> for signals::ShortcutModifier {
    fn from(modifier: binding::Modifier) -> Self {
        match modifier {
            binding::Modifier::Logo => Self::Logo,
            binding::Modifier::Ctrl => Self::Ctrl,
            binding::Modifier::Shift => Self::Shift,
            binding::Modifier::Alt => Self::Alt,
            binding::Modifier::Num => Self::Num,
        }
    }
}

impl From<signals::ShortcutModifier> for binding::Modifier {
    fn from(modifier: signals::ShortcutModifier) -> Self {
        match modifier {
            signals::ShortcutModifier::Logo => Self::Logo,
            signals::ShortcutModifier::Ctrl => Self::Ctrl,
            signals::ShortcutModifier::Shift => Self::Shift,
            signals::ShortcutModifier::Alt => Self::Alt,
            signals::ShortcutModifier::Num => Self::Num,
        }
    }
}

impl TryFrom<signals::ShortcutBindingInput> for Binding {
    type Error = binding::InvalidBinding;

    fn try_from(binding: signals::ShortcutBindingInput) -> Result<Self, Self::Error> {
        Binding::from_parts(
            binding.phase.into(),
            binding.modifiers.into_iter().map(Into::into),
            &binding.key,
        )
    }
}

impl TryFrom<signals::ShortcutSettingsRequest> for settings::Command {
    type Error = binding::InvalidBinding;

    fn try_from(request: signals::ShortcutSettingsRequest) -> Result<Self, Self::Error> {
        match request {
            signals::ShortcutSettingsRequest::Load => Ok(Self::Load),
            signals::ShortcutSettingsRequest::SetBinding { shortcut, binding } => {
                Ok(Self::SetBinding {
                    shortcut: shortcut.into(),
                    binding: binding.try_into()?,
                })
            }
            signals::ShortcutSettingsRequest::Disable { shortcut } => Ok(Self::Disable {
                shortcut: shortcut.into(),
            }),
            signals::ShortcutSettingsRequest::Reset { shortcut } => Ok(Self::Reset {
                shortcut: shortcut.into(),
            }),
        }
    }
}

impl From<&settings::Snapshot> for signals::ShortcutSettingsSnapshot {
    fn from(snapshot: &settings::Snapshot) -> Self {
        Self {
            rows: snapshot.rows.iter().map(Into::into).collect(),
            writable_path: snapshot.writable_path.display().to_string(),
            message: snapshot.message.clone(),
        }
    }
}

impl From<&settings::Row> for signals::ShortcutSettingsRow {
    fn from(row: &settings::Row) -> Self {
        Self {
            shortcut: row.shortcut.into(),
            label: row.label.to_owned(),
            description: row.description.to_owned(),
            category: row.category.into(),
            default_mapping: (&row.default_mapping).into(),
            effective_mapping: (&row.effective_mapping).into(),
            source: row.source.into(),
            conflict: row.conflict.map(Into::into),
        }
    }
}

impl From<&settings::ViewMapping> for signals::ShortcutMappingView {
    fn from(mapping: &settings::ViewMapping) -> Self {
        match mapping {
            settings::ViewMapping::Bound(binding) => Self::Bound {
                binding: binding.into(),
            },
            settings::ViewMapping::Disabled => Self::Disabled,
        }
    }
}

impl From<&settings::ViewBinding> for signals::ShortcutBindingView {
    fn from(binding: &settings::ViewBinding) -> Self {
        Self {
            phase: binding.phase.into(),
            modifiers: binding.modifiers.iter().copied().map(Into::into).collect(),
            key: binding.key.clone(),
            display: binding.display.clone(),
        }
    }
}

impl From<settings::Source> for signals::ShortcutMappingSource {
    fn from(source: settings::Source) -> Self {
        match source {
            settings::Source::Default => Self::Builtin,
            settings::Source::UserOverride => Self::UserOverride,
            settings::Source::Disabled => Self::Disabled,
        }
    }
}

impl From<&settings::Command> for signals::ShortcutSettingsRequest {
    fn from(command: &settings::Command) -> Self {
        match command {
            settings::Command::Load => Self::Load,
            settings::Command::SetBinding { shortcut, binding } => Self::SetBinding {
                shortcut: (*shortcut).into(),
                binding: signals::ShortcutBindingInput {
                    phase: binding.configured_phase().into(),
                    modifiers: binding.modifiers().into_iter().map(Into::into).collect(),
                    key: binding.key(),
                },
            },
            settings::Command::Disable { shortcut } => Self::Disable {
                shortcut: (*shortcut).into(),
            },
            settings::Command::Reset { shortcut } => Self::Reset {
                shortcut: (*shortcut).into(),
            },
        }
    }
}

impl From<&settings::Report> for signals::ShortcutSettingsCommandResult {
    fn from(report: &settings::Report) -> Self {
        match report {
            settings::Report::Started(command) => Self {
                command: command.into(),
                outcome: signals::ShortcutSettingsCommandOutcome::Started,
                shortcut: command.shortcut().map(Into::into),
                message: None,
            },
            settings::Report::Saved(command) => Self {
                command: command.into(),
                outcome: signals::ShortcutSettingsCommandOutcome::Saved,
                shortcut: command.shortcut().map(Into::into),
                message: None,
            },
            settings::Report::Failed { command, message } => Self {
                command: command.into(),
                outcome: signals::ShortcutSettingsCommandOutcome::Failed,
                shortcut: command.shortcut().map(Into::into),
                message: Some(message.clone()),
            },
        }
    }
}
