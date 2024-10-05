use crate::prelude::*;
use sargon::FungibleResourceIndicator as InternalFungibleResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
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
    indicator.into::<InternalFungibleResourceIndicator>.get_amount().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FungibleResourceIndicator;

    #[test]
    fn inequality() {
        assert_ne!(
            new_fungible_resource_indicator_sample(),
            new_fungible_resource_indicator_sample_other()
        );
    }

    #[test]
    fn get_amount() {
        let sut = SUT::sample();
        assert_eq!(
            sut.get_amount(),
            fungible_resource_indicator_get_amount(&sut)
        );
    }
}
