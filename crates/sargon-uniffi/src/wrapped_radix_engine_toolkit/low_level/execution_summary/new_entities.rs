use crate::prelude::*;
use sargon::NewEntities as InternalNewEntities;

/// Information on the global entities created in the transaction.
#[derive(Clone,  Default, PartialEq, Eq,  uniffi::Record)]
pub struct NewEntities {
    pub metadata: HashMap<ResourceAddress, NewlyCreatedResource>,
}

impl From<InternalNewEntities> for NewEntities {
    fn from(value: InternalNewEntities) -> Self {
        Self {
            metadata: value
                .metadata
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl Into<InternalNewEntities> for NewEntities {
    fn into(self) -> InternalNewEntities {
        InternalNewEntities {
            metadata: self
                .metadata
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}