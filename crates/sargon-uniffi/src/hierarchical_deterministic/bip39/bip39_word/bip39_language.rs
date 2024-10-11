use crate::prelude::*;
use sargon::BIP39Language as InternalBIP39Language;

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(Clone, Hash, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum BIP39Language {
    /// The English language.
    English,

    /// The French language.
    French,
}

impl From<InternalBIP39Language> for BIP39Language {
    fn from(value: InternalBIP39Language) -> Self {
        match value {
            InternalBIP39Language::English => Self::English,
            InternalBIP39Language::French => Self::French,
        }
    }
}

impl Into<InternalBIP39Language> for BIP39Language {
    fn into(self) -> InternalBIP39Language {
        match self {
            Self::English => InternalBIP39Language::English,
            Self::French => InternalBIP39Language::French,
        }
    }
}

#[uniffi::export]
pub fn new_bip39_language_sample() -> BIP39Language {
    InternalBIP39Language::sample().into()
}

#[uniffi::export]
pub fn new_bip39_language_sample_other() -> BIP39Language {
    InternalBIP39Language::sample_other().into()
}

#[uniffi::export]
pub fn bip39_language_wordlist(language: &BIP39Language) -> Vec<BIP39Word> {
    language.into_internal().wordlist().into_vec()
}
