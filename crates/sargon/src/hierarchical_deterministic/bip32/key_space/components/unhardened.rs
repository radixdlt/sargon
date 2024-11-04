use crate::prelude::*;

/// Represents an unhardened component in a BIP32 path.
///
/// Represented internally using a U31.
/// # Examples
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
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
///     Unhardened::from_global_key_space(2u32).unwrap().index_in_local_key_space(),
///     U31::from(2)
/// );
///
/// assert!(
///   matches!(
///     Unhardened::from_global_key_space(7 + GLOBAL_OFFSET_HARDENED),
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
    Deref,
    DeserializeFromStr,
    SerializeDisplay,
    derive_more::Display,
    derive_more::Debug,
)]
#[deref(forward)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct Unhardened(pub U31);

impl Unhardened {
    pub const fn new(value: U31) -> Self {
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
    const CANONICAL_SUFFIX: &'static str = "";
    const NON_CANONICAL_SUFFIX: &'static str = "";
}

impl FromStr for Unhardened {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = Unhardened;

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
    fn try_from_u32() {
        assert_eq!(
            Sut::try_from(0u32).unwrap(),
            Sut::from_local_key_space(0u32).unwrap()
        );
    }

    #[test]
    fn try_from_u32_fail() {
        assert!(Sut::try_from(Sut::MAX_LOCAL + 1).is_err());
    }

    #[test]
    fn from_str_valid_0() {
        assert_eq!("0".parse::<Sut>().unwrap(), Sut::ZERO);
    }

    #[test]
    fn from_str_valid_1() {
        assert_eq!("1".parse::<Sut>().unwrap(), Sut::ONE);
    }

    #[test]
    fn from_str_valid_max() {
        assert_eq!(
            "1073741823".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", Sut::ZERO), "0");
    }

    #[test]
    fn debug_0() {
        assert_eq!(format!("{:?}", Sut::ZERO), "0");
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1S".parse::<Sut>().is_err());
        assert!("1^".parse::<Sut>().is_err());
        assert!("1H".parse::<Sut>().is_err());
        assert!("1'".parse::<Sut>().is_err());
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap(),
            Sut::from_local_key_space(1337u32).unwrap()
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(matches!(
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED),
            Err(CommonError::IndexOverflow)
        ));
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
    fn map_to_local_key_space_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap().key_space(),
            KeySpace::Unsecurified { is_hardened: false }
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_local_key_space(1337u32)
                .unwrap()
                .map_to_global_key_space(),
            1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::from_local_key_space(1337u32).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("S"));
        assert_json_value_fails::<Sut>(json!("2H"));
        assert_json_value_fails::<Sut>(json!("2'"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }
    #[test]
    fn add_zero() {
        let sut = Sut::from_local_key_space(42u32).unwrap();
        assert_eq!(
            sut.checked_add(&Sut::from_local_key_space(0u32).unwrap())
                .unwrap(),
            sut
        );
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap();
        assert_eq!(
            sut.checked_add(&Sut::from_local_key_space(0u32).unwrap())
                .unwrap(),
            sut,
        );
    }

    #[test]
    fn add_max_to_zero_is_ok() {
        let sut = Sut::ZERO;
        assert_eq!(
            sut.checked_add_n(Sut::MAX_LOCAL).unwrap(),
            Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn add_one() {
        let sut = Sut::from_local_key_space(42).unwrap();
        assert_eq!(
            sut.checked_add_one().unwrap(),
            Sut::from_local_key_space(43).unwrap()
        );
    }

    #[test]
    fn add_one_to_two() {
        assert_eq!(Sut::TWO.checked_add(&Sut::ONE).unwrap(), Sut::THREE);
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL - 1).unwrap();
        assert_eq!(
            sut.checked_add_one().unwrap(),
            Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap();
        assert!(matches!(
            sut.checked_add(&Sut::ONE),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = Sut::ONE;
        assert!(matches!(
            sut.checked_add(
                &Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap()
            ),
            Err(CommonError::IndexOverflow)
        ));
    }
}
