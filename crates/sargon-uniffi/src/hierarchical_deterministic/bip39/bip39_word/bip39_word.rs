use crate::prelude::*;
use sargon::BIP39Word as InternalBIP39Word;

/// A word in the BIP39 word list of `language` at known `index` (0-2047).
#[derive(Zeroize, Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct BIP39Word {
    pub word: String,
    pub index: U11,

    #[zeroize(skip)]
    pub language: BIP39Language,
}

impl From<InternalBIP39Word> for BIP39Word {
    fn from(value: InternalBIP39Word) -> Self {
        Self {
            word: value.word,
            index: value.index.into(),
            language: value.language.into(),
        }
    }
}

impl Into<InternalBIP39Word> for BIP39Word {
    fn into(self) -> InternalBIP39Word {
        InternalBIP39Word {
            word: self.word,
            index: self.index.into(),
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

