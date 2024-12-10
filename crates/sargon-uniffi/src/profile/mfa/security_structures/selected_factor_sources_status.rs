use crate::prelude::*;
use sargon::SelectedFactorSourcesStatus as InternalSelectedFactorSourcesStatus;

/// Represents the status of selected factor sources in the Security Shield building process.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum,
)]
pub enum SelectedFactorSourcesStatus {
    /// The selected factor sources are insufficient to build a Security Shield.
    Insufficient,

    /// The selected factor sources are suboptimal for building a Security Shield.
    Suboptimal,

    /// The selected factor sources are optimal for building a Security Shield.
    Optimal,
}
