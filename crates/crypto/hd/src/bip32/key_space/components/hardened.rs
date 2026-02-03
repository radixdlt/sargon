use crate::prelude::*;

/// Represents a hardened component in a BIP32 path, known to be "Securified" or "Unsecurified", if it's "Securified" we
/// will not only add the `GLOBAL_OFFSET_HARDENED` (2^31 / 0x80000000) to the index, but also the `RELATIVELY_LOCAL_OFFSET_SECURIFIED` (2^30 / 0x40000000) to the index.
///
/// The internal representation hold a "local" offset - a non-hardened index,
/// and at the time of usage, when forming a BIP32 path, we "map" it to a global
/// offset by adding the appropriate global offset.
///
/// The internal representation keeps track of the key space, and the index in the local key space.
///
/// We can instantiate a `Hardened` from a global key space index, or from a local key space index, and we can convert it back to a global key space index.
///
/// # Examples
/// ```
/// use hierarchical_deterministic::prelude::*;
/// let hardened_offset = 1u32 << 31;
/// let securified_offset = hardened_offset + (1u32 << 30);
/// // From Global KeySpace
/// assert_eq!(
///   Hardened::from_global_key_space(42 + hardened_offset).unwrap(),
///   Hardened::Unsecurified(UnsecurifiedHardened::from_local_key_space(42u32).unwrap())
/// );
///
/// assert_eq!(
///   Hardened::from_global_key_space(5 + securified_offset).unwrap(),
///   Hardened::Securified(SecurifiedU30::from_local_key_space(5u32).unwrap())
/// );
///
/// assert!(Hardened::from_global_key_space(3).is_err());
///
/// // From Local KeySpace
/// assert_eq!(
///   Hardened::from_local_key_space(25u32, IsSecurified(false)).unwrap(),
///   Hardened::Unsecurified(UnsecurifiedHardened::from_local_key_space(25u32).unwrap())
/// );
///
/// // Ok to pass an `U31`
/// assert_eq!(
///   Hardened::from_local_key_space(77u32, IsSecurified(false)).unwrap(),
///   Hardened::Unsecurified(UnsecurifiedHardened::from_local_key_space(77u32).unwrap())
/// );
///
/// assert_eq!(
///   Hardened::from_local_key_space(9u32, IsSecurified(true)).unwrap(),
///   Hardened::Securified(SecurifiedU30::from_local_key_space(9u32).unwrap())
/// );
///
/// // Map to global KeySpace
/// assert_eq!(
///   Hardened::from_local_key_space(9u32, IsSecurified(true)).unwrap().map_to_global_key_space(),
///   9 + securified_offset
/// );
///
/// assert_eq!(
///   Hardened::from_local_key_space(4u32, IsSecurified(false)).unwrap().map_to_global_key_space(),
///   4 + hardened_offset
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
pub enum Hardened {
    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Unsecurified(UnsecurifiedHardened),

    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Securified(SecurifiedU30),
}

impl From<UnsecurifiedHardened> for Hardened {
    fn from(value: UnsecurifiedHardened) -> Self {
        Self::Unsecurified(value)
    }
}

impl From<SecurifiedU30> for Hardened {
    fn from(value: SecurifiedU30) -> Self {
        Self::Securified(value)
    }
}

impl Hardened {
    pub const MAX_LOCAL: u32 = U31_MAX;
}

impl AddViaGlobalKeySpace for Hardened {}

impl HasSampleValues for Hardened {
    fn sample() -> Self {
        Self::Unsecurified(UnsecurifiedHardened::sample())
    }

    fn sample_other() -> Self {
        Self::Securified(SecurifiedU30::sample_other())
    }
}

impl IsMappableToGlobalKeySpace for Hardened {
    fn map_to_global_key_space(&self) -> u32 {
        match self {
            Self::Unsecurified(u) => u.map_to_global_key_space(),
            Self::Securified(s) => s.map_to_global_key_space(),
        }
    }
}
impl HasIndexInLocalKeySpace for Hardened {
    fn index_in_local_key_space(&self) -> U31 {
        match self {
            Self::Unsecurified(u) => u.index_in_local_key_space(),
            Self::Securified(s) => s.index_in_local_key_space(),
        }
    }
}
impl IsKeySpaceAware for Hardened {
    fn key_space(&self) -> KeySpace {
        match self {
            Self::Unsecurified(u) => u.key_space(),
            Self::Securified(s) => s.key_space(),
        }
    }
}

