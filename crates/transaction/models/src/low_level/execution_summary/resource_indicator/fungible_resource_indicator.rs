use crate::prelude::*;

pub type FungibleResourceIndicator = GuaranteedOrPredicted<Decimal>;
pub(crate) type RetFungibleResourceIndicator =
    RetEitherGuaranteedOrPredicted<ScryptoDecimal192>;

impl From<RetFungibleResourceIndicator> for FungibleResourceIndicator {
    fn from(value: RetFungibleResourceIndicator) -> Self {
        match value {
            RetEitherGuaranteedOrPredicted::Guaranteed(value) => {
                Self::Guaranteed(value.into())
            }
            RetEitherGuaranteedOrPredicted::Predicted(value) => {
                Self::Predicted(value.into())
            }
        }
    }
}

pub type PredictedDecimal = Predicted<Decimal>;

impl From<RetTracked<ScryptoDecimal192>> for PredictedDecimal {
    fn from(value: RetTracked<ScryptoDecimal192>) -> Self {
        Self::new(value.value, *value.created_at.value() as u64)
    }
}

impl HasSampleValues for PredictedDecimal {
    fn sample() -> Self {
        Self::new(Decimal::one(), 0)
    }

    fn sample_other() -> Self {
        Self::new(Decimal::three(), 1)
    }
}

impl HasSampleValues for FungibleResourceIndicator {
    fn sample() -> Self {
        Self::new_guaranteed(Decimal::one())
    }

    fn sample_other() -> Self {
        Self::new_predicted(Decimal::two(), 0)
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::types::InstructionIndex;

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
        let ret = RetFungibleResourceIndicator::new_guaranteed(1.into());
        assert_eq!(SUT::from(ret), SUT::sample())
    }

    #[test]
    fn get_amount() {
        assert_eq!(SUT::sample().get_value(), Decimal192::from(1));
        assert_eq!(SUT::sample_other().get_value(), Decimal192::from(2));
    }

    #[test]
    fn from_ret_predicted() {
        let ret = RetFungibleResourceIndicator::new_predicted(
            2.into(),
            InstructionIndex::of(0),
        );
        assert_eq!(SUT::from(ret), SUT::new_predicted(2, 0))
    }
}
