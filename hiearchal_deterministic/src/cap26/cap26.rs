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
    fn __with_path_and_components(
        path: HDPath,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self;

    fn do_parse<F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        expected: HDPathComponent,
        err: F,
    ) -> Result<&HDPathComponent, CAP26Error>
    where
        F: Fn(usize, &HDPathComponent) -> CAP26Error,
    {
        let got = &path[index];
        if got != &expected {
            return Err(err(index, got));
        }
        Ok(got)
    }

    fn from_str(s: &str) -> Result<Self, CAP26Error> {
        use CAP26Error::*;
        let path = HDPath::from_str(s).map_err(|_| CAP26Error::InvalidBIP32Path(s.to_string()))?;
        if path.depth() != 6 {
            return Err(CAP26Error::InvalidDepthOfCAP26Path);
        }
        let components = path.components();

        let parse = |index: usize,
                     expected: HDPathComponent,
                     err: Box<dyn Fn(HDPathValue) -> CAP26Error>|
         -> Result<&HDPathComponent, CAP26Error> {
            Self::do_parse(components, index, expected, |i, v| err(v.value()))
        };

        // if !path.components().iter().all(|c| c.is_hardened()) {
        //     return Err(CAP26Error::NotAllComponentsAreHardened);
        // }
        // if path[0] != HDPathComponent::bip44_purpose() {
        //     return Err(CAP26Error::BIP44PurposeNotFoundAtIndex1(path[0].value()));
        // }
        _ = parse(
            0,
            HDPathComponent::bip44_purpose(),
            Box::new(|v| BIP44PurposeNotFoundAtIndex1(v)),
        )?;

        // if path[1] != HDPathComponent::bip44_cointype() {
        //     return Err(CAP26Error::CoinTypeNotFoundAtIndex2(path[1].value()));
        // }
        // if path[2].value() >= (u8::MAX as u32) {
        //     return Err(CAP26Error::InvalidNetworkIDExceedsLimit(path[2].value()));
        // }
        // let entity_kind_value = path[3].value();
        // let Some(entity_kind) = CAP26EntityKind::from_repr(entity_kind_value) else {
        //     return Err(CAP26Error::InvalidEntityKind(path[3].value()));
        // };

        // let key_kind_value = path[4].value();
        // let Some(key_kind) = CAP26KeyKind::from_repr(key_kind_value) else {
        //     return Err(CAP26Error::InvalidKeyKind(path[4].value()));
        // };

        // return Ok(Self(path));
        todo!()
    }
}
