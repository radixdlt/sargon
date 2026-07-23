use crate::prelude::*;
use sargon::RequiredBadge as InternalRequiredBadge;

/// A badge that is required to authorize/validate the transaction execution.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum RequiredBadge {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal192,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

#[uniffi::export]
pub fn new_required_badge_sample() -> RequiredBadge {
    InternalRequiredBadge::sample().into()
}

#[uniffi::export]
pub fn new_required_badge_sample_other() -> RequiredBadge {
    InternalRequiredBadge::sample_other().into()
}

decl_conversion_tests_for!(RequiredBadge);
