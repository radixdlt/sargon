use crate::prelude::*;
use sargon::SimpleNonFungibleResourceBounds as InternalSimpleNonFungibleResourceBounds;

/// Represents the bounds for a simple non-fungible resource, which can be either exact or not exact.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SimpleNonFungibleResourceBounds {
    pub certain_ids: Vec<NonFungibleLocalId>,
    pub additional_amount: Option<SimpleCountedResourceBounds>,
}
