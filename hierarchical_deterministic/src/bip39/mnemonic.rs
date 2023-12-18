use std::str::FromStr;

use bip39::Language;
use itertools::Itertools;
use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::error::hdpath_error::HDPathError as Error;

use super::{bip39_word::bip39_word::BIP39Word, bip39_word_count::BIP39WordCount};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mnemonic {
    internal: bip39::Mnemonic,
    pub words: Vec<BIP39Word>,
    pub word_count: BIP39WordCount,
    pub language: Language,
}

impl Mnemonic {
    pub fn phrase(&self) -> String {
        self.words.iter().map(|w| w.word.to_string()).join(" ")
    }
    pub fn from_phrase(phrase: &str) -> Result<Self, Error> {
        let internal =
            bip39::Mnemonic::from_str(phrase).map_err(|_| Error::InvalidMnemonicPhrase)?;

        let language = internal.language();

        let words = internal
            .word_iter()
            .map(|w| BIP39Word::new(w, language))
            .collect::<Result<Vec<BIP39Word>, Error>>()?;

        let word_count = BIP39WordCount::from_count(internal.word_count())?;

        Ok(Self {
            internal,
            words,
            word_count,
            language,
        })
    }

    pub fn to_seed(&self, passphrase: &str) -> Seed {
        self.internal.to_seed(passphrase)
    }
}

pub type Seed = [u8; 64];

impl Serialize for Mnemonic {
    /// Serializes this `Mnemonic` into a phrase, all words separated by a space.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.phrase())
    }
}

impl<'de> serde::Deserialize<'de> for Mnemonic {
    /// Tries to deserializes a JSON string as a Mnemonic
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Mnemonic::from_phrase(&s).map_err(de::Error::custom)
    }
}

impl TryInto<Mnemonic> for &str {
    type Error = wallet_kit_common::error::hdpath_error::HDPathError;

    /// Tries to deserializes a bech32 address into an `AccountAddress`.
    fn try_into(self) -> Result<Mnemonic, Self::Error> {
        Mnemonic::from_phrase(self)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Mnemonic {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::from_phrase("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate").expect("Valid mnemonic")
    }
}

#[cfg(test)]
mod tests {
    use bip39::Language;
    use serde_json::json;
    use wallet_kit_common::json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use crate::bip39::{bip39_word_count::BIP39WordCount, mnemonic::Mnemonic};

    #[test]
    fn language() {
        let mnemonic: Mnemonic =
            "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"
                .try_into()
                .unwrap();
        assert_eq!(mnemonic.language, Language::English);
        mnemonic
            .words
            .into_iter()
            .for_each(|w| assert_eq!(w.language, Language::English));
    }

    #[test]
    fn word_count() {
        assert_eq!( Mnemonic::from_phrase("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate").unwrap().word_count, BIP39WordCount::TwentyFour);
        assert_eq!(
            Mnemonic::from_phrase("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong")
                .unwrap()
                .word_count,
            BIP39WordCount::Twelve
        );
    }

    #[test]
    fn words() {
        let mnemonic = Mnemonic::placeholder();
        assert_eq!(mnemonic.words[0].word, "bright");
        assert_eq!(mnemonic.words[1].word, "club");
        assert_eq!(mnemonic.words[2].word, "bacon");
        assert_eq!(mnemonic.words[12].word, "humble");
        assert_eq!(mnemonic.words[22].word, "goose");
        assert_eq!(mnemonic.words[23].word, "mandate");
    }

    #[test]
    fn words_index() {
        let zoo: Mnemonic = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            .try_into()
            .unwrap();
        assert_eq!(zoo.words[0].index.clone().into_inner(), 2047);
        assert_eq!(zoo.words[1].index.clone().into_inner(), 2047);
        assert_eq!(zoo.words[10].index.clone().into_inner(), 2047);
        assert_eq!(zoo.words[11].index.clone().into_inner(), 2037);

        let abandon: Mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .try_into()
            .unwrap();
        assert_eq!(abandon.words[0].index.clone().into_inner(), 0);
        assert_eq!(abandon.words[1].index.clone().into_inner(), 0);
        assert_eq!(abandon.words[10].index.clone().into_inner(), 0);
        assert_eq!(abandon.words[11].index.clone().into_inner(), 3);
    }

    #[test]
    fn phrase_str_roundtrip() {
        let phrase = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate";
        let mnemonic = Mnemonic::from_phrase(phrase).unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
    }

    #[test]
    fn json_roundtrip() {
        let a: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"
            .try_into()
            .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"),
        );
    }
}
