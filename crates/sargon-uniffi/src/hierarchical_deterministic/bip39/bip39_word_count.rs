use crate::prelude::*;
use sargon::BIP39WordCount as InternalBIP39WordCount;

/// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
/// standard, a multiple of 3, from 12 to 24 words. All "Babylon" `DeviceFactorSource`s
/// use 24 words.
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,

    uniffi::Enum,
)]
#[repr(u8)]
pub enum BIP39WordCount {
    /// 24 words, used by all "Babylon" `DeviceFactorSource`s
    TwentyFour = 24,

    /// 21 words, potentially used by third-party Olympia wallets.
    TwentyOne = 21,

    /// 18 words, potentially used by third-party Olympia wallets.
    Eighteen = 18,

    /// 15 words, potentially used by third-party Olympia wallets.
    Fifteen = 15,

    /// 12 words, used by Radix Olympia legacy wallet.
    Twelve = 12,
}

impl From<InternalBIP39WordCount> for BIP39WordCount {
    fn from(value: InternalBIP39WordCount) -> Self {
        match value {
            InternalBIP39WordCount::TwentyFour => BIP39WordCount::TwentyFour,
            InternalBIP39WordCount::TwentyOne => BIP39WordCount::TwentyOne,
            InternalBIP39WordCount::Eighteen => BIP39WordCount::Eighteen,
            InternalBIP39WordCount::Fifteen => BIP39WordCount::Fifteen,
            InternalBIP39WordCount::Twelve => BIP39WordCount::Twelve,
        }
    }
}

impl Into<InternalBIP39WordCount> for BIP39WordCount {
    fn into(self) -> InternalBIP39WordCount {
        match self {
            BIP39WordCount::TwentyFour => InternalBIP39WordCount::TwentyFour,
            BIP39WordCount::TwentyOne => InternalBIP39WordCount::TwentyOne,
            BIP39WordCount::Eighteen => InternalBIP39WordCount::Eighteen,
            BIP39WordCount::Fifteen => InternalBIP39WordCount::Fifteen,
            BIP39WordCount::Twelve => InternalBIP39WordCount::Twelve,
        }
    }
}

#[uniffi::export]
pub fn bip39_word_count_all() -> Vec<BIP39WordCount> {
    InternalBIP39WordCount::all().into_iter().map(Into::into).collect()
}

