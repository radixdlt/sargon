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
#[display("{label} {word_count}")]
pub struct OffDeviceMnemonicHint {
    /// A user-assigned name for the passphrase, intended to help users
    /// differentiate between multiple passphrases.
    pub label: DisplayName,

    /// The number of words the `OffDeviceMnemonic`, intended to help the host provide the correct
    /// input form for validation when the user enters the words.
    pub word_count: BIP39WordCount,
}

impl OffDeviceMnemonicHint {
    pub fn new(label: DisplayName, word_count: BIP39WordCount) -> Self {
        Self { label, word_count }
    }
}

impl HasSampleValues for OffDeviceMnemonicHint {
    fn sample() -> Self {
        // https://xkcd.com/936/
        Self::new(
            DisplayName::new("Story about a horse").unwrap(),
            BIP39WordCount::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DisplayName::new("Thrilled with a shark").unwrap(),
            BIP39WordCount::sample_other(),
        )
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
