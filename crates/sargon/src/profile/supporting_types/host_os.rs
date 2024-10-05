use crate::HasSampleValues;
use std::fmt::{Display, Formatter, Pointer};

/// Describes the type of the Host machine and its version. Currently, as it stands at runtime
/// the possible values will be IOS or Android. Other is in place to facilitate unit tests
/// and to make sargon host agnostic.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl HostOS {
    /// Creates an iOS kind with version
    pub fn ios(version: impl AsRef<str>) -> Self {
        Self::IOS {
            version: version.as_ref().to_string(),
        }
    }

    /// Creates an Android kind with version
    pub fn android(vendor: impl AsRef<str>, version: impl AsRef<str>) -> Self {
        Self::Android {
            vendor: vendor.as_ref().to_string(),
            version: version.as_ref().to_string(),
        }
    }

    /// Creates an Other kind with a custom name and version
    pub fn other(
        name: impl AsRef<str>,
        vendor: impl AsRef<str>,
        version: impl AsRef<str>,
    ) -> Self {
        Self::Other {
            name: name.as_ref().to_string(),
            vendor: vendor.as_ref().to_string(),
            version: version.as_ref().to_string(),
        }
    }

    /// Returns the name of the host's os
    pub fn name(&self) -> String {
        match self {
            HostOS::IOS { .. } => "iOS".to_owned(),
            HostOS::Android { .. } => "Android".to_owned(),
            HostOS::Other {
                name,
                vendor: _vendor,
                version: _,
            } => name.clone(),
        }
    }

    /// Returns the version name of the host's os.
    pub fn version(&self) -> String {
        let version = match self {
            HostOS::IOS { version } => version,
            HostOS::Android { vendor: _, version } => version,
            HostOS::Other {
                name: _,
                vendor: _,
                version,
            } => version,
        };

        format!("{} {}", self.name(), version)
    }

    /// Returns the vendor of this host's os.
    pub fn vendor(&self) -> String {
        match self {
            HostOS::IOS { .. } => "Apple".to_owned(),
            HostOS::Android { vendor, version: _ } => vendor.to_owned(),
            HostOS::Other {
                name: _name,
                vendor,
                version: _,
            } => vendor.to_owned(),
        }
    }
}

impl Display for HostOS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version())
    }
}

impl HasSampleValues for HostOS {
    fn sample() -> Self {
        HostOS::ios("17.4.1")
    }

    fn sample_other() -> Self {
        HostOS::android("Google", "14 (API 34)")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HostOS;

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
    fn test_display() {
        assert_eq!(SUT::sample().to_string(), "iOS 17.4.1");
        assert_eq!(SUT::sample_other().to_string(), "Android 14 (API 34)");
        assert_eq!(
            SUT::other("Custom", "Unknown", "1.0.0").to_string(),
            "Custom 1.0.0"
        );
    }
}
