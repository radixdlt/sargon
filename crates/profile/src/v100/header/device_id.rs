use crate::prelude::*;

/// A stable and globally unique identifier of a device,
/// e.g. an Android phone.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    derive_more::Display,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
#[serde(transparent)]
pub struct DeviceID(pub Uuid);

uniffi::custom_newtype!(DeviceID, Uuid);

impl DeviceID {
    pub fn generate_new() -> Self {
        Self(id())
    }
}

impl FromStr for DeviceID {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(DeviceID).map_err(|_| {
            CommonError::InvalidDeviceID {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl HasSampleValues for DeviceID {
    fn sample() -> Self {
        DeviceID(Uuid::sample())
    }

    fn sample_other() -> Self {
        DeviceID(Uuid::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceID;

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
    fn from_str_invalid() {
        assert_eq!(
            "bad".parse::<SUT>(),
            Err(CommonError::InvalidDeviceID {
                bad_value: "bad".to_owned()
            })
        );
    }

    #[test]
    fn from_upper_case_is_ok() {
        assert!(SUT::from_str("66F07CA2-A9D9-49E5-8152-77ACA3D1DD74").is_ok())
    }

    #[test]
    fn generate_new_is_unique() {
        assert_ne!(SUT::generate_new(), SUT::generate_new());
    }
}
