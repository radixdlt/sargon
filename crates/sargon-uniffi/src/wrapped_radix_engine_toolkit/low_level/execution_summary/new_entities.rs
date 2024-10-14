use crate::prelude::*;
use sargon::NewEntities as InternalNewEntities;

/// Information on the global entities created in the transaction.
#[derive(Clone, Default, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct NewEntities {
    pub metadata: HashMap<ResourceAddress, NewlyCreatedResource>,
}
