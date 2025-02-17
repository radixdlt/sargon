use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Deref)]
struct EntitiesNotSignedFor(Vec<EntityNotSignedFor>); // want IndexSet, but Item is not StdHash.
impl From<Vec<EntityNotSignedFor>> for EntitiesNotSignedFor {
    fn from(v: Vec<EntityNotSignedFor>) -> Self {
        Self(v)
    }
}
