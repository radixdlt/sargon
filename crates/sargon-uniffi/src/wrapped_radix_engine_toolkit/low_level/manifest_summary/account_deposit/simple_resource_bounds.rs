use crate::prelude::*;
use sargon::SimpleResourceBounds as InternalSimpleResourceBounds;

/// Represents the bounds for a simple resource, which can be either fungible or non-fungible.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SimpleResourceBounds {
    Fungible {
        bounds: SimpleFungibleResourceBounds,
    },
    NonFungible {
        bounds: SimpleNonFungibleResourceBounds,
    },
}
