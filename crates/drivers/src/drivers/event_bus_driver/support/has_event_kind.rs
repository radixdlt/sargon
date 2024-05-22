use crate::prelude::*;

/// Event which can be turned into a discriminant `EventKind`,
/// which does not have any associated values.
pub trait HasEventKind {
    fn kind(&self) -> EventKind;
}
