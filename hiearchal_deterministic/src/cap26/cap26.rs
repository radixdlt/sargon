use std::str::FromStr;

use slip10::BIP32Path;
use wallet_kit_common::network_id::NetworkID;

use crate::bip32::{
    hd_path::HDPath,
    hd_path_component::{HDPathComponent, HDPathValue},
};

use super::{
    cap26_entity_kind::CAP26EntityKind, cap26_error::CAP26Error, cap26_key_kind::CAP26KeyKind,
};

pub trait CAP26Repr: Sized {
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

    fn parse_try_map<T, F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        try_map: F,
    ) -> Result<T, CAP26Error>
    where
        F: Fn(HDPathValue) -> Result<T, CAP26Error>,
    {
        let got = &path[index];
        try_map(got.value())
    }

    fn parse<F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        expected: HDPathComponent,
        err: F,
    ) -> Result<&HDPathComponent, CAP26Error>
    where
        F: Fn(HDPathValue) -> CAP26Error,
    {
        let got = &path[index];
        if got != &expected {
            return Err(err(got.value()));
        }
        Ok(got)
    }

    fn from_str(s: &str) -> Result<Self, CAP26Error> {
        use CAP26Error::*;
        let path = HDPath::from_str(s).map_err(|_| CAP26Error::InvalidBIP32Path(s.to_string()))?;
        if path.depth() != 6 {
            return Err(InvalidDepthOfCAP26Path);
        }
        let components = path.components();

        if !components.clone().iter().all(|c| c.is_hardened()) {
            return Err(NotAllComponentsAreHardened);
        }
        _ = Self::parse(
            components,
            0,
            HDPathComponent::bip44_purpose(),
            Box::new(|v| BIP44PurposeNotFound(v)),
        )?;

        _ = Self::parse(
            components,
            1,
            HDPathComponent::bip44_cointype(),
            Box::new(|v| CoinTypeNotFound(v)),
        )?;

        let network_id = Self::parse_try_map(
            components,
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

        let entity_kind = Self::parse_try_map(
            components,
            3,
            Box::new(|v| CAP26EntityKind::from_repr(v).ok_or(InvalidEntityKind(v))),
        )?;

        match Self::entity_kind() {
            Some(expected_entity_kind) => {
                if entity_kind != expected_entity_kind {
                    return Err(WrongEntityKind(entity_kind, expected_entity_kind));
                }
            }
            None => {}
        }

        let key_kind = Self::parse_try_map(
            components,
            4,
            Box::new(|v| CAP26KeyKind::from_repr(v).ok_or(InvalidKeyKind(v))),
        )?;

        let index = Self::parse_try_map(components, 4, Box::new(|v| Ok(v)))?;

        return Ok(Self::__with_path_and_components(
            path,
            network_id,
            entity_kind,
            key_kind,
            index,
        ));
    }
}
