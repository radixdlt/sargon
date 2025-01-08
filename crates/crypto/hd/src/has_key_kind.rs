use crate::prelude::*;

pub trait HasKeyKind {
    fn key_kind() -> CAP26KeyKind;
}

pub trait HasKeyKindObjectSafe {
    fn get_key_kind(&self) -> CAP26KeyKind;
}

impl<T: HasKeyKind> HasKeyKindObjectSafe for T {
    fn get_key_kind(&self) -> CAP26KeyKind {
        T::key_kind()
    }
}
