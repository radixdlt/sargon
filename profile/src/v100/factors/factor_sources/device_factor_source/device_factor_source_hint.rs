use std::cell::RefCell;

use hierarchical_deterministic::bip39::bip39_word_count::BIP39WordCount;
use serde::{Deserialize, Serialize};

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFactorSourceHint {
    /// "iPhone RED"
    pub name: RefCell<String>, // mutable so we can update name

    /// "iPhone SE 2nd gen"
    pub model: RefCell<String>, // mutable because name gets `async` fetched and updated later.

    /// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
    /// standard, a multiple of 3, from 12 to 24 words.
    pub mnemonic_word_count: BIP39WordCount,
}

impl DeviceFactorSourceHint {
    /// Instantiates a new DeviceFactorSourceHint from the specified name, model and word count.
    pub fn new(name: String, model: String, word_count: BIP39WordCount) -> Self {
        Self {
            name: RefCell::new(name),
            model: RefCell::new(model),
            mnemonic_word_count: word_count,
        }
    }

    pub fn iphone_unknown_model_and_name_with_word_count(word_count: BIP39WordCount) -> Self {
        Self::new("Unknown Name".to_string(), "iPhone".to_string(), word_count)
    }

    pub fn iphone_unknown() -> Self {
        Self::iphone_unknown_model_and_name_with_word_count(BIP39WordCount::TwentyFour)
    }
}

impl Default for DeviceFactorSourceHint {
    fn default() -> Self {
        Self::iphone_unknown()
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::factors::factor_sources::device_factor_source::device_factor_source_hint::DeviceFactorSourceHint;

    #[test]
    fn default_is_iphone_unknown() {
        assert_eq!(
            DeviceFactorSourceHint::default(),
            DeviceFactorSourceHint::iphone_unknown()
        );
    }

    #[test]
    fn json() {
        let model = DeviceFactorSourceHint::default();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "name": "Unknown Name",
            "model": "iPhone",
            "mnemonicWordCount": 24
        }
        "#,
        )
    }

    #[test]
    fn equality() {
        assert_eq!(
            DeviceFactorSourceHint::default(),
            DeviceFactorSourceHint::default()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            DeviceFactorSourceHint::default(),
            DeviceFactorSourceHint::iphone_unknown_model_and_name_with_word_count(
                hierarchical_deterministic::bip39::bip39_word_count::BIP39WordCount::Eighteen
            )
        );
    }
}
