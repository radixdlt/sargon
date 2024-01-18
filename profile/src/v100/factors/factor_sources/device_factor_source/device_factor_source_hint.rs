use crate::prelude::*;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{name} {model}")]
pub struct DeviceFactorSourceHint {
    /// "iPhone RED"
    pub name: String,

    /// "iPhone SE 2nd gen"
    pub model: String,

    /// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
    /// standard, a multiple of 3, from 12 to 24 words.
    pub mnemonic_word_count: BIP39WordCount,
}

impl DeviceFactorSourceHint {
    /// Instantiates a new DeviceFactorSourceHint from the specified name, model and word count.
    pub fn new(name: String, model: String, word_count: BIP39WordCount) -> Self {
        Self {
            name,
            model,
            mnemonic_word_count: word_count,
        }
    }

    pub fn unknown_model_of_client(
        word_count: BIP39WordCount,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        Self::new(
            "Unknown Name".to_string(),
            wallet_client_model.to_string(),
            word_count,
        )
    }

    pub fn iphone_unknown_model_with_word_count(word_count: BIP39WordCount) -> Self {
        Self::unknown_model_of_client(word_count, WalletClientModel::Iphone)
    }
}

impl HasPlaceholder for DeviceFactorSourceHint {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_iphone_unknown()
    }

    fn placeholder_other() -> Self {
        Self::new(
            "Android".to_string(),
            "Samsung Galaxy S23 Ultra".to_string(),
            BIP39WordCount::Twelve,
        )
    }
}

impl DeviceFactorSourceHint {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_iphone_unknown() -> Self {
        Self::iphone_unknown_model_with_word_count(BIP39WordCount::TwentyFour)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(
            DeviceFactorSourceHint::placeholder(),
            DeviceFactorSourceHint::placeholder()
        );
        assert_eq!(
            DeviceFactorSourceHint::placeholder_other(),
            DeviceFactorSourceHint::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            DeviceFactorSourceHint::placeholder(),
            DeviceFactorSourceHint::placeholder_other()
        );
    }

    #[test]
    fn set_model() {
        let mut sut = DeviceFactorSourceHint::placeholder();
        assert_eq!(sut.model, "iPhone".to_string());
        sut.model = "Android".to_string();
        assert_eq!(sut.model, "Android".to_string());
    }

    #[test]
    fn set_name() {
        let mut sut = DeviceFactorSourceHint::placeholder();
        sut.name = "Foo".to_string();
        assert_eq!(sut.name, "Foo".to_string());
    }

    #[test]
    fn get_word_count() {
        assert_eq!(
            DeviceFactorSourceHint::placeholder().mnemonic_word_count,
            BIP39WordCount::TwentyFour
        );
    }

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
}
