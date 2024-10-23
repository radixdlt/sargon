use crate::prelude::*;
use sargon::SimpleNonFungibleResourceBounds as InternalSimpleNonFungibleResourceBounds;

/// Represents the bounds for a simple non-fungible resource, which can be either exact or not exact.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SimpleNonFungibleResourceBounds {
    Exact {
        amount: Decimal,
        certain_ids: Vec<NonFungibleLocalId>,
    },
    NotExact {
        certain_ids: Vec<NonFungibleLocalId>,
        lower_bound: LowerBound,
        upper_bound: UpperBound,
        allowed_ids: AllowedIds,
    },
}
