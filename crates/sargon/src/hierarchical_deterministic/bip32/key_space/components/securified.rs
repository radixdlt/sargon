use crate::prelude::*;

/// Represents a "Securified" index in a BIP32 (SLIP10) path, used as
/// the last path component in an HDPath for a securified Account or
/// Persona.
///
/// The internal representation hold a non-hardened, so called "local" offset, and at the time of usage, when forming a BIP32 path, we "map" it to a global offset by adding `GLOBAL_OFFSET_HARDENED_SECURIFIED` to
/// the local index.
///
/// We can instantiate a `SecurifiedU30` from a global key space index, or from a local key space index, and we can convert it back to a global key space index.
///
/// # Examples
/// ```
/// extern crate hdpath;
/// use hdpath::prelude::*;
/// // From Global KeySpace
/// assert_eq!(
///   SecurifiedU30::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
///     .unwrap()
///     .index_in_local_key_space(),
///     U31::new(42)
/// );
///
/// assert!(
///   matches!(
///     SecurifiedU30::from_global_key_space(1234),
///     Err(CommonError::IndexInGlobalKeySpaceIsLowerThanOffset)
///  )
/// );
///
/// // From Local KeySpace
/// assert_eq!(
///     SecurifiedU30::from_local_key_space(55u32).unwrap().map_to_global_key_space(),
///     55 + GLOBAL_OFFSET_HARDENED_SECURIFIED
/// );
///
/// // To Global KeySpace
/// assert_eq!(
///   SecurifiedU30::from_global_key_space(237 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
///     .unwrap()
///     .map_to_global_key_space(),
///     237 + GLOBAL_OFFSET_HARDENED_SECURIFIED
/// );
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
pub struct SecurifiedU30(U30);

impl SecurifiedU30 {
    pub const MAX_LOCAL: u32 = U30::MAX;

    /// `Self::from_local_key_space(0).unwrap()`
    pub const ZERO: Self = Self(U30::ZERO);

    /// `Self::from_local_key_space(1).unwrap()`
    pub const ONE: Self = Self(U30::ONE);

    /// `Self::from_local_key_space(2).unwrap()`
    pub const TWO: Self = Self(U30::TWO);

    /// `Self::from_local_key_space(3.unwrap()`
    pub const THREE: Self = Self(U30::THREE);
}

impl AddViaDeref for SecurifiedU30 {}
impl AddSelfViaDeref for SecurifiedU30 {}

impl HasSampleValues for SecurifiedU30 {
    fn sample() -> Self {
        Self::from_local_key_space(*U30::sample()).unwrap()
    }

    fn sample_other() -> Self {
        Self::from_local_key_space(*U30::sample_other()).unwrap()
    }
}

impl IsInLocalKeySpace for SecurifiedU30 {
    fn key_space(&self) -> KeySpace {
        KeySpace::Securified
    }
    fn index_in_local_key_space(&self) -> U31 {
        U31::from(self.0)
    }
}
impl HasOffsetFromGlobalKeySpace for SecurifiedU30 {
    fn offset_from_global_key_space() -> u32 {
        GLOBAL_OFFSET_HARDENED_SECURIFIED
    }
}

impl FromLocalKeySpace for SecurifiedU30 {
    type Magnitude = U30;
}
impl From<U30> for SecurifiedU30 {
    fn from(value: U30) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for SecurifiedU30 {
    type Error = CommonError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        U30::try_from(value).map(Self)
    }
}

impl TryFrom<HDPathComponent> for SecurifiedU30 {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_securified()
            .map_err(|_| CommonError::IndexUnsecurifiedExpectedSecurified)
    }
}
impl IsPathComponentStringConvertible for SecurifiedU30 {
    const CANONICAL_SUFFIX: &'static str = "S";
    const NON_CANONICAL_SUFFIXES: &'static str = "^";
}
impl FromStr for SecurifiedU30 {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = SecurifiedU30;

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
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn try_from_u32_fail() {
        assert!(Sut::try_from(Sut::MAX_LOCAL + 1).is_err());
    }

    #[test]
    fn from_str_valid_canonical_0() {
        assert_eq!(
            "0S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_1() {
        assert_eq!(
            "1S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_max() {
        assert_eq!(
            "1073741823S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_0() {
        assert_eq!(
            "0^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_1() {
        assert_eq!(
            "1^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_max() {
        assert_eq!(
            "1073741823^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", Sut::from_local_key_space(0).unwrap()), "0S");
    }

    #[test]
    fn debug_0() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(0).unwrap()),
            "0^"
        );
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823S"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823^"
        );
    }

    #[test]
    fn try_from_hd_path_component_successful() {
        let sut = Sut::sample();
        let from = HDPathComponent::Securified(sut);
        assert_eq!(Sut::try_from(from).unwrap(), sut)
    }

    #[test]
    fn try_from_hd_path_component_fail() {
        let from = HDPathComponent::Unsecurified(Unsecurified::sample());
        assert!(matches!(
            Sut::try_from(from),
            Err(CommonError::IndexUnsecurifiedExpectedSecurified)
        ))
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1X".parse::<Sut>().is_err());
        assert!("987654321987654321S".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            Sut::from_global_key_space(
                GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
            )
            .unwrap(),
            Sut::from_local_key_space(1337).unwrap()
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(Sut::from_global_key_space(0).is_err());
        assert!(Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).is_err());
        assert!(Sut::from_global_key_space(
            GLOBAL_OFFSET_HARDENED_SECURIFIED - 1
        )
        .is_err());
    }

    #[test]
    fn from_local_invalid() {
        assert!(Sut::from_local_key_space(U32_MAX).is_err());
        assert!(Sut::from_local_key_space(U31_MAX).is_err());
        assert!(Sut::from_local_key_space(U30_MAX + 1).is_err());
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            Sut::from_global_key_space(
                GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
            )
            .unwrap()
            .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn map_to_local_key_space_key_space() {
        assert_eq!(
            Sut::from_global_key_space(
                GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
            )
            .unwrap()
            .key_space(),
            KeySpace::Securified
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_local_key_space(1337)
                .unwrap()
                .map_to_global_key_space(),
            GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::from_local_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337S"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0S"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("S"));
        assert_json_value_fails::<Sut>(json!("2"));
        assert_json_value_fails::<Sut>(json!("2'"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }

    #[test]
    fn add_zero() {
        let sut = Sut::from_local_key_space(42).unwrap();
        assert_eq!(sut.checked_add(&Sut::ZERO).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap();
        assert_eq!(sut.checked_add(&Sut::ZERO).unwrap(), sut,);
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
    fn add_one_to_max_minus_1_is_max() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL - 1).unwrap();
        assert_eq!(
            sut.checked_add(&Sut::from_local_key_space(1u32).unwrap())
                .unwrap(),
            Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap();
        assert!(matches!(
            sut.checked_add(&Sut::from_local_key_space(1u32).unwrap()),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn add_one_to_two() {
        assert_eq!(Sut::TWO.checked_add(&Sut::ONE).unwrap(), Sut::THREE);
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = Sut::from_local_key_space(1).unwrap();
        assert!(matches!(
            sut.checked_add(
                &Sut::from_local_key_space(Sut::MAX_LOCAL).unwrap()
            ),
            Err(CommonError::IndexOverflow)
        ));
    }
}
