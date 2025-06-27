use crate::prelude::*;
use sargon::BIP39WordCount as InternalBIP39WordCount;

/// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
/// standard, a multiple of 3, from 12 to 24 words. All "Babylon" `DeviceFactorSource`s
/// use 24 words.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
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

#[uniffi::export]
pub fn bip39_word_count_all() -> Vec<BIP39WordCount> {
    InternalBIP39WordCount::all().into_type()
}
