use wallet_kit_common::NetworkID;

use crate::{
    bip32::HDPathValue,
    cap26::{cap26_key_kind::CAP26KeyKind, cap26_repr::CAP26Repr},
};

pub trait IsEntityPath: CAP26Repr {
    fn network_id(&self) -> NetworkID;
    fn key_kind(&self) -> CAP26KeyKind;
    fn index(&self) -> HDPathValue;
}

pub trait HasEntityPath<Path: IsEntityPath> {
    fn path(&self) -> Path;

    #[cfg(not(tarpaulin_include))] // false negative
    fn network_id(&self) -> NetworkID {
        self.path().network_id()
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn key_kind(&self) -> CAP26KeyKind {
        self.path().key_kind()
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn index(&self) -> HDPathValue {
        self.path().index()
    }
}
