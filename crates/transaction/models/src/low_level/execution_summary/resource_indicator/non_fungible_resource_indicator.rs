use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuaranteedOrPredicted<T> {
    Guaranteed(T),
    Predicted(Predicted<T>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicted<T> {
    pub value: T,
    pub instruction_index: u64,
}

impl<R, S> From<RetEitherGuaranteedOrPredicted<R>> for GuaranteedOrPredicted<S>
where
    S: From<R>,
{
    fn from(ret: RetEitherGuaranteedOrPredicted<R>) -> Self {
        match ret {
            RetEitherGuaranteedOrPredicted::Guaranteed(value) => {
                GuaranteedOrPredicted::Guaranteed(S::from(value))
            }
            RetEitherGuaranteedOrPredicted::Predicted(predicted) => {
                GuaranteedOrPredicted::Predicted(Predicted::from(predicted))
            }
        }
    }
}

impl<R, S> From<RetTracked<R>> for Predicted<S>
where
    S: From<R>,
{
    fn from(tracked: RetTracked<R>) -> Self {
        Self {
            value: S::from(tracked.value),
            instruction_index: *tracked.created_at.value() as u64,
        }
    }
}

impl<R, S> From<RetTracked<IndexSet<R>>> for Predicted<IndexSet<S>>
where
    S: From<R>,
{
    fn from(tracked: RetTracked<R>) -> Self {
        Self {
            value: S::from(tracked.value),
            instruction_index: *tracked.created_at.value() as u64,
        }
    }
}

pub type NonFungibleResourceIndicator = GuaranteedOrPredicted<IndexSet<NonFungibleLocalId>>;
pub type FunibleResourceIndicator = GuaranteedOrPredicted<Decimal>;


impl NonFungibleResourceIndicator {
    pub fn ids(&self) -> IndexSet<NonFungibleLocalId> {
        match self {
            GuaranteedOrPredicted::Guaranteed(ids) => ids.clone(),
            GuaranteedOrPredicted::Predicted(predicted_ids) => predicted_ids
                .value
                .clone()
        }
    }
}

impl HasSampleValues for NonFungibleResourceIndicator {
    fn sample() -> Self {
        let idx: IndexSet<ScryptoNonFungibleLocalId>;
        let y = IndexSet<NonFungibleLocalId>::from
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
