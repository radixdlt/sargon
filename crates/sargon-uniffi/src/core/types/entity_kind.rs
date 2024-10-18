use crate::prelude::*;
use sargon::EntityKind as InternalEntityKind;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum EntityKind {
    Account,
    Persona,
}
