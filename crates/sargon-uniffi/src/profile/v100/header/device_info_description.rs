use crate::prelude::*;
use sargon::DeviceInfoDescription as InternalDeviceInfoDescription;

/// A name and model of a host device.
///
/// This used to be a String only in Pre 1.6.0 wallets, so
/// we have a custom Deserialize impl of it.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    InternalConersion,
     uniffi::Record,
)]
pub struct DeviceInfoDescription {
    /// Host device name, e.g. "My Precious"
    pub name: String,

    /// Host device model, e.g. "iPhone 15 Pro"
    pub model: String,
}

impl From<InternalDeviceInfoDescription> for DeviceInfoDescription {
    fn from(value: InternalDeviceInfoDescription) -> Self {
        Self {
            name: value.name,
            model: value.model,
        }
    }
}

impl Into<InternalDeviceInfoDescription> for DeviceInfoDescription {
    fn into(self) -> InternalDeviceInfoDescription {
        InternalDeviceInfoDescription {
            name: self.name,
            model: self.model,
        }
    }
}

#[uniffi::export]
pub fn new_device_info_description_sample() -> DeviceInfoDescription {
    InternalDeviceInfoDescription::sample().into()
}

#[uniffi::export]
pub fn new_device_info_description_sample_other() -> DeviceInfoDescription {
    InternalDeviceInfoDescription::sample_other().into()
}

#[uniffi::export]
pub fn device_info_description_to_string(
    device_info_description: &DeviceInfoDescription,
) -> String {
    device_info_description.into_internal().to_string()
}

