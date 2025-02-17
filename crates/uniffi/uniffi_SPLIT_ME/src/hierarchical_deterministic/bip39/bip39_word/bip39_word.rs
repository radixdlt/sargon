use crate::prelude::*;
use sargon::BIP39Word as InternalBIP39Word;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
pub struct BIP39Word {
    pub word: String,
    pub index: U11,
    pub language: BIP39Language,
}

#[uniffi::export]
pub fn new_bip39_word_sample() -> BIP39Word {
    InternalBIP39Word::sample().into()
}

#[uniffi::export]
pub fn new_bip39_word_sample_other() -> BIP39Word {
    InternalBIP39Word::sample_other().into()
}
