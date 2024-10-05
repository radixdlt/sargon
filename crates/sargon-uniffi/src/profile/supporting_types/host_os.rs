use std::fmt::{Display, Formatter, Pointer};
use sargon::HostOS as InternalHostOS;

/// Describes the type of the Host machine and its version. Currently, as it stands at runtime
/// the possible values will be IOS or Android. Other is in place to facilitate unit tests
/// and to make sargon host agnostic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum HostOS {
    IOS {
        version: String,
    },
    Android {
        vendor: String,
        version: String,
    },
    Other {
        name: String,
        vendor: String,
        version: String,
    },
}

impl Into<InternalHostOS> for HostOS {
    fn into(self) -> InternalHostOS {
        match self {
            HostOS::IOS { version } => InternalHostOS::IOS { version },
            HostOS::Android { vendor, version } => InternalHostOS::Android { vendor, version },
            HostOS::Other {
                name,
                vendor,
                version,
            } => InternalHostOS::Other { name, vendor, version },
        }
    }
}

impl From<InternalHostOS> for HostOS {
    fn from(internal: InternalHostOS) -> Self {
        match internal {
            InternalHostOS::IOS { version } => HostOS::IOS { version },
            InternalHostOS::Android { vendor, version } => HostOS::Android { vendor, version },
            InternalHostOS::Other { name, vendor, version } => HostOS::Other {
                name,
                vendor,
                version,
            },
        }
    }
}

#[uniffi::export]
pub fn new_host_os_ios(version: String) -> HostOS {
    HostOS::ios(version)
}

#[uniffi::export]
pub fn new_host_os_android(vendor: String, version: String) -> HostOS {
    HostOS::android(vendor, version)
}

#[uniffi::export]
pub fn new_host_os_other(
    name: String,
    vendor: String,
    version: String,
) -> HostOS {
    HostOS::other(name, vendor, version)
}

#[uniffi::export]
pub fn host_os_get_name(host_os: &HostOS) -> String {
    host_os.name()
}

#[uniffi::export]
pub fn host_os_get_vendor(host_os: &HostOS) -> String {
    host_os.vendor()
}

#[uniffi::export]
pub fn host_os_get_version(host_os: &HostOS) -> String {
    host_os.version()
}

#[uniffi::export]
pub fn new_host_os_sample() -> HostOS {
    HostOS::sample()
}

#[uniffi::export]
pub fn new_host_os_sample_other() -> HostOS {
    HostOS::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostOS;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_host_os_sample(),
                new_host_os_sample_other(),
                // duplicates should get removed
                new_host_os_sample(),
                new_host_os_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_ios() {
        assert_eq!(new_host_os_ios("17.4.1".to_owned()), HostOS::ios("17.4.1"));
    }

    #[test]
    fn test_new_android() {
        assert_eq!(
            new_host_os_android("Google".to_owned(), "14 (API 34)".to_owned()),
            HostOS::android("Google", "14 (API 34)")
        );
    }

    #[test]
    fn test_new_other() {
        assert_eq!(
            new_host_os_other(
                "macos".to_owned(),
                "Apple".to_owned(),
                "14.5".to_owned()
            ),
            HostOS::other("macos", "Apple", "14.5")
        );
    }

    #[test]
    fn test_get_name() {
        let sut = SUT::sample();
        assert_eq!(host_os_get_name(&sut), sut.name());
    }

    #[test]
    fn test_get_vendor() {
        let sut = SUT::sample();
        assert_eq!(host_os_get_vendor(&sut), sut.vendor());
    }

    #[test]
    fn test_get_version() {
        let sut = SUT::sample();
        assert_eq!(host_os_get_version(&sut), sut.version());
    }
}
