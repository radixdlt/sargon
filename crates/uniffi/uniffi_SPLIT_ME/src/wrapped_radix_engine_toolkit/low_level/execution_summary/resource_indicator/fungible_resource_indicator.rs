use crate::prelude::*;
use sargon::FungibleResourceIndicator as InternalFungibleResourceIndicator;

#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

impl FungibleResourceIndicator {
    pub fn into_internal(&self) -> InternalFungibleResourceIndicator {
        self.clone().into()
    }
}

impl From<InternalFungibleResourceIndicator> for FungibleResourceIndicator {
    fn from(value: InternalFungibleResourceIndicator) -> Self {
        match value {
            InternalFungibleResourceIndicator::Guaranteed(decimal) => {
                Self::Guaranteed {
                    decimal: decimal.into(),
                }
            }
            InternalFungibleResourceIndicator::Predicted(predicted_decimal) => {
                Self::Predicted {
                    predicted_decimal: PredictedDecimal::from(
                        predicted_decimal,
                    ),
                }
            }
        }
    }
}

impl From<FungibleResourceIndicator> for InternalFungibleResourceIndicator {
    fn from(val: FungibleResourceIndicator) -> Self {
        match val {
            FungibleResourceIndicator::Guaranteed { decimal } => {
                InternalFungibleResourceIndicator::Guaranteed(decimal.into())
            }
            FungibleResourceIndicator::Predicted { predicted_decimal } => {
                InternalFungibleResourceIndicator::Predicted(
                    predicted_decimal.into(),
                )
            }
        }
    }
}

decl_conversion_tests_for!(FungibleResourceIndicator);

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
    indicator.into_internal().get_value().into()
}
