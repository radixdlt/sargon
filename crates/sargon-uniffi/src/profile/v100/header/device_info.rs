use crate::prelude::*;
use sargon::DeviceInfo as InternalDeviceInfo;

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct DeviceInfo {
    /// A best effort stable and unique identifier of this
    /// device.
    ///
    /// Apple has made it so that iOS devices cannot
    /// query iOS for a unique identifier of the device, thus
    /// the iOS team has made their own impl of a best effort
    /// stable identifier.
    pub id: DeviceID,

    /// The date this description of the device was made, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub date: Timestamp,

    /// A short description of the device, we devices should
    /// read the device model and a given name from the device
    /// if they are able to.
    pub description: String, // FIXME: Start using `DeviceInfoDescription` !

    /// The **last known** version of the device's operating system, e.g. "iOS 17.4.1".
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub system_version: Option<String>,

    /// The **last known** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_app_version: Option<String>,

    /// The vendor of the host client, e.g. "Apple" for iPhone clients,
    /// or "Samsung" for Android clients.
    ///
    /// MUST be optional since this was added on 2024-05-16 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_vendor: Option<String>,
}

json_data_convertible!(DeviceInfo);

#[uniffi::export]
pub fn new_device_info_sample() -> DeviceInfo {
    InternalDeviceInfo::sample().into()
}

#[uniffi::export]
pub fn new_device_info_sample_other() -> DeviceInfo {
    InternalDeviceInfo::sample_other().into()
}

#[uniffi::export]
pub fn new_device_info_from_host_info(
    host_id: &HostId,
    host_info: &HostInfo,
) -> DeviceInfo {
    InternalDeviceInfo::new_from_info(
        &host_id.into_internal(),
        &host_info.into_internal(),
    )
    .into()
}
