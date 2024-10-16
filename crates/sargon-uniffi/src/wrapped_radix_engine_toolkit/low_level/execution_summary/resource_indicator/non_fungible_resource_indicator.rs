use crate::prelude::*;
use sargon::NonFungibleResourceIndicator as InternalNonFungibleResourceIndicator;

#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Enum)]
pub enum NonFungibleResourceIndicator {
    ByAll {
        predicted_amount: PredictedDecimal,
        predicted_ids: PredictedNonFungibleLocalIds,
    },
    ByAmount {
        amount: Decimal192,
        predicted_ids: PredictedNonFungibleLocalIds,
    },
    ByIds {
        ids: Vec<NonFungibleLocalId>,
    },
}

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample(
) -> NonFungibleResourceIndicator {
    InternalNonFungibleResourceIndicator::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample_other(
) -> NonFungibleResourceIndicator {
    InternalNonFungibleResourceIndicator::sample_other().into()
}

#[uniffi::export]
pub fn non_fungible_resource_indicator_get_ids(
    indicator: &NonFungibleResourceIndicator,
) -> Vec<NonFungibleLocalId> {
    indicator.into_internal().ids().into_type()
}
