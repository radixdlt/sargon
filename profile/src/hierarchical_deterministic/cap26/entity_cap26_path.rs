use crate::prelude::*;

const ENTITY_PATH_DEPTH: usize = 6;
pub trait EntityCAP26Path: Derivation + FromStr {
    fn entity_kind() -> CAP26EntityKind;

    fn __with_path_and_components(
        path: HDPath,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self;

    #[cfg(not(tarpaulin_include))] // false negative, this is in fact heavily tested.
    fn try_from_hdpath(hdpath: &HDPath) -> Result<Self> {
        let (path, components) =
            HDPath::try_parse_base_hdpath(hdpath, |v| CommonError::InvalidDepthOfCAP26Path {
                expected: ENTITY_PATH_DEPTH,
                found: v,
            })?;
        if !components.clone().iter().all(|c| c.is_hardened()) {
            return Err(CommonError::NotAllComponentsAreHardened);
        }
        if path.depth() != ENTITY_PATH_DEPTH {
            return Err(CommonError::InvalidDepthOfCAP26Path {
                expected: ENTITY_PATH_DEPTH,
                found: path.depth(),
            });
        }
        let network_id = HDPath::parse_try_map(
            &components,
            2,
            Box::new(|v| {
                if v <= u8::MAX as u32 {
                    let d = v as u8;
                    NetworkID::from_repr(d).ok_or(CommonError::UnsupportedNetworkID(d))
                } else {
                    Err(CommonError::InvalidNetworkIDExceedsLimit(v))
                }
            }),
        )?;
        let entity_kind = HDPath::parse_try_map(
            &components,
            3,
            Box::new(|v| CAP26EntityKind::from_repr(v).ok_or(CommonError::InvalidEntityKind(v))),
        )?;

        if entity_kind != Self::entity_kind() {
            return Err(CommonError::WrongEntityKind {
                expected: Self::entity_kind(),
                found: entity_kind,
            });
        }

        let key_kind = HDPath::parse_try_map(
            &components,
            4,
            Box::new(|v| CAP26KeyKind::from_repr(v).ok_or(CommonError::InvalidKeyKind(v))),
        )?;

        let index = HDPath::parse_try_map(&components, 5, Box::new(|v| Ok(v)))?;

        return Ok(Self::__with_path_and_components(path, network_id, entity_kind, key_kind, index));
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn from_bip32str(s: &str) -> Result<Self> {
        let (path, _) = HDPath::try_parse_base(s, |v| CommonError::InvalidDepthOfCAP26Path {
            expected: ENTITY_PATH_DEPTH,
            found: v,
        })?;
        Self::try_from_hdpath(&path)
    }

    fn new(network_id: NetworkID, key_kind: CAP26KeyKind, index: HDPathValue) -> Self {
        let entity_kind = Self::entity_kind();
        let c0 = HDPathComponent::bip44_purpose();
        let c1 = HDPathComponent::bip44_cointype();
        let c2 = HDPathComponent::harden(network_id.discriminant() as HDPathValue);
        let c3 = HDPathComponent::harden(entity_kind.discriminant());
        let c4 = HDPathComponent::harden(key_kind.discriminant());
        let c5 = HDPathComponent::harden(index);
        let components = vec![c0, c1, c2, c3, c4, c5];
        assert!(components.clone().iter().all(|c| c.is_hardened()));
        let path = HDPath::from_components(components);
        return Self::__with_path_and_components(path, network_id, entity_kind, key_kind, index);
    }

    fn new_mainnet_transaction_signing(index: HDPathValue) -> Self {
        Self::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, index)
    }
}
