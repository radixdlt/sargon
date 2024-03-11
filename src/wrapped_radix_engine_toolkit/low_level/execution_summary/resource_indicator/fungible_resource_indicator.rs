use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

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
        Self::Guaranteed {
            decimal: Decimal::one(),
        }
    }

    fn sample_other() -> Self {
        Self::Predicted {
            predicted_decimal: PredictedDecimal::new(Decimal::two(), 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::transaction_types::Predicted;

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
    fn from_ret_predicted() {
        let ret = RetFungibleResourceIndicator::Predicted(Predicted {
            value: 2.into(),
            instruction_index: 0,
        });
        assert_eq!(SUT::from(ret), SUT::sample_other())
    }
}
