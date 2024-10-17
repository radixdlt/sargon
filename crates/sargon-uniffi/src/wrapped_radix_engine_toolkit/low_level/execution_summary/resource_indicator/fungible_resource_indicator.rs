use crate::prelude::*;
use sargon::FungibleResourceIndicator as InternalFungibleResourceIndicator;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

#[uniffi::export]
pub fn new_fungible_resource_indicator_sample() -> FungibleResourceIndicator {
    InternalFungibleResourceIndicator::sample().into()
}

#[uniffi::export]
pub fn new_fungible_resource_indicator_sample_other(
) -> FungibleResourceIndicator {
    InternalFungibleResourceIndicator::sample_other().into()
}

#[uniffi::export]
pub fn fungible_resource_indicator_get_amount(
    indicator: &FungibleResourceIndicator,
) -> Decimal192 {
    indicator.into_internal().get_amount().into()
}
