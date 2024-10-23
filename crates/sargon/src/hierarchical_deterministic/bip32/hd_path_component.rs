use crate::prelude::*;

/// A component for a BIP32 hd path, mappable into a `u32`. Retains information about the creating context, if this component is securified or not. And if it is not securified it is hardened or not.
///
///
/// ```ignore
/// [ <<------- UNHARDENED ------->> | <<-------- HARDENED --------->> ]
/// [ <<------------ UNSECURIFIED ---|-------->>  | <<- SECURIFIED ->> ]
/// ^                                ^            ^                    ^
/// 0                              2^31       2^31+2^30          2^32+1
///
/// ```
///
/// # Examples
/// ```
/// extern crate hdpath;
/// use hdpath::prelude::*;
///
/// assert_eq!(
///     HDPathComponent::from_global_key_space(0).unwrap(),
///     HDPathComponent::Unsecurified(Unsecurified::Unhardened(Unhardened::ZERO))
/// );
///
/// assert_eq!(
///     HDPathComponent::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap(),
///     HDPathComponent::Unsecurified(Unsecurified::Hardened(UnsecurifiedHardened::ONE))
/// );
///
/// assert_eq!(
///     HDPathComponent::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED_SECURIFIED).unwrap(),
///     HDPathComponent::Securified(SecurifiedU30::TWO)
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
pub enum HDPathComponent {
    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Unsecurified(Unsecurified),

    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Securified(SecurifiedU30),
}

impl From<Unsecurified> for HDPathComponent {
    fn from(value: Unsecurified) -> Self {
        Self::Unsecurified(value)
    }
}
impl From<SecurifiedU30> for HDPathComponent {
    fn from(value: SecurifiedU30) -> Self {
        Self::Securified(value)
    }
}

impl AddViaGlobalKeySpace for HDPathComponent {}

impl HasSampleValues for HDPathComponent {
    fn sample() -> Self {
        Self::Unsecurified(Unsecurified::sample())
    }

    fn sample_other() -> Self {
        Self::Securified(SecurifiedU30::sample_other())
    }
}

impl IsInLocalKeySpace for HDPathComponent {
    fn key_space(&self) -> KeySpace {
        match self {
            Self::Unsecurified(u) => u.key_space(),
            Self::Securified(s) => s.key_space(),
        }
    }
    fn index_in_local_key_space(&self) -> U31 {
        match self {
            Self::Unsecurified(u) => u.index_in_local_key_space(),
            Self::Securified(s) => s.index_in_local_key_space(),
        }
    }
}

impl FromGlobalKeySpace for HDPathComponent {
    fn from_global_key_space(value: u32) -> Result<Self> {
        SecurifiedU30::from_global_key_space(value)
            .map(Self::Securified)
            .or(Unsecurified::from_global_key_space(value)
                .map(Self::Unsecurified))
    }
}

impl From<Hardened> for HDPathComponent {
    fn from(value: Hardened) -> Self {
        match value {
            Hardened::Unsecurified(u) => {
                HDPathComponent::Unsecurified(Unsecurified::from(u))
            }
            Hardened::Securified(s) => HDPathComponent::Securified(s),
        }
    }
}

impl HDPathComponent {
    fn securified(value: impl Into<SecurifiedU30>) -> Self {
        Self::Securified(value.into())
    }

    pub fn unsecurified_hardened(local_index: u32) -> Result<Self> {
        Self::from_local_key_space(
            local_index,
            KeySpace::Unsecurified { is_hardened: true },
        )
    }
}

impl HDPathComponent {
    pub fn is_unhardened(&self) -> bool {
        match self {
            Self::Unsecurified(u) => u.is_unhardened(),
            Self::Securified(_) => false,
        }
    }
    pub fn is_hardened(&self) -> bool {
        match self {
            Self::Unsecurified(u) => u.is_hardened(),
            Self::Securified(_) => true,
        }
    }
}

impl FromBIP32Str for HDPathComponent {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        SecurifiedU30::from_bip32_string(s)
            .map(Self::securified)
            .or(Unsecurified::from_bip32_string(s).map(Self::Unsecurified))
    }
}

