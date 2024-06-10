use crate::prelude::*;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct OffDeviceFactorSourceHint {
    pub display_name: DisplayName,
}

impl OffDeviceFactorSourceHint {
    pub fn new(display_name: DisplayName) -> Self {
        Self { display_name }
    }
}

impl HasSampleValues for OffDeviceFactorSourceHint {
    fn sample() -> Self {
        Self::new(DisplayName::new("Story about a horse").unwrap())
    }

    fn sample_other() -> Self {
        Self::new(DisplayName::new("Thrilled with a shark").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceFactorSourceHint;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
