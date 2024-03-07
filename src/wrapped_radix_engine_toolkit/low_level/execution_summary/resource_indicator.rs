use crate::prelude::*;

use radix_engine::types::ResourceAddress as ScryptoResourceAddress;
use radix_engine_toolkit::transaction_types::FungibleResourceIndicator as RetFungibleResourceIndicator;
use radix_engine_toolkit::transaction_types::NonFungibleResourceIndicator as RetNonFungibleResourceIndicator;
use radix_engine_toolkit::transaction_types::Predicted as RetPredicted;
use radix_engine_toolkit::transaction_types::ResourceIndicator as RetResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum ResourceIndicator {
    Fungible {
        resource_address: ResourceAddress,
        indicator: FungibleResourceIndicator,
    },
    NonFungible {
        resource_address: ResourceAddress,
        indicator: NonFungibleResourceIndicator,
    },
}

impl From<(RetResourceIndicator, NetworkID)> for ResourceIndicator {
    fn from(value: (RetResourceIndicator, NetworkID)) -> Self {
        match value.0 {
            RetResourceIndicator::Fungible(
                resource_address,
                fungible_indicator,
            ) => Self::Fungible {
                resource_address: (resource_address, value.1).into(),
                indicator: fungible_indicator.into(),
            },
            RetResourceIndicator::NonFungible(
                resource_address,
                non_fungible_indicator,
            ) => Self::NonFungible {
                resource_address: (resource_address, value.1).into(),
                indicator: non_fungible_indicator.into(),
            },
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum FungibleResourceIndicator {
    Guaranteed { decimal: Decimal },
    Predicted { predicted_decimal: PredictedDecimal },
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleResourceIndicator {
    ByAll {
        predicted_amount: PredictedDecimal,
        predicted_ids: PredictedNonFungibleLocalIds,
    },
    ByAmount {
        amount: Decimal,
        predicted_ids: PredictedNonFungibleLocalIds,
    },
    ByIds {
        ids: Vec<NonFungibleLocalId>,
    },
}

use radix_engine::prelude::IndexSet;
use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;

type ScryptoNonFungibleLocalIds = IndexSet<ScryptoNonFungibleLocalId>;
type RetPredictedNonFungibleLocalIds = RetPredicted<ScryptoNonFungibleLocalIds>;

// Cannot be a `From` impl, since we neither own ScryptoNonFungibleLocalIds nor Vec,
// and a newtype for `Vec<NonFungibleLocalId>` is not worth it.
fn from_scrypto_ids(
    ids: ScryptoNonFungibleLocalIds,
) -> Vec<NonFungibleLocalId> {
    ids.into_iter()
        .map(Into::<NonFungibleLocalId>::into)
        .collect_vec()
}

impl From<RetPredictedNonFungibleLocalIds> for PredictedNonFungibleLocalIds {
    fn from(value: RetPredictedNonFungibleLocalIds) -> Self {
        Self {
            value: from_scrypto_ids(value.value),
            instruction_index: value.instruction_index as u64,
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
                ids: from_scrypto_ids(ids),
            },
        }
    }
}

use paste::*;

macro_rules! decl_predicted {
    (
        $(
            #[doc = $expr: expr]
        )*
        $wrapped_type:ty,
        $struct_name_suffix:ident
    ) => {
        paste! {

            $(
                #[doc = $expr]
            )*
            #[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
            pub struct [< Predicted $struct_name_suffix >] {
                pub value: $wrapped_type,
                pub instruction_index: u64,
            }

            impl [< Predicted $struct_name_suffix >] {
                pub fn from_ret<T>(ret_predicted: RetPredicted<T>) -> Self where T: Into<$wrapped_type> {
                    Self {
                        value: Into::<$wrapped_type>::into(ret_predicted.value),
                        instruction_index: ret_predicted.instruction_index as u64
                    }
                }
            }
        }
    };

    (
        $(
            #[doc = $expr: expr]
        )*
        $wrapped_type:ty
    ) => {
        paste! {
            decl_predicted!(
                $(
                    #[doc = $expr]
                )*
                $wrapped_type,
                [< $wrapped_type >]
            );
        }
    };
}

decl_predicted!(
    /// A PredictedDecimal is not a guaranteed amount, but a approximated based
    /// on the contents of the transaction manifest and the state of the ledger
    /// at the time of analysis (preview).
    Decimal
);

decl_predicted!(
    /// A prediction of a collection of NonFungibleLocalId
    Vec<NonFungibleLocalId>,
    NonFungibleLocalIds
);
