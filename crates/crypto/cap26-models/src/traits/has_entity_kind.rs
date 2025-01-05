use crate::prelude::*;
pub trait HasEntityKind {
    fn entity_kind() -> CAP26EntityKind;
}

pub trait HasEntityKindObjectSafe {
    fn get_entity_kind(&self) -> CAP26EntityKind;
}

impl<T: HasEntityKind> HasEntityKindObjectSafe for T {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        T::entity_kind()
    }
}