impl FromGlobalKeySpace for Hardened {
    fn from_global_key_space(value: u32) -> Result<Self> {
        SecurifiedU30::from_global_key_space(value)
            .map(Self::Securified)
            .or(UnsecurifiedHardened::from_global_key_space(value)
                .map(Self::Unsecurified))
            .map_err(|_| CommonError::IndexNotHardened { bad_value: value })
    }
}

pub const HARDENED_SUFFIX_BIP32: &str = "H";
pub const HARDENED_SUFFIX_BIP44: &str = "'";

impl FromCAP43String for Hardened {
    fn from_cap43_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        SecurifiedU30::from_cap43_string(s)
            .map(Self::Securified)
            .or(UnsecurifiedHardened::from_cap43_string(s)
                .map(Self::Unsecurified))
    }
}

pub struct IsSecurified(pub bool);

impl Hardened {
    pub fn from_local_key_space_unsecurified(
        value: impl TryInto<U31>,
    ) -> Result<Self> {
        Self::from_local_key_space(value, IsSecurified(false))
    }
    pub fn from_local_key_space(
        value: impl TryInto<U31>,
        is_securified: IsSecurified,
    ) -> Result<Self> {
        if is_securified.0 {
            SecurifiedU30::from_local_key_space(value).map(Self::Securified)
        } else {
            UnsecurifiedHardened::from_local_key_space(value)
                .map(Self::Unsecurified)
        }
    }
}

impl TryFrom<HDPathComponent> for Hardened {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        match value {
            HDPathComponent::Unsecurified(u) => {
                UnsecurifiedHardened::try_from(u).map(Self::Unsecurified)
            }
            HDPathComponent::Securified(s) => Ok(Hardened::Securified(s)),
        }
    }
}

impl FromStr for Hardened {
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
    type SUT = Hardened;

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
    fn unsecurified_from_local() {
        assert_eq!(
            SUT::from_local_key_space(U31::ONE, IsSecurified(false)).unwrap(),
            SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED + 1).unwrap()
        );

