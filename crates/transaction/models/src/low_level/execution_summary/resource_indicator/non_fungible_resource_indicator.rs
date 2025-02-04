use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonFungibleResourceIndicator {
    Guaranteed { ids: IndexSet<NonFungibleLocalId> },
    Predicted { predicted_ids: PredictedNonFungibleLocalIds },
}

impl NonFungibleResourceIndicator {
    pub fn ids(&self) -> Vec<NonFungibleLocalId> {
        match self {
            NonFungibleResourceIndicator::Guaranteed { ids } => ids.iter().cloned().collect(),
            NonFungibleResourceIndicator::Predicted { predicted_ids } => {
                predicted_ids.value.iter().cloned().collect()
            }
        }
    }
}

type RetNonFungibleResourceIndicator = RetEitherGuaranteedOrPredicted<IndexSet<ScryptoNonFungibleLocalId>>;
impl From<RetNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn from(value: RetNonFungibleResourceIndicator) -> Self {
        match value {
            RetNonFungibleResourceIndicator::Guaranteed(ids) => {
                Self::Guaranteed {
                    ids: ids.into_iter().map(NonFungibleLocalId::from).collect(),
                }
            }
            RetNonFungibleResourceIndicator::Predicted(predicted_ids) => {
                Self::Predicted {
                    predicted_ids: PredictedNonFungibleLocalIds::from_ret(predicted_ids),
                }
            }
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
    fn get_ids() {
        let ids = vec![
            NonFungibleLocalId::random(),
            NonFungibleLocalId::random(),
            NonFungibleLocalId::random(),
        ];

        assert_eq!(
            SUT::by_all(
                PredictedDecimal::sample(),
                PredictedNonFungibleLocalIds::new(ids.clone(), 0)
            )
            .ids(),
            ids.clone()
        );

        assert_eq!(
            SUT::by_amount(
                0,
                PredictedNonFungibleLocalIds::new(ids.clone(), 0)
            )
            .ids(),
            ids.clone()
        );

        assert_eq!(SUT::by_ids(ids.clone()).ids(), ids);
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
