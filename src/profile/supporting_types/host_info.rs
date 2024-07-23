use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
)]
#[display("Host {} '{} {}' running on {}, firmware: {}", id, description.name, description.model, host_os_version, host_app_version)]
pub struct HostInfo {
    /// A best effort stable and unique identifier of this
    /// device.
    pub id: DeviceID,

    /// The date this description of the device id was generated, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub date: Timestamp,

    /// A short description of the device. The host should
    /// read the device model and a given name from the device
    /// if they are able to.
    pub description: DeviceInfoDescription,

    /// The **current** version of the device's operating system, e.g. "iOS 17.4.1".
    pub host_os_version: String,

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    pub host_app_version: String,

    /// The vendor of the host client, e.g. "Apple" for iPhone clients,
    /// or "Samsung" for Android clients.
    pub host_vendor: String,
}

impl HostInfo {
    pub fn new(
        host_id: HostId,
        description: DeviceInfoDescription,
        host_os_version: impl AsRef<str>,
        host_app_version: impl AsRef<str>,
        host_vendor: impl AsRef<str>,
    ) -> Self {
        Self {
            id: host_id.id,
            date: host_id.generated_at,
            description,
            host_os_version: host_os_version.as_ref().to_owned(),
            host_app_version: host_app_version.as_ref().to_owned(),
            host_vendor: host_vendor.as_ref().to_owned(),
        }
    }
}

// HostInfo can be converted to DeviceInfo but not the opposite
// without contacting HostInfoDriver
#[allow(clippy::from_over_into)]
impl Into<DeviceInfo> for HostInfo {
    fn into(self) -> DeviceInfo {
        let info = self.clone();
        DeviceInfo::new(
            info.id,
            info.date,
            info.description,
            info.host_os_version,
            info.host_app_version,
            info.host_vendor,
        )
    }
}

impl HasSampleValues for HostInfo {
    fn sample() -> Self {
        Self {
            id: DeviceID::from_str("66F07CA2-A9D9-49E5-8152-77ACA3D1DD74")
                .unwrap(),
            date: Timestamp::sample(),
            description: DeviceInfoDescription {
                name: "My precious".to_owned(),
                model: "iPhone SE 2nd gen".to_owned(),
            },
            host_os_version: "iOS 17.4.1".to_string(),
            host_app_version: "1.6.4".to_string(),
            host_vendor: "Apple".to_string(),
        }
    }

    fn sample_other() -> Self {
        Self {
            id: DeviceID::from_str("f07ca662-d9a9-9e45-1582-aca773d174dd")
                .unwrap(),
            date: Timestamp::sample_other(),
            description: DeviceInfoDescription {
                name: "My Pixel".to_owned(),
                model: "Pixel 8 Pro".to_owned(),
            },
            host_os_version: "Android 14".to_string(),
            host_app_version: "1.6.4".to_string(),
            host_vendor: "Google".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostInfo;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_to_string() {
        let info = HostInfo::sample();
        assert_eq!(
            "Host 66f07ca2-a9d9-49e5-8152-77aca3d1dd74 'My precious iPhone SE 2nd gen' running on iOS 17.4.1, firmware: 1.6.4",
            info.to_string()
        )
    }

    #[test]
    fn test_into_device_info() {
        let sut = SUT::sample();

        assert_eq!(
            DeviceInfo {
                id: sut.id,
                date: sut.date,
                description: sut.description.to_string(),
                system_version: Some(sut.host_os_version.clone()),
                host_app_version: Some(sut.host_app_version.clone()),
                host_vendor: Some(sut.host_vendor.clone()),
            },
            sut.into()
        )
    }
}
