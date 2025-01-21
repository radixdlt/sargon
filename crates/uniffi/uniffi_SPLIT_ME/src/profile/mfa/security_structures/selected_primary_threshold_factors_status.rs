use crate::prelude::*;
use sargon::SelectedPrimaryThresholdFactorsStatus as InternalSelectedPrimaryThresholdFactorsStatus;
use sargon::SelectedPrimaryThresholdFactorsStatusInvalidReason as InternalSelectedPrimaryThresholdFactorsStatusInvalidReason;

/// Represents the status of selected Primary Threshold factor sources in the Security Shield building process.
/// Primarily used for UI logic representation in host applications.
#[derive(Clone, Debug, PartialEq, InternalConversion, uniffi::Enum)]
pub enum SelectedPrimaryThresholdFactorsStatus {
    /// The selected factor sources are optimal
    /// in the Security Shield building process.
    Optimal,

    /// The selected factor sources are suboptimal
    /// in the Security Shield building process.
    ///
    /// Note: Typically used in hosts as a warning message.
    Suboptimal,

    /// The selected factor sources are insufficient
    /// in the Security Shield building process.
    Insufficient,

    /// The selected factor sources form an invalid combination
    /// in the Security Shield building process.
    ///
    /// Example: A Password factor source cannot be used alone.
    Invalid {
        /// The reason why the selected factor sources are invalid.
        reason: SelectedPrimaryThresholdFactorsStatusInvalidReason,
    },
}

/// Represents the reason why the selected primary threshold factor sources are invalid.
/// Primarily used for UI logic representation in host applications.
#[derive(Clone, Debug, PartialEq, InternalConversion, uniffi::Enum)]
pub enum SelectedPrimaryThresholdFactorsStatusInvalidReason {
    CannotBeUsedAlone {
        factor_source_kind: FactorSourceKind,
    },
    Other {
        underlying: SecurityShieldBuilderRuleViolation,
    },
}
