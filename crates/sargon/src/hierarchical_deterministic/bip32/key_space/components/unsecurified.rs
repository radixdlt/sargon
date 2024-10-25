use crate::prelude::*;

/// Represents an "unsecurified" component in a BIP32 path, known to be unhardened or hardened. If it is hardened we will add `GLOBAL_OFFSET_HARDENED` to it when mapped to global key space.
///
/// The internal representation holds wither a `Unhardened` (`U31`) or a `UnsecurifiedHardened` (`U30`).
///
/// # Examples
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert_eq!(
///     Unsecurified::from_global_key_space(1).unwrap(),
///     Unsecurified::Unhardened(Unhardened::ONE)
/// );
///
/// assert_eq!(
///     Unsecurified::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap(),
///     Unsecurified::Hardened(UnsecurifiedHardened::TWO)
/// );
///
/// assert!(
///   matches!(
///     Unsecurified::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED_SECURIFIED),
///     Err(CommonError::IndexOverflow)
///  )
/// );
/// ```
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumAsInner,
    derive_more::Display,
    derive_more::Debug,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Unsecurified {
    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Unhardened(Unhardened),

    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Hardened(UnsecurifiedHardened),
}

impl From<Unhardened> for Unsecurified {
    fn from(value: Unhardened) -> Self {
        Unsecurified::Unhardened(value)
    }
}
impl From<UnsecurifiedHardened> for Unsecurified {
    fn from(value: UnsecurifiedHardened) -> Self {
        Unsecurified::Hardened(value)
    }
}

impl Unsecurified {
    pub const MAX_LOCAL: u32 = GLOBAL_OFFSET_HARDENED_SECURIFIED - 1;
}

impl AddViaGlobalKeySpace for Unsecurified {}

impl HasSampleValues for Unsecurified {
    fn sample() -> Self {
        Self::Unhardened(Unhardened::sample())
    }

    fn sample_other() -> Self {
        Self::Hardened(UnsecurifiedHardened::sample_other())
    }
}

impl IsInLocalKeySpace for Unsecurified {
    fn key_space(&self) -> KeySpace {
        match self {
            Self::Unhardened(u) => u.key_space(),
            Self::Hardened(h) => h.key_space(),
        }
    }
    fn index_in_local_key_space(&self) -> U31 {
        match self {
            Self::Unhardened(u) => u.index_in_local_key_space(),
            Self::Hardened(h) => h.index_in_local_key_space(),
        }
    }
}

impl FromGlobalKeySpace for Unsecurified {
    fn from_global_key_space(value: u32) -> Result<Self> {
        UnsecurifiedHardened::from_global_key_space(value)
            .map(Self::Hardened)
            .or(Unhardened::from_global_key_space(value).map(Self::Unhardened))
    }
}

impl IsMappableToGlobalKeySpace for Unsecurified {
    fn map_to_global_key_space(&self) -> u32 {
        match self {
            Self::Unhardened(u) => u.map_to_global_key_space(),
            Self::Hardened(h) => h.map_to_global_key_space(),
        }
    }
}

pub struct IsHardened(pub bool);

impl Unsecurified {
    pub fn from_local_key_space(
        local: u32,
        is_hardened: IsHardened,
    ) -> Result<Self> {
        if is_hardened.0 {
            UnsecurifiedHardened::from_local_key_space(local)
                .map(Self::Hardened)
        } else {
            Unhardened::from_local_key_space(local).map(Self::Unhardened)
        }
    }
}

impl TryFrom<HDPathComponent> for Unsecurified {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_unsecurified()
            .map_err(|_| CommonError::IndexSecurifiedExpectedUnsecurified)
    }
}

impl FromBIP32Str for Unsecurified {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        UnsecurifiedHardened::from_bip32_string(s)
            .map(Self::Hardened)
            .or(Unhardened::from_bip32_string(s).map(Self::Unhardened))
    }
}