impl FromStr for HDPathComponent {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

impl IsMappableToGlobalKeySpace for HDPathComponent {
    fn map_to_global_key_space(&self) -> u32 {
        match self {
            HDPathComponent::Unsecurified(u) => u.map_to_global_key_space(),
            HDPathComponent::Securified(s) => s.map_to_global_key_space(),
        }
    }
}

impl HDPathComponent {
    pub fn from_local_key_space(
        value: u32,
        key_space: KeySpace,
    ) -> Result<Self> {
        match key_space {
            KeySpace::Securified => {
                SecurifiedU30::from_local_key_space(value).map(Self::Securified)
            }
            KeySpace::Unsecurified { is_hardened } => {
                Unsecurified::from_local_key_space(
                    value,
                    IsHardened(is_hardened),
                )
                .map(Self::Unsecurified)
            }
        }
    }
}

impl From<NetworkID> for HDPathComponent {
    fn from(value: NetworkID) -> Self {
        HDPathComponent::unsecurified_hardened(value.discriminant() as u32)
            .expect("NetworkID values are small so always fit inside U30")
    }
}

impl From<CAP26EntityKind> for HDPathComponent {
    fn from(value: CAP26EntityKind) -> Self {
        HDPathComponent::unsecurified_hardened(value.discriminant())
            .expect("CAP26EntityKind values are small so always fit inside U30")
    }
}

impl From<CAP26KeyKind> for HDPathComponent {
    fn from(value: CAP26KeyKind) -> Self {
        HDPathComponent::unsecurified_hardened(value.discriminant())
            .expect("CAP26KeyKind values are small so always fit inside U30")
    }
}

/// # Safety
/// Unsafe, does not validate the value to be small enough.
///
/// Only use this for tests and constants.
const unsafe fn hard(value: u16) -> HDPathComponent {
    unsafe {
        HDPathComponent::Unsecurified(Unsecurified::Hardened(
            UnsecurifiedHardened::new(U30::new(value)),
        ))
    }
}

/// # Safety
/// Unsafe, does not validate the value to be small enough.
///
/// Only use this for tests and constants.
const unsafe fn unhard(value: u16) -> HDPathComponent {
    HDPathComponent::Unsecurified(Unsecurified::Unhardened(Unhardened::new(
        U31::new(value),
    )))
}

pub(crate) const PURPOSE: HDPathComponent = unsafe { hard(44) };
pub(crate) const GET_ID_LAST: HDPathComponent = unsafe { hard(365) };
pub(crate) const COIN_TYPE: HDPathComponent = unsafe { hard(1022) };
pub(crate) const BIP44_ACCOUNT: HDPathComponent = unsafe { hard(0) };
pub(crate) const BIP44_CHANGE: HDPathComponent = unsafe { unhard(0) };

pub(crate) fn cap26(
    network_id: NetworkID,
    entity_kind: CAP26EntityKind,
    key_kind: CAP26KeyKind,
    index: Hardened,
) -> HDPath {
    let mut path: [HDPathComponent; 6] = [PURPOSE; 6];
    path[1] = COIN_TYPE;
    path[2] = HDPathComponent::from(network_id);
    path[3] = HDPathComponent::from(entity_kind);
    path[4] = HDPathComponent::from(key_kind);
    path[5] = HDPathComponent::from(index);
    HDPath::new(Vec::from_iter(path))
}

pub(crate) fn bip44(index: HDPathComponent) -> HDPath {
    let mut path: [HDPathComponent; 5] = [PURPOSE; 5];
    path[1] = COIN_TYPE;
    path[2] = BIP44_ACCOUNT;
    path[3] = BIP44_CHANGE;
    path[4] = index;
    HDPath::new(Vec::from_iter(path))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = HDPathComponent;

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
    fn key_space() {
        let sut = Sut::Securified(SecurifiedU30::sample());
        assert!(sut.key_space().is_securified())
    }

    #[test]
    fn securified_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, KeySpace::Securified).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, KeySpace::Securified).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }

    #[test]
    fn unsecurified_unhardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: false }
            )
            .unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(
                3,
                KeySpace::Unsecurified { is_hardened: false }
            )
            .unwrap(),
            Sut::from_global_key_space(3).unwrap()
        );
    }

    #[test]
    fn unsecurified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(
                3,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_hardened() {
        let sec = SecurifiedU30::sample();
        let hardened = Hardened::Securified(sec);
        assert_eq!(Sut::from(hardened), Sut::Securified(sec));
    }

    #[test]
    fn map_to_global_securified() {
        let sec = SecurifiedU30::sample();
        let sut = Sut::Securified(sec);
        assert_eq!(
            sut.map_to_global_key_space(),
            30 + GLOBAL_OFFSET_HARDENED_SECURIFIED
        );
    }

    #[test]
    fn from_local_key_space_securified() {
        assert_eq!(
            Sut::from_local_key_space(42, KeySpace::Securified).unwrap(),
            Sut::securified(U30::new(42))
        )
    }

    #[test]
    fn securified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, KeySpace::Securified).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, KeySpace::Securified).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
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
    fn display_u30_max() {
        assert_eq!(
            format!("{}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn debug_u30_max() {
        assert_eq!(
            format!("{:?}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }


    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap(),
            Sut::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(1337).unwrap()
            ))
        );

        assert_eq!(
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap(),
            Sut::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(42).unwrap()
            ))
        );

        assert_eq!(
            Sut::from_global_key_space(237 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap(),
            Sut::Securified(SecurifiedU30::from_local_key_space(237).unwrap())
        );
    }

    #[test]
    fn index_of_local_key_space() {
        let sut = Sut::from_global_key_space(1337).unwrap();
        assert!(sut.key_space().is_unsecurified_unhardened());
        assert_eq!(sut.index_in_local_key_space(), U31::from(1337));
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
    fn into_global() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .map_to_global_key_space(),
            1337
        );
    }

    #[test]
    fn json_roundtrip_unhardened() {
        let sut = Sut::from_global_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_roundtrip_hardened_unsecurified() {
        let sut =
            Sut::from_global_key_space(6 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json!("6H"));
    }

    #[test]
    fn json_roundtrip_securified() {
        let sut = Sut::from_global_key_space(
            5109 + GLOBAL_OFFSET_HARDENED_SECURIFIED,
        )
        .unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json!("5109S"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }

    #[test]
    fn add_zero_unhardened() {
        let sut = Sut::from_global_key_space(42).unwrap();
        assert_eq!(sut.checked_add_n_to_global(0).unwrap(), sut);
    }

    #[test]
    fn add_zero_unsecurified_hardened() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(sut.checked_add_n_to_global(0).unwrap(), sut);
    }

    #[test]
    fn add_zero_securified() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap();
        assert_eq!(sut.checked_add_n_to_global(0).unwrap(), sut);
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
    fn add_one_unsecurified_unhardened() {
        let sut = Sut::from_global_key_space(42).unwrap();
        assert_eq!(
            sut.checked_add_one_to_global().unwrap(),
            Sut::from_global_key_space(43).unwrap()
        );
    }

    #[test]
    fn add_one_unsecurified_unhardened_max_is_err() {
        let sut = Sut::Unsecurified(Unsecurified::Unhardened(
            Unhardened::from_local_key_space(Unhardened::MAX_LOCAL).unwrap(),
        ));
        assert!(sut.checked_add_one_to_global().is_err());
    }

    #[test]
    fn cannot_add_one_to_max_unsecurified_unhardened() {
        assert!(matches!(
            Sut::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(Unhardened::MAX_LOCAL)
                    .unwrap()
            ))
            .checked_add_n_to_global(1),
            Err(CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace)
        ))
    }

    #[test]
    fn cannot_add_one_to_max_unsecurified_hardened() {
        assert!(matches!(
            Sut::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(
                    UnsecurifiedHardened::MAX_LOCAL
                )
                .unwrap()
            ))
            .checked_add_n_to_global(1),
            Err(CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace)
        ))
    }

    #[test]
    fn add_one_unsecurified_hardened() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(
            sut.checked_add_one_to_global().unwrap(),
            Sut::from_global_key_space(43 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn add_one_securified() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap();
        assert_eq!(
            sut.checked_add_one_to_global().unwrap(),
            Sut::from_global_key_space(43 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }

    #[test]
    fn add_three_unsecurified_unhardened() {
        let sut = Sut::from_global_key_space(42).unwrap();
        assert_eq!(
            sut.checked_add_n_to_global(3).unwrap(),
            Sut::from_global_key_space(45).unwrap()
        );
    }

    #[test]
    fn add_three_unsecurified_hardened() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_eq!(
            sut.checked_add_n_to_global(3).unwrap(),
            Sut::from_global_key_space(45 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn add_three_securified() {
        let sut =
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap();
        assert_eq!(
            sut.checked_add_n_to_global(3).unwrap(),
            Sut::from_global_key_space(45 + GLOBAL_OFFSET_HARDENED_SECURIFIED)
                .unwrap()
        );
    }
}
