use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Deref)]
pub(crate) struct EntitiesSignedFor(pub(crate) Vec<EntitySignedFor>); // want IndexSet, but Item is not StdHash.

impl From<Vec<EntitySignedFor>> for EntitiesSignedFor {
    fn from(v: Vec<EntitySignedFor>) -> Self {
        Self(v)
    }
}
