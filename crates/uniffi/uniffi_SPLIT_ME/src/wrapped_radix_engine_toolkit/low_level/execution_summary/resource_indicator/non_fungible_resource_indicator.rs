use crate::prelude::*;
use sargon::NonFungibleResourceIndicator as InternalNonFungibleResourceIndicator;

#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleResourceIndicator {
    Guaranteed {
        ids: Vec<NonFungibleLocalId>,
    },
    Predicted {
        predicted_ids: PredictedNonFungibleLocalIds,
    },
}

impl NonFungibleResourceIndicator {
    pub fn into_internal(&self) -> InternalNonFungibleResourceIndicator {
        self.clone().into()
    }
}

impl From<InternalNonFungibleResourceIndicator>
    for NonFungibleResourceIndicator
{
    fn from(value: InternalNonFungibleResourceIndicator) -> Self {
        match value {
            InternalNonFungibleResourceIndicator::Guaranteed(ids) => {
                Self::Guaranteed {
                    ids: ids.into_type(),
                }
            }
            InternalNonFungibleResourceIndicator::Predicted(predicted_ids) => {
                Self::Predicted {
                    predicted_ids: PredictedNonFungibleLocalIds::from(
                        predicted_ids,
                    ),
                }
            }
        }
    }
}

impl From<NonFungibleResourceIndicator>
    for InternalNonFungibleResourceIndicator
{
    fn from(val: NonFungibleResourceIndicator) -> Self {
        match val {
            NonFungibleResourceIndicator::Guaranteed { ids } => {
                InternalNonFungibleResourceIndicator::Guaranteed(
                    ids.into_internal(),
                )
            }
            NonFungibleResourceIndicator::Predicted { predicted_ids } => {
                InternalNonFungibleResourceIndicator::Predicted(
                    predicted_ids.into_internal(),
                )
            }
        }
    }
}

decl_conversion_tests_for!(NonFungibleResourceIndicator);

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
    indicator.into_internal().get_value().into_type()
}
