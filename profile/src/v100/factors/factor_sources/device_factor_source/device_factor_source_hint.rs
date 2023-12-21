use std::cell::RefCell;

use derive_getters::Getters;
use hd::bip39::bip39_word_count::BIP39WordCount;
use serde::{Deserialize, Serialize};

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFactorSourceHint {
    /// "iPhone RED"
    name: RefCell<String>, // mutable so we can update name

    /// "iPhone SE 2nd gen"
    model: RefCell<String>, // mutable because name gets `async` fetched and updated later.

    /// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
    /// standard, a multiple of 3, from 12 to 24 words.
    mnemonic_word_count: BIP39WordCount,
}

impl DeviceFactorSourceHint {
    pub fn set_name(&self, new: String) {
        *self.name.borrow_mut() = new
    }
    pub fn set_model(&self, new: String) {
        *self.model.borrow_mut() = new
    }
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

    pub fn unknown_model_and_name_with_word_count(word_count: BIP39WordCount, model: &str) -> Self {
        Self::new("Unknown Name".to_string(), model.to_string(), word_count)
    }
    pub fn iphone_unknown_model_and_name_with_word_count(word_count: BIP39WordCount) -> Self {
        Self::unknown_model_and_name_with_word_count(word_count, "iPhone")
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl DeviceFactorSourceHint {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::placeholder_iphone_unknown()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_iphone_unknown() -> Self {
        Self::iphone_unknown_model_and_name_with_word_count(BIP39WordCount::TwentyFour)
    }
}

#[cfg(test)]
mod tests {
    use hd::bip39::bip39_word_count::BIP39WordCount;
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::factors::factor_sources::device_factor_source::device_factor_source_hint::DeviceFactorSourceHint;

    #[test]
    fn json() {
        let model = DeviceFactorSourceHint::placeholder_iphone_unknown();
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
            DeviceFactorSourceHint::placeholder_iphone_unknown(),
            DeviceFactorSourceHint::placeholder_iphone_unknown()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            DeviceFactorSourceHint::placeholder_iphone_unknown(),
            DeviceFactorSourceHint::iphone_unknown_model_and_name_with_word_count(
                BIP39WordCount::Eighteen
            )
        );
    }
}
