use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("Host '{} {}' running on {}, firmware: {}", description.name, description.model, host_os, host_app_version)]
pub struct HostInfo {
    /// A short description of the device. The host should
    /// read the device model and a given name from the device
    /// if they are able to.
    pub description: DeviceInfoDescription,

    /// The **current** os and version of the device's operating system, e.g. "iOS 17.4.1".
    pub host_os: HostOS,

    /// The **current** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    pub host_app_version: String,
}

impl HostInfo {
    pub fn new(
        description: DeviceInfoDescription,
        host_os: HostOS,
        host_app_version: impl AsRef<str>,
    ) -> Self {
        Self {
            description,
            host_os,
            host_app_version: host_app_version.as_ref().to_owned(),
        }
    }
}

impl HasSampleValues for HostInfo {
    fn sample() -> Self {
        Self {
            description: DeviceInfoDescription {
                name: "My precious".to_owned(),
                model: "iPhone SE 2nd gen".to_owned(),
            },
            host_os: HostOS::sample(),
            host_app_version: "1.6.4".to_string(),
        }
    }

    fn sample_other() -> Self {
        Self {
            description: DeviceInfoDescription {
                name: "My Pixel".to_owned(),
                model: "Pixel 8 Pro".to_owned(),
            },
            host_os: HostOS::sample_other(),
            host_app_version: "1.6.4".to_string(),
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
            "Host 'My precious iPhone SE 2nd gen' running on iOS 17.4.1, firmware: 1.6.4",
            info.to_string()
        )
    }
}
