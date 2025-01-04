use short_string::prelude::DisplayName;

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
    /// A user-assigned name for the passphrase, intended to help users
    /// differentiate between multiple passphrases.
    pub label: DisplayName,
}

impl OffDeviceMnemonicHint {
    pub fn new(label: DisplayName) -> Self {
        Self { label }
    }
}

impl HasSampleValues for OffDeviceMnemonicHint {
    fn sample() -> Self {
        // https://xkcd.com/936/
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
