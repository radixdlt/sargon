use crate::prelude::*;
use sargon::FeeSummary as InternalFeeSummary;

/// Detailed information on the amount of cost units consumed.
#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct FeeSummary {
    pub execution_cost: Decimal192,
    pub finalization_cost: Decimal192,
    pub royalty_cost: Decimal192,
    pub storage_expansion_cost: Decimal192,
}