impl FromStr for Unsecurified {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = Unsecurified;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample(),);
        assert_eq!(Sut::sample_other(), Sut::sample_other(),);
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other(),);
    }

    #[test]
    fn ord() {
        assert!(Sut::sample() < Sut::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn unhardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0u32, IsHardened(false)).unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3u32, IsHardened(false)).unwrap(),
            Sut::from_global_key_space(3).unwrap()
        );
    }

    #[test]
    fn from_global_key_space_max() {
        assert_eq!(
            Sut::from_global_key_space(Sut::MAX_LOCAL).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED_SECURIFIED - 1)
                .unwrap()
        );
    }

    #[test]
    fn from_global_key_space_of_hardened() {
        assert_eq!(
            Sut::from_global_key_space(Sut::MAX_LOCAL)
                .unwrap()
                .key_space(),
            KeySpace::Unsecurified { is_hardened: true }
        );
    }

    #[test]
    fn from_global_index_of_hardened() {
        assert_eq!(
            Sut::from_global_key_space(Sut::MAX_LOCAL)
                .unwrap()
                .index_in_local_key_space(),
            U31::try_from(U30::MAX).unwrap()
        );
    }

    #[test]
    fn from_global_key_space_of_non_hardened() {
        assert_eq!(
            Sut::from_global_key_space(5).unwrap().key_space(),
            KeySpace::Unsecurified { is_hardened: false }
        );
    }

    #[test]
    fn from_global_index_of_non_hardened() {
        assert_eq!(
            Sut::from_global_key_space(5)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(5)
        );
    }

    #[test]
    fn from_global_key_space_max_plus_one_is_err() {
        assert!(Sut::from_global_key_space(Sut::MAX_LOCAL + 1).is_err());
    }

    #[test]
    fn hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, IsHardened(true)).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, IsHardened(true)).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_unhardened() {
        assert_eq!(
            "0".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_unhardened() {
        assert_eq!(
            "1".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_hardened_canonical() {
        assert_eq!(
            "0H".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_hardened_canonical() {
        assert_eq!(
            "1H".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_2_hardened_non_canonical() {
        assert_eq!(
            "2'".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_3_hardened_non_canonical() {
        assert_eq!(
            "3'".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_max() {
        assert_eq!(
            "2147483647".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(U31_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", Sut::from_global_key_space(0).unwrap()), "0");
    }

    #[test]
    fn debug_0() {
        assert_eq!(
            format!("{:?}", Sut::from_global_key_space(0).unwrap()),
            "0"
        );
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1S".parse::<Sut>().is_err());
        assert!("1^".parse::<Sut>().is_err());
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap(),
            Sut::Unhardened(Unhardened::from_global_key_space(1337).unwrap())
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .is_err()
        );
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn unhardened_map_to_local_key_space_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap().key_space(),
            KeySpace::Unsecurified { is_hardened: false }
        );
    }

    #[test]
    fn hardened_map_to_local_key_space_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .key_space(),
            KeySpace::Unsecurified { is_hardened: true }
        );
    }

    #[test]
    fn try_from_hd_path_component_fail() {
        let from = HDPathComponent::Securified(SecurifiedU30::sample());

        assert!(matches!(
            Sut::try_from(from),
            Err(CommonError::IndexSecurifiedExpectedUnsecurified)
        ))
    }

    #[test]
    fn try_from_hd_path_component_success() {
        let sut = Unsecurified::sample();
        let from = HDPathComponent::Unsecurified(sut);
        assert_eq!(Sut::try_from(from).unwrap(), sut)
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .map_to_global_key_space(),
            1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::from_global_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("2S"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }

    #[test]
    fn add_zero() {
        let sut = Sut::from_global_key_space(42).unwrap();
        assert_eq!(sut.checked_add_n_to_global(0u32).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = Sut::from_global_key_space(Sut::MAX_LOCAL).unwrap();
        assert_eq!(sut.checked_add_n_to_global(0u32).unwrap(), sut,);
    }

    #[test]
    fn add_max_to_zero_is_err_since_it_changes_key_space() {
        let sut = Sut::from_global_key_space(0).unwrap();
        assert!(matches!(
            sut.checked_add_n_to_global(Sut::MAX_LOCAL),
            Err(CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace)
        ));
    }

    #[test]
    fn add_one() {
        let sut = Sut::from_global_key_space(42).unwrap();
        assert_eq!(
            sut.checked_add_one_to_global().unwrap(),
            Sut::from_global_key_space(43).unwrap()
        );
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = Sut::from_global_key_space(Sut::MAX_LOCAL - 1).unwrap();
        assert_eq!(
            sut.checked_add_n_to_global(1u32).unwrap(),
            Sut::from_global_key_space(Sut::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = Sut::from_global_key_space(Sut::MAX_LOCAL).unwrap();
        assert!(matches!(
            sut.checked_add_n_to_global(1u32),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = Sut::from_global_key_space(1).unwrap();
        assert!(matches!(
            sut.checked_add_n_to_global(Sut::MAX_LOCAL),
            Err(CommonError::IndexOverflow)
        ));
    }
}