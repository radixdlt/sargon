use std::str::FromStr;

use crate::{BIP39Language, HDPathError as Error, Hex32Bytes};
use itertools::Itertools;
use serde::{de, Deserializer, Serialize, Serializer};

use crate::{BIP39Word, BIP39WordCount};

use crate::HasPlaceholder;

#[derive(Clone, PartialEq, Eq, Debug, Hash, uniffi::Record)]
pub struct Mnemonic {
    pub words: Vec<BIP39Word>,
    pub word_count: BIP39WordCount,
    pub language: BIP39Language,
}

/// Returns the words of a mnemonic as a String joined by spaces, e.g. "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
#[uniffi::export]
pub fn mnemonic_phrase(from: &Mnemonic) -> String {
    from.phrase()
}

impl Mnemonic {
    fn from_internal(internal: bip39::Mnemonic) -> Self {
        let language = internal.language();

        let words = internal
            .word_iter()
            .map(|w| BIP39Word::new(w, language.into()))
            .collect::<Result<Vec<BIP39Word>, Error>>()
            .expect("Crate bip39 generated words unknown to us.");

        let word_count = BIP39WordCount::from_count(internal.word_count())
            .expect("Crate bip39 generated a BIP39 standard incompatible word count.");

        Self {
            words,
            word_count,
            language: language.into(),
        }
    }
    pub fn from_entropy(entropy: [u8; 32]) -> Self {
        let internal = bip39::Mnemonic::from_entropy(entropy.as_slice()).unwrap();
        Self::from_internal(internal)
    }
    pub fn from_hex32(bytes: Hex32Bytes) -> Self {
        Self::from_entropy(bytes.bytes())
    }

    pub fn generate_new() -> Self {
        Self::from_hex32(Hex32Bytes::generate())
    }
    fn internal(&self) -> bip39::Mnemonic {
        bip39::Mnemonic::from_str(&self.phrase()).unwrap()
    }
    pub fn phrase(&self) -> String {
        self.words.iter().map(|w| w.word.to_string()).join(" ")
    }
    pub fn from_phrase(phrase: &str) -> Result<Self, Error> {
        bip39::Mnemonic::from_str(phrase)
            .map_err(|_| Error::InvalidMnemonicPhrase)
            .map(Self::from_internal)
    }

    pub fn to_seed(&self, passphrase: &str) -> Seed {
        self.internal().to_seed(passphrase)
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
    type Error = crate::HDPathError;

    /// Tries to deserializes a bech32 address into an `AccountAddress`.
    fn try_into(self) -> Result<Mnemonic, Self::Error> {
        Mnemonic::from_phrase(self)
    }
}

impl HasPlaceholder for Mnemonic {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::from_phrase("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate").expect("Valid mnemonic")
    }

    fn placeholder_other() -> Self {
        Self::from_phrase("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong")
            .expect("Valid mnemonic")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip, BIP39Language, HDPathError, HasPlaceholder,
    };

    use serde_json::json;

    use crate::{BIP39WordCount, Mnemonic};

    #[test]
    fn equality() {
        assert_eq!(Mnemonic::placeholder(), Mnemonic::placeholder());
        assert_eq!(Mnemonic::placeholder_other(), Mnemonic::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Mnemonic::placeholder(), Mnemonic::placeholder_other());
    }

    #[test]
    fn language() {
        let mnemonic: Mnemonic =
            "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"
                .try_into()
                .unwrap();
        assert_eq!(mnemonic.language, BIP39Language::English);
        mnemonic
            .words
            .into_iter()
            .for_each(|w| assert_eq!(w.language, BIP39Language::English));
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
        assert_eq!(zoo.words[0].index.clone().inner, 2047);
        assert_eq!(zoo.words[1].index.clone().inner, 2047);
        assert_eq!(zoo.words[10].index.clone().inner, 2047);
        assert_eq!(zoo.words[11].index.clone().inner, 2037);

        let abandon: Mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .try_into()
            .unwrap();
        assert_eq!(abandon.words[0].index.clone().inner, 0);
        assert_eq!(abandon.words[1].index.clone().inner, 0);
        assert_eq!(abandon.words[10].index.clone().inner, 0);
        assert_eq!(abandon.words[11].index.clone().inner, 3);
    }

    #[test]
    fn phrase_str_roundtrip() {
        let phrase = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate";
        let mnemonic = Mnemonic::from_phrase(phrase).unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
    }

    #[test]
    fn from_phrase_invalid() {
        assert_eq!(
            Mnemonic::from_phrase("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon"),
            Err(HDPathError::InvalidMnemonicPhrase)
        );
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
