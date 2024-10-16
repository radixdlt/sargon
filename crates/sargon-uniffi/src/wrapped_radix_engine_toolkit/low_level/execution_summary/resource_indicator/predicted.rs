use crate::prelude::*;
use sargon::PredictedDecimal as InternalPredictedDecimal;
use sargon::PredictedNonFungibleLocalIds as InternalPredictedNonFungibleLocalIds;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PredictedDecimal {
    pub value: Decimal,
    pub instruction_index: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PredictedNonFungibleLocalIds {
    pub value: Vec<NonFungibleLocalId>,
    pub instruction_index: u64,
}
