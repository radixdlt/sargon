use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
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

impl NonFungibleResourceIndicator {
    pub fn by_all(
        predicted_amount: PredictedDecimal,
        predicted_ids: PredictedNonFungibleLocalIds,
    ) -> Self {
        Self::ByAll {
            predicted_amount,
            predicted_ids,
        }
    }
    pub fn by_amount(
        amount: impl Into<Decimal192>,
        predicted_ids: PredictedNonFungibleLocalIds,
    ) -> Self {
        Self::ByAmount {
            amount: amount.into(),
            predicted_ids,
        }
    }
    pub fn by_ids(ids: impl IntoIterator<Item = NonFungibleLocalId>) -> Self {
        Self::ByIds {
            ids: ids.into_iter().collect(),
        }
    }
}

impl From<RetNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn from(value: RetNonFungibleResourceIndicator) -> Self {
        match value {
            RetNonFungibleResourceIndicator::ByAll {
                predicted_amount,
                predicted_ids,
            } => Self::ByAll {
                predicted_amount: PredictedDecimal::from_ret(predicted_amount),
                predicted_ids: predicted_ids.into(),
            },
            RetNonFungibleResourceIndicator::ByAmount {
                amount,
                predicted_ids,
            } => Self::ByAmount {
                amount: amount.into(),
                predicted_ids: predicted_ids.into(),
            },
            RetNonFungibleResourceIndicator::ByIds(ids) => Self::ByIds {
                ids: ids.into_iter().map(NonFungibleLocalId::from).collect(),
            },
        }
    }
}

impl HasSampleValues for NonFungibleResourceIndicator {
    fn sample() -> Self {
        Self::by_amount(3, PredictedNonFungibleLocalIds::sample())
    }

    fn sample_other() -> Self {
        Self::by_all(
            PredictedDecimal::sample(),
            PredictedNonFungibleLocalIds::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::transaction_types::Predicted;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceIndicator;

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
    fn from_ret_by_amount() {
        let ret = RetNonFungibleResourceIndicator::ByAmount {
            amount: 3.into(),
            predicted_ids: Predicted {
                value: [
                    NonFungibleLocalId::sample(),
                    NonFungibleLocalId::sample_other(),
                ]
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
                instruction_index: 0,
            },
        };
        assert_eq!(SUT::from(ret), SUT::sample());
    }

    #[test]
    fn from_ret_by_ids() {
        let ids = vec![
            NonFungibleLocalId::sample(),
            NonFungibleLocalId::sample_other(),
        ];

        let ret = RetNonFungibleResourceIndicator::ByIds(
            ids.clone()
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
        );

        assert_eq!(SUT::from(ret), SUT::by_ids(ids));
    }
}
