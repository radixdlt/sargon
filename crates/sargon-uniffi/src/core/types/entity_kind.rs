use crate::prelude::*;
use sargon::EntityKind as InternalEntityKind;

#[derive( Clone, PartialEq, Eq, Hash,  uniffi::Enum)]
pub enum EntityKind {
    Account,
    Persona,
}