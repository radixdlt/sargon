use radix_engine_toolkit::types::EitherGuaranteedOrPredicted;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

impl FungibleResourceIndicator {
    pub fn guaranteed(decimal: impl Into<Decimal>) -> Self {
        Self::Guaranteed {
            decimal: decimal.into(),
        }
    }
    pub fn predicted(
        decimal: impl Into<Decimal>,
        instruction_index: u64,
    ) -> Self {
        Self::Predicted {
            predicted_decimal: PredictedDecimal::new(
                decimal,
                instruction_index,
            ),
        }
    }
}

impl FungibleResourceIndicator {
    pub fn get_amount(&self) -> Decimal192 {
        match self {
            FungibleResourceIndicator::Guaranteed { decimal } => *decimal,
            FungibleResourceIndicator::Predicted { predicted_decimal } => {
                predicted_decimal.value
            }
        }
    }
}

pub type RetFungibleResourceIndicator = EitherGuaranteedOrPredicted<ScryptoDecimal192>;

impl From<RetFungibleResourceIndicator> for FungibleResourceIndicator {
    fn from(value: RetFungibleResourceIndicator) -> Self {
        match value {
            RetFungibleResourceIndicator::Guaranteed(decimal) => {
                Self::Guaranteed {
                    decimal: decimal.into(),
                }
            }
            RetFungibleResourceIndicator::Predicted(predicted_decimal) => {
                Self::Predicted {
                    predicted_decimal: PredictedDecimal::from_ret(
                        predicted_decimal,
                    ),
                }
            }
        }
    }
}

impl HasSampleValues for FungibleResourceIndicator {
    fn sample() -> Self {
        Self::guaranteed(1)
    }

    fn sample_other() -> Self {
        Self::predicted(2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FungibleResourceIndicator;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_ret_guaranteed() {
        let ret = RetFungibleResourceIndicator::Guaranteed(1.into());
        assert_eq!(SUT::from(ret), SUT::sample())
    }

    #[test]
    fn get_amount() {
        assert_eq!(SUT::sample().get_amount(), Decimal192::from(1));
        assert_eq!(SUT::sample_other().get_amount(), Decimal192::from(2));
    }

    #[test]
    fn from_ret_predicted() {
        let ret = RetFungibleResourceIndicator::Predicted(Predicted {
            value: 2.into(),
            instruction_index: 0,
        });
        assert_eq!(SUT::from(ret), SUT::sample_other())
    }
}