        assert_eq!(
            SUT::from_local_key_space(U31::THREE, IsSecurified(false)).unwrap(),
            SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED + 3).unwrap()
        );
    }

    #[test]
    fn securified_from_local() {
        assert_eq!(
            SUT::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap(),
            SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );

        assert_eq!(
            SUT::from_local_key_space(U31::THREE, IsSecurified(true)).unwrap(),
            SUT::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_unsecurified() {
        assert_eq!(
            "0H".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ZERO, IsSecurified(false)).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_securified() {
        assert_eq!(
            "0S".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_securified_verbose_syntax() {
        assert_eq!(
            "1S".parse::<SUT>().unwrap(),
            SUT::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }
    #[test]
    fn from_str_valid_1_securified_shorthand_syntax() {
        assert_eq!(
            "1^".parse::<SUT>().unwrap(),
            SUT::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_hardened_verbose_syntax() {
        assert_eq!(
            "1H".parse::<SUT>().unwrap(),
            SUT::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_2_hardened_shorthand_syntax() {
        assert_eq!(
            "2'".parse::<SUT>().unwrap(),
            SUT::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_3_hardened_shorthand_syntax() {
        assert_eq!(
            "3'".parse::<SUT>().unwrap(),
            SUT::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_max() {
        assert_eq!(
            "1073741823S".parse::<SUT>().unwrap(),
            SUT::from_local_key_space(U30_MAX, IsSecurified(true)).unwrap()
        );
    }

    #[test]
    fn display_unsec_1() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap()
            ),
            "1H"
        );
    }
    #[test]
    fn display_unsec_2() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap()
            ),
            "2H"
        );
    }

    #[test]
    fn display_sec_1() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_global_key_space(
                    1 + GLOBAL_OFFSET_HARDENED_SECURIFIED
                )
                .unwrap()
            ),
            "1S"
        );
    }
    #[test]
    fn display_sec_2() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_global_key_space(
                    2 + GLOBAL_OFFSET_HARDENED_SECURIFIED
                )
                .unwrap()
            ),
            "2S"
        );
    }

    #[test]
    fn debug_unsec_0() {
        assert_eq!(
            format!(
                "{:?}",
                SUT::from_local_key_space(U31::ZERO, IsSecurified(false))
                    .unwrap()
            ),
            "0'"
        );
    }

    #[test]
    fn debug_sec_0() {
        assert_eq!(
            format!(
                "{:?}",
                SUT::from_local_key_space(U31::ZERO, IsSecurified(true))
                    .unwrap()
            ),
            "0^"
        );
    }

    #[test]
    fn display_unsec_0() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_local_key_space(U31::ZERO, IsSecurified(false))
                    .unwrap()
            ),
            "0H"
        );
    }

    #[test]
    fn display_sec_0() {
        assert_eq!(
            format!(
                "{}",
                SUT::from_local_key_space(U31::ZERO, IsSecurified(true))
                    .unwrap()
            ),
            "0S"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<SUT>().is_err());
        assert!("foobar".parse::<SUT>().is_err());
        assert!("1".parse::<SUT>().is_err());
        assert!("987654321987654321".parse::<SUT>().is_err());
    }

    #[test]
    fn from_global_valid_securified() {
        assert_eq!(
            SUT::from_global_key_space(
                1337 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap(),
            SUT::Securified(
                SecurifiedU30::from_local_key_space(
                    U31::try_from(1337u32).unwrap()
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn from_global_valid_unsecurified() {
        assert_eq!(
            SUT::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED).unwrap(),
            SUT::Unsecurified(
                UnsecurifiedHardened::from_local_key_space(
                    U31::try_from(1337u32).unwrap()
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(SUT::from_global_key_space(0).is_err());
    }

    #[test]
    fn index_in_local_key_space_unsecurified() {
        assert_eq!(
            SUT::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn index_in_local_key_space_unsecurified_key_space() {
        assert_eq!(
            SUT::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .key_space(),
            KeySpace::Unsecurified { is_hardened: true }
        );
    }

    #[test]
    fn index_in_local_key_space_securified_key_space() {
        assert_eq!(
            SUT::from_global_key_space(
                1337 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap()
            .key_space(),
            KeySpace::Securified
        );
    }

    #[test]
    fn try_from_hd_path_component_securified() {
        let secu30 = SecurifiedU30::sample();
        let sut = SUT::Securified(secu30);
        let from = HDPathComponent::Securified(secu30);
        assert_eq!(SUT::try_from(from).unwrap(), sut)
    }

    #[test]
    fn index_in_local_key_space_securified() {
        assert_eq!(
            SUT::from_global_key_space(
                1337 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap()
            .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn into_global_unsecurified() {
        assert_eq!(
            SUT::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .map_to_global_key_space(),
            1337 + GLOBAL_OFFSET_HARDENED
        );
    }

    #[test]
    fn into_global_securified() {
        assert_eq!(
            SUT::from_global_key_space(
                1337 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap()
            .map_to_global_key_space(),
            1337 + GLOBAL_OFFSET_HARDENED_SECURIFIED
        );
    }

    #[test]
    fn json_roundtrip_securified() {
        let sut = SUT::from_local_key_space(
            U31::try_from(1337u32).unwrap(),
            IsSecurified(true),
        )
        .unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337S"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0S"));
    }

    #[test]
    fn json_roundtrip_unsecurified() {
        let sut = SUT::from_local_key_space(
            U31::try_from(1337u32).unwrap(),
            IsSecurified(false),
        )
        .unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337H"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0H"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(""));
        assert_json_value_fails::<SUT>(json!("^"));
        assert_json_value_fails::<SUT>(json!("2"));
        assert_json_value_fails::<SUT>(json!("2X"));
        assert_json_value_fails::<SUT>(json!("   "));
    }

    #[test]
    fn add_zero() {
        let sut =
            SUT::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(sut.checked_add_n_to_global(0).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut =
            SUT::from_global_key_space(SUT::MAX_LOCAL + GLOBAL_OFFSET_HARDENED)
                .unwrap();
        assert_eq!(sut.checked_add_n_to_global(0).unwrap(), sut,);
    }

    #[test]
    fn add_max_to_zero_is_err_since_it_changes_key_space() {
        let sut = SUT::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(sut.index_in_local_key_space(), U31::from(0));

        assert!(matches!(
            sut.checked_add_n_to_global(SUT::MAX_LOCAL),
            Err(CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace)
        ));
    }

    #[test]
    fn add_one() {
        let sut =
            SUT::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(
            sut.checked_add_one_to_global().unwrap(),
            SUT::from_global_key_space(43 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = SUT::from_global_key_space(
            SUT::MAX_LOCAL - 1 + GLOBAL_OFFSET_HARDENED,
        )
        .unwrap();
        assert_eq!(
            sut.checked_add_n_to_global(1).unwrap(),
            SUT::from_global_key_space(SUT::MAX_LOCAL + GLOBAL_OFFSET_HARDENED)
                .unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut =
            SUT::from_global_key_space(SUT::MAX_LOCAL + GLOBAL_OFFSET_HARDENED)
                .unwrap();
        assert!(matches!(
            sut.checked_add_n_to_global(1),
            Err(CommonError::IndexOverflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut =
            SUT::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert!(sut.checked_add_n_to_global(SUT::MAX_LOCAL).is_err());
    }
}
