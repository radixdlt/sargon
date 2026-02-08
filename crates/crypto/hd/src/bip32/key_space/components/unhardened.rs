use crate::prelude::*;

/// Represents an unhardened component in a BIP32 path.
///
/// Represented internally using a U31.
/// # Examples
/// ```
/// use hierarchical_deterministic::prelude::*;
/// let hardened_offset = 1u32 << 31;
///
/// assert_eq!(
///     Unhardened::from_local_key_space(0u32).unwrap().map_to_global_key_space(),
///     0
/// );
///
/// assert_eq!(
///     Unhardened::from_global_key_space(1u32).unwrap().map_to_global_key_space(),
///     1
/// );
///
/// assert_eq!(
///     Unhardened::from_global_key_space(2u32).unwrap(),
///     Unhardened::from_local_key_space(2u32).unwrap()
/// );
///
/// assert!(
///     Unhardened::from_global_key_space(7 + hardened_offset).is_err()
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
    DeserializeFromStr,
    SerializeDisplay,
    derive_more::Deref,
    derive_more::Display,
    derive_more::Debug,
)]
#[deref(forward)]
#[display("{}", self.to_cap43_string())]
#[debug("{}", self.to_cap43_string_debug())]
pub struct Unhardened(pub U31);

impl Unhardened {
    pub(crate) const fn new(value: U31) -> Self {
        Self(value)
    }
}

impl Unhardened {
    pub const MAX_LOCAL: u32 = U31::MAX;

    /// `Self::from_local_key_space(0).unwrap()`
    pub const ZERO: Self = Self(U31::ZERO);

    /// `Self::from_local_key_space(1).unwrap()`
    pub const ONE: Self = Self(U31::ONE);

    /// `Self::from_local_key_space(2).unwrap()`
    pub const TWO: Self = Self(U31::TWO);

    /// `Self::from_local_key_space(3.unwrap()`
    pub const THREE: Self = Self(U31::THREE);
}

impl AddViaDeref for Unhardened {}
impl AddSelfViaDeref for Unhardened {}

impl HasSampleValues for Unhardened {
    fn sample() -> Self {
        Self::from_local_key_space(*U31::sample()).unwrap()
    }

    fn sample_other() -> Self {
        Self::from_local_key_space(*U31::sample_other()).unwrap()
    }
}

impl HasIndexInLocalKeySpace for Unhardened {
    fn index_in_local_key_space(&self) -> U31 {
        self.0
    }
}
impl IsKeySpaceAware for Unhardened {
    fn key_space(&self) -> KeySpace {
        KeySpace::Unsecurified { is_hardened: false }
    }
}

impl HasOffsetFromGlobalKeySpace for Unhardened {
    fn offset_from_global_key_space() -> u32 {
        0
    }
}

impl FromLocalKeySpace for Unhardened {
    type Magnitude = U31;
}

impl From<U31> for Unhardened {
    fn from(value: U31) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for Unhardened {
    type Error = CommonError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        U31::try_from(value).map(Self)
    }
}

impl IsPathComponentStringConvertible for Unhardened {
    const VERBOSE_SYNTAX_SUFFIX: &'static str = "";
    const SHORTHAND_SYNTAX_SUFFIX: &'static str = "";
}

impl FromStr for Unhardened {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_cap43_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Unhardened;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample(),);
        assert_eq!(SUT::sample_other(), SUT::sample_other(),);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other(),);
    }

    #[test]
    fn ord() {
        assert!(SUT::sample() < SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn try_from_u32() {
        assert_eq!(
            SUT::try_from(0u32).unwrap(),
            SUT::from_local_key_space(0u32).unwrap()
        );
    }

    #[test]
    fn try_from_u32_fail() {
        assert!(SUT::try_from(SUT::MAX_LOCAL + 1).is_err());
    }

    #[test]
    fn from_str_valid_0() {
        assert_eq!("0".parse::<SUT>().unwrap(), SUT::ZERO);
    }

    #[test]
    fn from_str_valid_1() {
        assert_eq!("1".parse::<SUT>().unwrap(), SUT::ONE);
    }

    #[test]
    fn from_str_valid_max() {
        assert_eq!(
            "1073741823".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", SUT::ZERO), "0");
    }

    #[test]
    fn debug_0() {
        assert_eq!(format!("{:?}", SUT::ZERO), "0");
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", SUT::from_local_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", SUT::from_local_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<SUT>().is_err());
        assert!("foobar".parse::<SUT>().is_err());
        assert!("1S".parse::<SUT>().is_err());
        assert!("1^".parse::<SUT>().is_err());
        assert!("1H".parse::<SUT>().is_err());
        assert!("1'".parse::<SUT>().is_err());
        assert!("987654321987654321".parse::<SUT>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            SUT::from_global_key_space(1337).unwrap(),
            SUT::from_local_key_space(1337u32).unwrap()
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(matches!(
            SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            SUT::from_global_key_space(1337)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn map_to_local_key_space_key_space() {
        assert_eq!(
            SUT::from_global_key_space(1337).unwrap().key_space(),
            KeySpace::Unsecurified { is_hardened: false }
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            SUT::from_local_key_space(1337u32)
                .unwrap()
                .map_to_global_key_space(),
            1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::from_local_key_space(1337u32).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("^"));
        assert_json_value_fails::<SUT>(json!("S"));
        assert_json_value_fails::<SUT>(json!("2H"));
        assert_json_value_fails::<SUT>(json!("2'"));
        assert_json_value_fails::<SUT>(json!("2X"));
        assert_json_value_fails::<SUT>(json!("   "));
    }
    #[test]
    fn add_zero() {
        let sut = SUT::from_local_key_space(42u32).unwrap();
        assert_eq!(
            sut.checked_add(&SUT::from_local_key_space(0u32).unwrap())
                .unwrap(),
            sut
        );
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap();
        assert_eq!(
            sut.checked_add(&SUT::from_local_key_space(0u32).unwrap())
                .unwrap(),
            sut,
        );
    }

    #[test]
    fn add_max_to_zero_is_ok() {
        let sut = SUT::ZERO;
        assert_eq!(
            sut.checked_add_n(SUT::MAX_LOCAL).unwrap(),
            SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn add_one() {
        let sut =
            SUT::from_local_key_space(U31::try_from(42u32).unwrap()).unwrap();
        assert_eq!(
            sut.checked_add_one().unwrap(),
            SUT::from_local_key_space(U31::try_from(43u32).unwrap()).unwrap()
        );
    }

    #[test]
    fn add_one_to_two() {
        assert_eq!(SUT::TWO.checked_add(&SUT::ONE).unwrap(), SUT::THREE);
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL - 1).unwrap();
        assert_eq!(
            sut.checked_add_one().unwrap(),
            SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap();
        assert!(matches!(
            sut.checked_add(&SUT::ONE),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = SUT::ONE;
        assert!(matches!(
            sut.checked_add(
                &SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap()
            ),
            Err(CommonError::IndexOverflow)
        ));
    }
}
