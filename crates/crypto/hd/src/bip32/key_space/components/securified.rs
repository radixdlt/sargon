use crate::prelude::*;

/// Represents a "Securified" index in a BIP32 (SLIP10) path, used as
/// the last path component in an HDPath for a securified Account or
/// Persona.
///
/// The internal representation hold a "local" offset, a non-hardened index,
/// and at the time of usage, when forming a BIP32 path, we "map" it to a
/// global offset by adding `GLOBAL_OFFSET_HARDENED_SECURIFIED` to the local index.
///
/// We can instantiate a `SecurifiedU30` from a global key space index, or from a local key space index, and we can convert it back to a global key space index.
///
/// # Examples
/// ```
/// use hierarchical_deterministic::prelude::*;
/// let securified_offset = (1u32 << 31) + (1u32 << 30);
/// // From Global KeySpace
/// assert_eq!(
///   SecurifiedU30::from_global_key_space(42 + securified_offset).unwrap(),
///   SecurifiedU30::from_local_key_space(42u32).unwrap()
/// );
///
/// assert!(SecurifiedU30::from_global_key_space(1234).is_err());
///
/// // From Local KeySpace
/// assert_eq!(
///     SecurifiedU30::from_local_key_space(55u32).unwrap().map_to_global_key_space(),
///     55 + securified_offset
/// );
///
/// // To Global KeySpace
/// assert_eq!(
///   SecurifiedU30::from_global_key_space(237 + securified_offset)
///     .unwrap()
///     .map_to_global_key_space(),
///     237 + securified_offset
/// );
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
pub struct SecurifiedU30(pub U30);

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

impl HasIndexInLocalKeySpace for SecurifiedU30 {
    fn index_in_local_key_space(&self) -> U31 {
        U31::from(self.0)
    }
}
impl IsKeySpaceAware for SecurifiedU30 {
    fn key_space(&self) -> KeySpace {
        KeySpace::Securified
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
    const VERBOSE_SYNTAX_SUFFIX: &'static str = "S";
    const SHORTHAND_SYNTAX_SUFFIX: &'static str = "^";
}

impl SecurifiedU30 {
    /// Accepts `1073741824H` which will be interpreted as `0S`
    /// and `1073741825H` which will be interpreted as `1S` etc.
    fn from_bip32_str(s: &str) -> Result<Self> {
        let offsetted = Self::value_in_local_keyspace_from_cap43_string_with_acceptable_suffixes(s,
            vec![
                HARDENED_SUFFIX_BIP32,
                HARDENED_SUFFIX_BIP44,
            ])?;
        let unoffsetted = offsetted
            .checked_sub(RELATIVELY_LOCAL_OFFSET_SECURIFIED)
            .ok_or(CommonError::IndexOverflow)?;
        Self::from_local_key_space(unoffsetted)
    }

    /// Tries to parse a CAP43 string and falls back to BIP32
    pub(crate) fn from_string_lenient(s: &str) -> Result<Self> {
        Self::from_cap43_string(s).or_else(|_| Self::from_bip32_str(s))
    }
}
impl FromStr for SecurifiedU30 {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_string_lenient(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurifiedU30;

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
    fn from_canonical_bip32() {
        assert_eq!(
            SUT::from_bip32_str("1073741825H").unwrap(),
            SUT::from_local_key_space(U31::ONE).unwrap()
        );
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
            SUT::from_local_key_space(U31::ZERO).unwrap()
        );
    }

    #[test]
    fn try_from_u32_fail() {
        assert!(SUT::try_from(SUT::MAX_LOCAL + 1).is_err());
    }

    #[test]
    fn from_str_valid_verbose_syntax_0() {
        assert_eq!(
            "0S".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ZERO).unwrap()
        );
    }

