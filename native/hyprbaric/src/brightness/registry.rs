//! Cached brightness devices and optimistic state.
//!
//! [`Registry`] owns the selected device and keeps UI state responsive while
//! backend writes settle asynchronously.

use super::{
    Error,
    domain::{Device, DeviceId, DeviceKind, Percent, Snapshot, Target},
};

/// Cached brightness devices and the current selection.
#[derive(Default)]
pub(super) struct Registry {
    devices: Vec<Device>,
    selected: Option<DeviceId>,
}

impl Registry {
    /// Returns whether no brightness device is currently known.
    pub(super) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Replaces all devices of one [`DeviceKind`] while preserving other kinds.
    pub(super) fn replace_kind(&mut self, kind: DeviceKind, devices: Vec<Device>) {
        self.devices.retain(|device| device.kind != kind);
        self.devices.extend(devices);
        self.devices = sorted_devices(std::mem::take(&mut self.devices));
        let selected_missing = self
            .selected
            .as_ref()
            .map(|selected| !self.devices.iter().any(|device| &device.id == selected))
            .unwrap_or(true);
        if selected_missing {
            self.selected = self.devices.first().map(|device| device.id.clone());
        }
    }

    /// Inserts or replaces one known [`Device`].
    pub(super) fn upsert(&mut self, device: Device) {
        if let Some(known) = self.devices.iter_mut().find(|known| known.id == device.id) {
            *known = device;
        } else {
            self.devices.push(device);
        }
        self.devices = sorted_devices(std::mem::take(&mut self.devices));
        if self.selected.is_none() {
            self.selected = self.devices.first().map(|device| device.id.clone());
        }
    }

    /// Returns a device by identity.
    pub(super) fn device(&self, id: &DeviceId) -> Option<&Device> {
        self.devices.iter().find(|device| &device.id == id)
    }

    /// Applies an optimistic brightness value to the selected device.
    pub(super) fn set_optimistic(
        &mut self,
        target: Target,
        value: Percent,
    ) -> Result<Device, Error> {
        let device_id = match target {
            Target::Selected => self.selected_id().ok_or(Error::NoDevice)?,
        };
        let Some(index) = self
            .devices
            .iter()
            .position(|device| device.id == device_id)
        else {
            return Err(Error::DeviceUnavailable {
                device: device_id.to_string(),
            });
        };
        self.devices[index].value = value;
        self.selected = Some(self.devices[index].id.clone());
        Ok(self.devices[index].clone())
    }

    /// Builds the UI-facing [`Snapshot`] from the selected device.
    pub(super) fn snapshot(&self, message: Option<&str>) -> Snapshot {
        if let Some(device) = self.selected_device() {
            return Snapshot::Available {
                value: device.value,
                device: device.label.clone(),
            };
        }

        Snapshot::unavailable(message.unwrap_or("No brightness device is available"))
    }

    fn selected_id(&self) -> Option<DeviceId> {
        if let Some(selected) = &self.selected
            && self.devices.iter().any(|device| &device.id == selected)
        {
            return Some(selected.clone());
        }
        self.devices.first().map(|device| device.id.clone())
    }

    fn selected_device(&self) -> Option<&Device> {
        let selected = self.selected_id()?;
        self.device(&selected)
    }
}

pub(super) fn sorted_devices(mut devices: Vec<Device>) -> Vec<Device> {
    devices.sort_by(|left, right| {
        device_kind_rank(left.kind)
            .cmp(&device_kind_rank(right.kind))
            .then(
                left.label
                    .cmp(&right.label)
                    .then(left.id.to_string().cmp(&right.id.to_string())),
            )
    });
    devices
}

pub(super) fn deduplicate_devices(devices: Vec<Device>) -> Vec<Device> {
    let mut merged = Vec::new();
    for device in sorted_devices(devices) {
        if merged
            .iter()
            .any(|known: &Device| known.id == device.id || known.label == device.label)
        {
            continue;
        }
        merged.push(device);
    }
    merged
}

const fn device_kind_rank(kind: DeviceKind) -> u8 {
    match kind {
        DeviceKind::Backlight => 0,
        DeviceKind::DdcCi => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brightness::domain::{Device, DeviceId, DeviceKind, Percent};

    #[test]
    fn registry_optimistically_updates_selected_device() {
        let mut registry = Registry::default();
        registry.replace_kind(
            DeviceKind::Backlight,
            vec![Device {
                id: DeviceId::backlight("intel_backlight"),
                label: "Internal".to_owned(),
                kind: DeviceKind::Backlight,
                value: Percent::new(40),
            }],
        );

        let device = registry
            .set_optimistic(Target::Selected, Percent::new(72))
            .expect("selected device should exist");

        assert_eq!(device.value, Percent::new(72));
        assert_eq!(
            registry.snapshot(None),
            Snapshot::Available {
                value: Percent::new(72),
                device: "Internal".to_owned(),
            }
        );
    }
}
