use wallet_kit_common::network_id::NetworkID;

use crate::{
    bip32::hd_path_component::HDPathValue,
    cap26::{cap26_key_kind::CAP26KeyKind, cap26_repr::CAP26Repr},
};

pub trait IsEntityPath: CAP26Repr {
    fn network_id(&self) -> NetworkID;
    fn key_kind(&self) -> CAP26KeyKind;
    fn index(&self) -> HDPathValue;
}

pub trait HasEntityPath<Path: IsEntityPath> {
    fn path(&self) -> Path;

    fn network_id(&self) -> NetworkID {
        self.path().network_id()
    }
    fn key_kind(&self) -> CAP26KeyKind {
        self.path().key_kind()
    }
    fn index(&self) -> HDPathValue {
        self.path().index()
    }
}
