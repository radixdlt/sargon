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

impl std::fmt::Display for BIP39WordCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} words", self.discriminant())
    }
}

impl BIP39WordCount {
    /// The raw representation of the word count as a number.
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    pub fn from_count(count: usize) -> Result<Self> {
        let repr = u8::try_from(count).map_err(|_| CommonError::InvalidBIP39WordCount(count))?;
        let self_ = Self::from_repr(repr).ok_or(CommonError::InvalidBIP39WordCount(count))?;
        Ok(self_)
    }
}

impl Default for BIP39WordCount {
    fn default() -> Self {
        Self::TwentyFour
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };
    use serde_json::json;

    use crate::BIP39WordCount;

    #[test]
    fn default_is_24() {
        assert_eq!(BIP39WordCount::default(), BIP39WordCount::TwentyFour);
    }

    #[test]
    fn discriminant() {
        assert_eq!(BIP39WordCount::TwentyFour.discriminant(), 24);
        assert_eq!(BIP39WordCount::TwentyOne.discriminant(), 21);
        assert_eq!(BIP39WordCount::Eighteen.discriminant(), 18);
        assert_eq!(BIP39WordCount::Fifteen.discriminant(), 15);
        assert_eq!(BIP39WordCount::Twelve.discriminant(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", BIP39WordCount::TwentyFour), "24 words");
        assert_eq!(format!("{}", BIP39WordCount::Twelve), "12 words");
    }

    #[test]
    fn equality() {
        assert_eq!(BIP39WordCount::TwentyFour, BIP39WordCount::TwentyFour);
        assert_eq!(BIP39WordCount::Twelve, BIP39WordCount::Twelve);
    }
    #[test]
    fn inequality() {
        assert_ne!(BIP39WordCount::TwentyFour, BIP39WordCount::Twelve);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter(
                [BIP39WordCount::TwentyFour, BIP39WordCount::TwentyFour].into_iter()
            )
            .len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(BIP39WordCount::Twelve < BIP39WordCount::TwentyFour);
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&BIP39WordCount::TwentyFour, json!(24));
        assert_json_value_ne_after_roundtrip(&BIP39WordCount::TwentyFour, json!(12));
        assert_json_roundtrip(&BIP39WordCount::TwentyFour);
    }
}
