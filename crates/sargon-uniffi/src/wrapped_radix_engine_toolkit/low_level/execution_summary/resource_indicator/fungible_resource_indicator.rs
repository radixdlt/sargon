use crate::prelude::*;
use sargon::FungibleResourceIndicator as InternalFungibleResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, InternalConersion, uniffi::Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

impl From<InternalFungibleResourceIndicator> for FungibleResourceIndicator {
    fn from(value: InternalFungibleResourceIndicator) -> Self {
        match value {
            InternalFungibleResourceIndicator::Guaranteed { decimal } => FungibleResourceIndicator::Guaranteed { decimal: decimal.into() },
            InternalFungibleResourceIndicator::Predicted { predicted_decimal } => FungibleResourceIndicator::Predicted { predicted_decimal: predicted_decimal.into() },
        }
    }
}

impl Into<InternalFungibleResourceIndicator> for FungibleResourceIndicator {
    fn into(self) -> InternalFungibleResourceIndicator {
        match self {
            FungibleResourceIndicator::Guaranteed { decimal } => InternalFungibleResourceIndicator::Guaranteed { decimal: decimal.into() },
            FungibleResourceIndicator::Predicted { predicted_decimal } => InternalFungibleResourceIndicator::Predicted { predicted_decimal: predicted_decimal.into() },
        }
    }
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

