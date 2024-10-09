use crate::prelude::*;
use sargon::FeeSummary as InternalFeeSummary;

/// Detailed information on the amount of cost units consumed.
#[derive(Clone, Debug, PartialEq, Eq,  uniffi::Record)]
pub struct FeeSummary {
    pub execution_cost: Decimal192,
    pub finalization_cost: Decimal192,
    pub royalty_cost: Decimal192,
    pub storage_expansion_cost: Decimal192,
}

impl From<InternalFeeSummary> for FeeSummary {
    fn from(value: InternalFeeSummary) -> Self {
        Self {
            execution_cost: value.execution_cost.into(),
            finalization_cost: value.finalization_cost.into(),
            royalty_cost: value.royalty_cost.into(),
            storage_expansion_cost: value.storage_expansion_cost.into(),
        }
    }
}

impl Into<InternalFeeSummary> for FeeSummary {
    fn into(self) -> InternalFeeSummary {
        InternalFeeSummary {
            execution_cost: self.execution_cost.into(),
            finalization_cost: self.finalization_cost.into(),
            royalty_cost: self.royalty_cost.into(),
            storage_expansion_cost: self.storage_expansion_cost.into(),
        }
    }
}