use crate::prelude::*;
use sargon::EntityKind as InternalEntityKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EntityKind {
    Account,
    Persona,
}

impl From<InternalEntityKind> for EntityKind {
    fn from(value: InternalEntityKind) -> Self {
        match value {
            InternalEntityKind::Account => Self::Account,
            InternalEntityKind::Persona => Self::Persona,
        }
    }
}

impl Into<InternalEntityKind> for EntityKind {
    fn into(self) -> InternalEntityKind {
        match self {
            EntityKind::Account => InternalEntityKind::Account,
            EntityKind::Persona => InternalEntityKind::Persona,
        }
    }
}