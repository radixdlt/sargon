use crate::prelude::*;
use sargon::SimpleResourceBounds as InternalSimpleResourceBounds;

/// Represents the bounds for a simple resource, which can be either fungible or non_fungible.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SimpleResourceBounds {
    Fungible {
        resource_address: ResourceAddress,
        bounds: SimpleCountedResourceBounds,
    },
    NonFungible {
        resource_address: ResourceAddress,
        bounds: SimpleNonFungibleResourceBounds,
    },
}
