use crate::prelude::*;
use sargon::Mnemonic as InternalMnemonic;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Mnemonic {
    pub words: Vec<BIP39Word>,

    pub word_count: BIP39WordCount,

    pub language: BIP39Language,
}

impl From<InternalMnemonic> for Mnemonic {
    fn from(value: InternalMnemonic) -> Self {
        Self {
            words: value.words.into_vec(),
            word_count: value.word_count.into(),
            language: value.language.into(),
        }
    }
}

impl Into<InternalMnemonic> for Mnemonic {
    fn into(self) -> InternalMnemonic {
        InternalMnemonic {
            words: self.words.into_internal_vec(),
            word_count: self.word_count.into_internal(),
            language: self.language.into_internal(),
        }
    }
}

#[uniffi::export]
pub fn new_mnemonic_generate_with_entropy(
    entropy: BIP39Entropy,
    language: BIP39Language,
) -> Mnemonic {
    InternalMnemonic::from_entropy_in(
        entropy.into_internal(),
        language.into_internal(),
    )
    .into()
}

/// Returns new mnemonic from a string of words
#[uniffi::export]
pub fn new_mnemonic_from_phrase(phrase: String) -> Result<Mnemonic> {
    InternalMnemonic::from_phrase(&phrase).map_result()
}

#[uniffi::export]
pub fn new_mnemonic_from_phrase_language(
    phrase: String,
    language: BIP39Language,
) -> Result<Mnemonic> {
    InternalMnemonic::from(&phrase, language.into_internal()).map_result()
}

#[uniffi::export]
pub fn new_mnemonic_from_words(words: Vec<BIP39Word>) -> Result<Mnemonic> {
    InternalMnemonic::from_words(words.into_internal_vec()).map_result()
}

/// Returns the words of a mnemonic as a String joined by spaces, e.g. "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
#[uniffi::export]
pub fn mnemonic_phrase(from: &Mnemonic) -> String {
    from.into_internal().phrase()
}

#[uniffi::export]
pub fn new_mnemonic_sample() -> Mnemonic {
    InternalMnemonic::sample().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_other() -> Mnemonic {
    InternalMnemonic::sample_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_device() -> Mnemonic {
    InternalMnemonic::sample_device().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_device_other() -> Mnemonic {
    InternalMnemonic::sample_device_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_device_12_words() -> Mnemonic {
    InternalMnemonic::sample_device_12_words().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_device_12_words_other() -> Mnemonic {
    InternalMnemonic::sample_device_12_words_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_ledger() -> Mnemonic {
    InternalMnemonic::sample_ledger().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_ledger_other() -> Mnemonic {
    InternalMnemonic::sample_ledger_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_off_device() -> Mnemonic {
    InternalMnemonic::sample_off_device().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_off_device_other() -> Mnemonic {
    InternalMnemonic::sample_off_device_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_security_questions() -> Mnemonic {
    InternalMnemonic::sample_security_questions().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_security_questions_other() -> Mnemonic {
    InternalMnemonic::sample_security_questions_other().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_arculus() -> Mnemonic {
    InternalMnemonic::sample_arculus().into()
}

#[uniffi::export]
pub fn new_mnemonic_sample_arculus_other() -> Mnemonic {
    InternalMnemonic::sample_arculus_other().into()
}
