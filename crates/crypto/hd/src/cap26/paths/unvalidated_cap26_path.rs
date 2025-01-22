use crate::prelude::*;

/// A derivation path consisting of CAP26 components, however, not validated
/// as canonical.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UnvalidatedCAP26Path {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
}

impl UnvalidatedCAP26Path {
    pub const DEPTH: usize = 6;
}

impl TryFrom<HDPathComponent> for NetworkID {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        Self::try_from(value.index_in_local_key_space())
    }
}

pub const CAP26_PATH_ENTITY_INDEX_POS: usize = 5;

impl TryFrom<HDPath> for UnvalidatedCAP26Path {
    type Error = CommonError;

    fn try_from(path: HDPath) -> Result<Self> {
        let components = path.components();
        if components.iter().any(|c| c.is_unhardened()) {
            return Err(CommonError::NotAllComponentsAreHardened);
        }
        if components.len() != Self::DEPTH {
            return Err(CommonError::InvalidDepthOfCAP26Path {
                expected: Self::DEPTH as u64,
                found: components.len() as u64,
            });
        }
        if components[0] != PURPOSE {
            return Err(CommonError::BIP44PurposeNotFound {
                bad_value: u32::from(components[0].index_in_local_key_space()),
            });
        }
        if components[1] != COIN_TYPE {
            return Err(CommonError::CoinTypeNotFound {
                bad_value: u32::from(components[1].index_in_local_key_space()),
            });
        }
        let network_id = NetworkID::try_from(components[2])?;

        let entity_kind = CAP26EntityKind::try_from(components[3])?;
        let key_kind = CAP26KeyKind::try_from(components[4])?;

        let hardened =
            Hardened::try_from(components[CAP26_PATH_ENTITY_INDEX_POS])?;

        Ok(UnvalidatedCAP26Path {
            network_id,
            entity_kind,
            key_kind,
            index: hardened,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnvalidatedCAP26Path;

    #[test]
    fn from_str_invalid_purpose() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44/1022H/1H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/43H/1022H/1H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::BIP44PurposeNotFound { bad_value: _ })
        ));
    }

    #[test]
    fn from_str_invalid_cointype() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022/1H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/55555H/1H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::CoinTypeNotFound { bad_value: _ })
        ));
    }

    #[test]
    fn from_str_invalid_network_id_unknown() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/5555/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));

        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/200H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::UnknownNetworkID { bad_value: _ })
        ));
    }

    #[test]
    fn from_str_invalid_network_id_too_large() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/5555/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));

        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/5555H/525H/1460H/0H").unwrap()
            ),
            Err(CommonError::InvalidNetworkIDExceedsLimit { bad_value: 5555 })
        ));
    }

    #[test]
    fn from_str_invalid_entity_kind() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/1H/525/1460H/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/1H/333H/1460H/0H").unwrap()
            ),
            Err(CommonError::InvalidEntityKind { bad_value: _ })
        ));
    }

    #[test]
    fn from_str_invalid_key_kind() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/1H/525H/1460/0H").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));

        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/1H/525H/22H/0H").unwrap()
            ),
            Err(CommonError::InvalidKeyKind { bad_value: _ })
        ));
    }

    #[test]
    fn from_str_invalid_index_not_hardened() {
        assert!(matches!(
            SUT::try_from(
                HDPath::from_str("m/44H/1022H/1H/525/1460H/0").unwrap()
            ),
            Err(CommonError::NotAllComponentsAreHardened)
        ));
    }
}
