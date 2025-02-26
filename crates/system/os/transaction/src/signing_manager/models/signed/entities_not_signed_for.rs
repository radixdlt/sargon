use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Deref)]
pub(crate) struct EntitiesNotSignedFor(pub(crate) Vec<EntityNotSignedFor>); // want IndexSet, but Item is not StdHash.

impl From<Vec<EntityNotSignedFor>> for EntitiesNotSignedFor {
    fn from(v: Vec<EntityNotSignedFor>) -> Self {
        Self(v)
    }
}
