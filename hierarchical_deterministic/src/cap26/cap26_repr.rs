use wallet_kit_common::network_id::NetworkID;

use crate::{
    bip32::{
        hd_path::HDPath,
        hd_path_component::{HDPathComponent, HDPathValue},
    },
    derivation::derivation::Derivation,
    hdpath_error::HDPathError,
};

use super::{cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind};

pub trait CAP26Repr: Derivation {
    fn entity_kind() -> Option<CAP26EntityKind> {
        Option::None
    }

    fn __with_path_and_components(
        path: HDPath,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self;

    fn from_str(s: &str) -> Result<Self, HDPathError> {
        use HDPathError::*;
        let (path, components) = HDPath::try_parse_base(s, HDPathError::InvalidDepthOfCAP26Path)?;
        if !components.clone().iter().all(|c| c.is_hardened()) {
            return Err(NotAllComponentsAreHardened);
        }
        if path.depth() != 6 {
            return Err(InvalidDepthOfCAP26Path);
        }
        let network_id = HDPath::parse_try_map(
            &components,
            2,
            Box::new(|v| {
                if v <= u8::MAX as u32 {
                    let d = v as u8;
                    NetworkID::from_repr(d).ok_or(UnsupportedNetworkID(d))
                } else {
                    Err(InvalidNetworkIDExceedsLimit(v))
                }
            }),
        )?;
        let entity_kind = HDPath::parse_try_map(
            &components,
            3,
            Box::new(|v| CAP26EntityKind::from_repr(v).ok_or(InvalidEntityKind(v))),
        )?;

        if let Some(expected_entity_kind) = Self::entity_kind() {
            if entity_kind != expected_entity_kind {
                return Err(WrongEntityKind(entity_kind, expected_entity_kind));
            }
        }

        let key_kind = HDPath::parse_try_map(
            &components,
            4,
            Box::new(|v| CAP26KeyKind::from_repr(v).ok_or(InvalidKeyKind(v))),
        )?;

        let index = HDPath::parse_try_map(&components, 5, Box::new(|v| Ok(v)))?;

        return Ok(Self::__with_path_and_components(
            path,
            network_id,
            entity_kind,
            key_kind,
            index,
        ));
    }

    fn new(network_id: NetworkID, key_kind: CAP26KeyKind, index: HDPathValue) -> Self {
        let entity_kind = Self::entity_kind().expect("GetID cannot be used with this constructor");
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
}