    #[test]
    fn from_str_valid_verbose_syntax_1() {
        assert_eq!(
            "1S".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ONE).unwrap()
        );
    }

    #[test]
    fn from_str_valid_verbose_syntax_max() {
        assert_eq!(
            "1073741823S".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn from_str_valid_shorthand_syntax_0() {
        assert_eq!(
            "0^".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ZERO).unwrap()
        );
    }

    #[test]
    fn from_str_valid_shorthand_syntax_1() {
        assert_eq!(
            "1^".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ONE).unwrap()
        );
    }

    #[test]
    fn from_str_valid_shorthand_syntax_max() {
        assert_eq!(
            "1073741823^".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(
            format!("{}", SUT::from_local_key_space(U31::ZERO).unwrap()),
            "0S"
        );
    }

    #[test]
    fn debug_0() {
        assert_eq!(
            format!("{:?}", SUT::from_local_key_space(U31::ZERO).unwrap()),
            "0^"
        );
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", SUT::from_local_key_space(U30_MAX).unwrap()),
            "1073741823S"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", SUT::from_local_key_space(U30_MAX).unwrap()),
            "1073741823^"
        );
    }

    #[test]
    fn try_from_hd_path_component_successful() {
        let sut = SUT::sample();
        let from = HDPathComponent::Securified(sut);
        assert_eq!(SUT::try_from(from).unwrap(), sut)
    }

    #[test]
    fn try_from_hd_path_component_fail() {
        let from = HDPathComponent::Unsecurified(Unsecurified::sample());
        assert!(matches!(
            SUT::try_from(from),
            Err(CommonError::IndexUnsecurifiedExpectedSecurified)
        ))
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<SUT>().is_err());
        assert!("foobar".parse::<SUT>().is_err());
        assert!("1X".parse::<SUT>().is_err());
        assert!("987654321987654321S".parse::<SUT>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            SUT::from_global_key_space(
                GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
            )
            .unwrap(),
            SUT::from_local_key_space(U31::try_from(1337u32).unwrap()).unwrap()
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(SUT::from_global_key_space(0).is_err());
        assert!(SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED).is_err());
        assert!(SUT::from_global_key_space(
            GLOBAL_OFFSET_HARDENED_SECURIFIED - 1
        )
        .is_err());
    }

    #[test]
    fn from_local_invalid() {
        assert!(SUT::from_local_key_space(U32_MAX).is_err());
        assert!(SUT::from_local_key_space(U31_MAX).is_err());
        assert!(SUT::from_local_key_space(U30_MAX + 1).is_err());
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            SUT::from_global_key_space(
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
            SUT::from_global_key_space(
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
            SUT::from_local_key_space(U31::try_from(1337u32).unwrap())
                .unwrap()
                .map_to_global_key_space(),
            GLOBAL_OFFSET_HARDENED_SECURIFIED + 1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut =
            SUT::from_local_key_space(U31::try_from(1337u32).unwrap()).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337S"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0S"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("^"));
        assert_json_value_fails::<SUT>(json!("S"));
        assert_json_value_fails::<SUT>(json!("2"));
        assert_json_value_fails::<SUT>(json!("2X"));
        assert_json_value_fails::<SUT>(json!("   "));
    }

    #[test]
    fn add_zero() {
        let sut =
            SUT::from_local_key_space(U31::try_from(42u32).unwrap()).unwrap();
        assert_eq!(sut.checked_add(&SUT::ZERO).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap();
        assert_eq!(sut.checked_add(&SUT::ZERO).unwrap(), sut,);
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
    fn add_one_to_max_minus_1_is_max() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL - 1).unwrap();
        assert_eq!(
            sut.checked_add(&SUT::from_local_key_space(1u32).unwrap())
                .unwrap(),
            SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap();
        assert!(matches!(
            sut.checked_add(&SUT::from_local_key_space(1u32).unwrap()),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn add_one_to_two() {
        assert_eq!(SUT::TWO.checked_add(&SUT::ONE).unwrap(), SUT::THREE);
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = SUT::from_local_key_space(U31::ONE).unwrap();
        assert!(matches!(
            sut.checked_add(
                &SUT::from_local_key_space(SUT::MAX_LOCAL).unwrap()
            ),
            Err(CommonError::IndexOverflow)
        ));
    }
}
