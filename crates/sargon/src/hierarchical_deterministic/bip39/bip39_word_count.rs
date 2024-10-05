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
    /// Returns collection of all word counts
    pub fn all() -> Vec<Self> {
        all::<Self>().collect()
    }

    /// The raw representation of the word count as a number.
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    pub fn from_count(count: usize) -> Result<Self> {
        let repr = u8::try_from(count).map_err(|_| {
            CommonError::InvalidBIP39WordCount {
                bad_value: count as u64,
            }
        })?;
        let self_ = Self::from_repr(repr).ok_or(
            CommonError::InvalidBIP39WordCount {
                bad_value: count as u64,
            },
        )?;
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

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39WordCount;

    #[test]
    fn default_is_24() {
        assert_eq!(SUT::default(), SUT::TwentyFour);
    }

    #[test]
    fn test_all() {
        assert_eq!(
            SUT::all(),
            vec![
                SUT::TwentyFour,
                SUT::TwentyOne,
                SUT::Eighteen,
                SUT::Fifteen,
                SUT::Twelve,
            ]
        )
    }

    #[test]
    fn discriminant() {
        assert_eq!(SUT::TwentyFour.discriminant(), 24);
        assert_eq!(SUT::TwentyOne.discriminant(), 21);
        assert_eq!(SUT::Eighteen.discriminant(), 18);
        assert_eq!(SUT::Fifteen.discriminant(), 15);
        assert_eq!(SUT::Twelve.discriminant(), 12);
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", SUT::TwentyFour), "24 words");
        assert_eq!(format!("{}", SUT::Twelve), "12 words");
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::TwentyFour, SUT::TwentyFour);
        assert_eq!(SUT::Twelve, SUT::Twelve);
    }
    #[test]
    fn inequality() {
        assert_ne!(SUT::TwentyFour, SUT::Twelve);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::TwentyFour, SUT::TwentyFour].into_iter())
                .len(),
            1
        );
    }

    #[test]
    fn invalid_word_count_error() {
        assert_eq!(
            SUT::from_count(23),
            Err(CommonError::InvalidBIP39WordCount { bad_value: 23 })
        )
    }

    #[test]
    fn ord() {
        assert!(SUT::Twelve < SUT::TwentyFour);
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::TwentyFour, json!(24));
        assert_json_value_ne_after_roundtrip(&SUT::TwentyFour, json!(12));
        assert_json_roundtrip(&SUT::TwentyFour);
    }
}
