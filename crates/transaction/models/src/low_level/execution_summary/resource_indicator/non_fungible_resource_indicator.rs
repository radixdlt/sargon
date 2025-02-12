use crate::prelude::*;

pub type NonFungibleLocalIds = Vec<NonFungibleLocalId>;
pub type NonFungibleResourceIndicator =
    GuaranteedOrPredicted<NonFungibleLocalIds>;
pub type PredictedNonFungibleLocalIds = Predicted<NonFungibleLocalIds>;

type RetNonFungibleResourceIndicator =
    RetEitherGuaranteedOrPredicted<IndexSet<ScryptoNonFungibleLocalId>>;
impl From<RetNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn from(value: RetNonFungibleResourceIndicator) -> Self {
        match value {
            RetEitherGuaranteedOrPredicted::Guaranteed(value) => {
                Self::Guaranteed(
                    value
                        .into_iter()
                        .map(NonFungibleLocalId::from)
                        .collect_vec(),
                )
            }
            RetEitherGuaranteedOrPredicted::Predicted(value) => {
                Self::Predicted(PredictedNonFungibleLocalIds::from(value))
            }
        }
    }
}

type RetPredictedNonFungibleLocalIds =
    RetTracked<IndexSet<ScryptoNonFungibleLocalId>>;
impl From<RetPredictedNonFungibleLocalIds> for PredictedNonFungibleLocalIds {
    fn from(value: RetPredictedNonFungibleLocalIds) -> Self {
        Self::new(
            value
                .value
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect_vec(),
            *value.created_at.value() as u64,
        )
    }
}

impl HasSampleValues for NonFungibleResourceIndicator {
    fn sample() -> Self {
        Self::new_guaranteed(NonFungibleLocalIds::sample())
    }

    fn sample_other() -> Self {
        Self::new_predicted(NonFungibleLocalIds::sample_other(), 0)
    }
}

impl HasSampleValues for PredictedNonFungibleLocalIds {
    fn sample() -> Self {
        Self::new(vec![NonFungibleLocalId::sample()], 0)
    }

    fn sample_other() -> Self {
        Self::new(vec![NonFungibleLocalId::sample_other()], 0)
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::types::InstructionIndex;

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

        assert_eq!(SUT::new_guaranteed(ids.clone()).get_value(), ids.clone());

        assert_eq!(SUT::new_predicted(ids.clone(), 0).get_value(), ids.clone());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_ret_guaranteed() {
        let ret = RetNonFungibleResourceIndicator::new_guaranteed(
            SUT::sample()
                .get_value()
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
        );
        assert_eq!(SUT::from(ret), SUT::sample());
    }

    #[test]
    fn from_ret_predicted() {
        let ids = vec![
            NonFungibleLocalId::sample(),
            NonFungibleLocalId::sample_other(),
        ];

        let ret = RetNonFungibleResourceIndicator::new_predicted(
            ids.clone()
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
            InstructionIndex::of(0),
        );

        assert_eq!(SUT::from(ret), SUT::new_predicted(ids, 0));
    }
}
