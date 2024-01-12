use crate::prelude::*;

// TODO: Merge trait IsEntityPath into trait EntityCAP26Path ?
pub trait IsEntityPath: EntityCAP26Path {
    fn network_id(&self) -> NetworkID;
    fn key_kind(&self) -> CAP26KeyKind;
    fn index(&self) -> HDPathValue;
}
// TODO: Remove - not used???
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
