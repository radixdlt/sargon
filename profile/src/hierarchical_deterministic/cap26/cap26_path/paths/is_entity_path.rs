use crate::{CAP26KeyKind, CAP26Path, CAP26Repr, HDPathError, HDPathValue, NetworkID};

pub trait IsEntityPath: CAP26Repr + Into<CAP26Path> + TryFrom<CAP26Path> {
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
