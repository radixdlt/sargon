use crate::prelude::*;

/// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
/// standard, a multiple of 3, from 12 to 24 words. All "Babylon" `DeviceFactorSource`s
/// use 24 words.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    enum_iterator::Sequence,
    Ord,
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

#[uniffi::export]
pub fn bip39_word_count_all() -> Vec<BIP39WordCount> {
    BIP39WordCount::all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bip39_word_count_all() {
        assert_eq!(bip39_word_count_all().len(), 5);
    }
}
