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
)]
#[serde(rename_all = "camelCase")]
pub struct OffDeviceMnemonicHint {
    pub display_name: DisplayName,
}

impl OffDeviceMnemonicHint {
    pub fn new(display_name: DisplayName) -> Self {
        Self { display_name }
    }
}

impl HasSampleValues for OffDeviceMnemonicHint {
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
    type SUT = OffDeviceMnemonicHint;

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
