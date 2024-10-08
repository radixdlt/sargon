use crate::prelude::*;
use sargon::HostInfo as InternalHostInfo;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
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

impl From<InternalHostInfo> for HostInfo {
    fn from(value: InternalHostInfo) -> Self {
        Self {
            description: value.description.into(),
            host_os: value.host_os.into(),
            host_app_version: value.host_app_version,
        }
    }
}

impl Into<InternalHostInfo> for HostInfo {
    fn into(self) -> InternalHostInfo {
        InternalHostInfo {
            description: self.description.into(),
            host_os: self.host_os.into(),
            host_app_version: self.host_app_version,
        }
    }
}

#[uniffi::export]
pub fn new_host_info_sample() -> HostInfo {
    InternalHostInfo::sample().into()
}

#[uniffi::export]
pub fn new_host_info_sample_other() -> HostInfo {
    InternalHostInfo::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostInfo;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_host_info_sample(),
                new_host_info_sample_other(),
                // duplicates should get removed
                new_host_info_sample(),
                new_host_info_sample_other(),
            ])
            .len(),
            2
        );
    }
}
