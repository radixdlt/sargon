use crate::prelude::*;
use sargon::BIP39Word as InternalBIP39Word;
use sargon::U11 as InternalU11;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct BIP39Word {
    pub word: String,
    pub index: u16,
    pub language: BIP39Language,
}

impl From<InternalBIP39Word> for BIP39Word {
    fn from(value: InternalBIP39Word) -> Self {
        Self {
            word: value.word,
            index: value.index.inner,
            language: value.language.into(),
        }
    }
}

impl Into<InternalBIP39Word> for BIP39Word {
    fn into(self) -> InternalBIP39Word {
        InternalBIP39Word {
            word: self.word,
            index: InternalU11 { inner: self.index },
            language: self.language.into(),
        }
    }
}

#[uniffi::export]
pub fn new_bip39_word_sample() -> BIP39Word {
    InternalBIP39Word::sample().into()
}

#[uniffi::export]
pub fn new_bip39_word_sample_other() -> BIP39Word {
    InternalBIP39Word::sample_other().into()
}
