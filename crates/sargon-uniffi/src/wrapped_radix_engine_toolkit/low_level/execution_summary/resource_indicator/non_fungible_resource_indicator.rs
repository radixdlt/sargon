use crate::prelude::*;
use sargon::NonFungibleResourceIndicator as InternalNonFungibleResourceIndicator;

#[derive(Clone,  PartialEq, Eq, InternalConversion, uniffi::Enum)]
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

impl From<InternalNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn from(value: InternalNonFungibleResourceIndicator) -> Self {
        match value {
            InternalNonFungibleResourceIndicator::ByAll {
                predicted_amount,
                predicted_ids,
            } => NonFungibleResourceIndicator::ByAll {
                predicted_amount: predicted_amount.into(),
                predicted_ids: predicted_ids.into(),
            },
            InternalNonFungibleResourceIndicator::ByAmount {
                amount,
                predicted_ids,
            } => NonFungibleResourceIndicator::ByAmount {
                amount: amount.into(),
                predicted_ids: predicted_ids.into(),
            },
            InternalNonFungibleResourceIndicator::ByIds { ids } => {
                NonFungibleResourceIndicator::ByIds {
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
}

impl Into<InternalNonFungibleResourceIndicator> for NonFungibleResourceIndicator {
    fn into(self) -> InternalNonFungibleResourceIndicator {
        match self {
            NonFungibleResourceIndicator::ByAll {
                predicted_amount,
                predicted_ids,
            } => InternalNonFungibleResourceIndicator::ByAll {
                predicted_amount: predicted_amount.into(),
                predicted_ids: predicted_ids.into(),
            },
            NonFungibleResourceIndicator::ByAmount {
                amount,
                predicted_ids,
            } => InternalNonFungibleResourceIndicator::ByAmount {
                amount: amount.into(),
                predicted_ids: predicted_ids.into(),
            },
            NonFungibleResourceIndicator::ByIds { ids } => {
                InternalNonFungibleResourceIndicator::ByIds {
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
        }
    }
}

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample(
) -> NonFungibleResourceIndicator {
    InternalNonFungibleResourceIndicator::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample_other(
) -> NonFungibleResourceIndicator {
    InternalNonFungibleResourceIndicator::sample_other().into()
}

#[uniffi::export]
pub fn non_fungible_resource_indicator_get_ids(
    indicator: &NonFungibleResourceIndicator,
) -> Vec<NonFungibleLocalId> {
    indicator.into_internal().ids().into_vec()
}

