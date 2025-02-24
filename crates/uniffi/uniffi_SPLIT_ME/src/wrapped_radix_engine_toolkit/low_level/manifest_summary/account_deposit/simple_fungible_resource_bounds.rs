use crate::prelude::*;
use sargon::SimpleCountedResourceBounds as InternalSimpleCountedResourceBounds;

/// Represents the bounds for a simple fungible resource, which can
/// be exact, at most, at least, between, or unknown amount.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SimpleCountedResourceBounds {
    Exact {
        amount: Decimal,
    },
    AtMost {
        amount: Decimal,
    },
    AtLeast {
        amount: Decimal,
    },
    Between {
        min_amount: Decimal,
        max_amount: Decimal,
    },
    UnknownAmount,
}
