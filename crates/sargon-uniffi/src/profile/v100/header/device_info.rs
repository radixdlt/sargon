use crate::prelude::*;

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} | created: {} | #{}", description, self.date.date(), id.to_string())]
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
    DeviceInfo::sample()
}

#[uniffi::export]
pub fn new_device_info_sample_other() -> DeviceInfo {
    DeviceInfo::sample_other()
}

#[uniffi::export]
pub fn new_device_info_from_host_info(
    host_id: &HostId,
    host_info: &HostInfo,
) -> DeviceInfo {
    DeviceInfo::new_from_info(host_id, host_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfo;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_device_info_sample(),
                new_device_info_sample_other(),
                // duplicates should get removed
                new_device_info_sample(),
                new_device_info_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_from_host_info() {
        let host_id = HostId::sample();
        let host_info = HostInfo::sample();

        assert_eq!(
            new_device_info_from_host_info(&host_id, &host_info),
            DeviceInfo::new_from_info(&host_id, &host_info)
        )
    }
}