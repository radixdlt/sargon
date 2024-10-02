use crate::prelude::*;

/// Detailed information on the amount of cost units consumed.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct FeeSummary {
    pub execution_cost: Decimal192,
    pub finalization_cost: Decimal192,
    pub royalty_cost: Decimal192,
    pub storage_expansion_cost: Decimal192,
}

impl FeeSummary {
    pub fn new(
        execution_cost: impl Into<Decimal192>,
        finalization_cost: impl Into<Decimal192>,
        storage_expansion_cost: impl Into<Decimal192>,
        royalty_cost: impl Into<Decimal192>,
    ) -> Self {
        Self {
            execution_cost: execution_cost.into(),
            finalization_cost: finalization_cost.into(),
            storage_expansion_cost: storage_expansion_cost.into(),
            royalty_cost: royalty_cost.into(),
        }
    }
}

impl From<RetFeeSummary> for FeeSummary {
    fn from(value: RetFeeSummary) -> Self {
        Self::new(
            value.execution_cost,
            value.finalization_cost,
            value.storage_expansion_cost,
            value.royalty_cost,
        )
    }
}

impl HasSampleValues for FeeSummary {
    fn sample() -> Self {
        Self::new(
            "0.092499".parse::<Decimal>().unwrap(),
            "0.02100205".parse::<Decimal>().unwrap(),
            "0.08459091041".parse::<Decimal>().unwrap(),
            0,
        )
    }

    fn sample_other() -> Self {
        FeeSummary::new(
            "0.1495719".parse::<Decimal>().unwrap(),
            "0.1557717".parse::<Decimal>().unwrap(),
            "0.3290176335".parse::<Decimal>().unwrap(),
            10,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FeeSummary;

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
    fn from_ret() {
        assert_eq!(
            SUT::from(RetFeeSummary {
                execution_cost: "0.092499"
                    .parse::<ScryptoDecimal192>()
                    .unwrap(),
                finalization_cost: "0.02100205"
                    .parse::<ScryptoDecimal192>()
                    .unwrap(),
                storage_expansion_cost: "0.08459091041"
                    .parse::<ScryptoDecimal192>()
                    .unwrap(),
                royalty_cost: 0.into(),
            }),
            SUT::sample()
        );
    }
}
