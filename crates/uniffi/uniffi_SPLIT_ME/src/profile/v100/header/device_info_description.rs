use crate::prelude::*;
use sargon::DeviceInfoDescription as InternalDeviceInfoDescription;

/// A name and model of a host device.
///
/// This used to be a String only in Pre 1.6.0 wallets, so
/// we have a custom Deserialize impl of it.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct DeviceInfoDescription {
    /// Host device name, e.g. "My Precious"
    pub name: String,

    /// Host device model, e.g. "iPhone 15 Pro"
    pub model: String,
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
