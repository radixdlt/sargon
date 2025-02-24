use crate::prelude::*;

/// Represents the status of selected Primary Threshold factor sources in the Security Shield building process.
/// Primarily used for UI logic representation in host applications.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum SelectedPrimaryThresholdFactorsStatusInvalidReason {
    CannotBeUsedAlone {
        factor_source_kind: FactorSourceKind,
    },
    Other {
        underlying: SecurityShieldBuilderRuleViolation,
    },
}

impl HasSampleValues for SelectedPrimaryThresholdFactorsStatusInvalidReason {
    fn sample() -> Self {
        SelectedPrimaryThresholdFactorsStatusInvalidReason::CannotBeUsedAlone {
            factor_source_kind: FactorSourceKind::Password,
        }
    }

    fn sample_other() -> Self {
        SelectedPrimaryThresholdFactorsStatusInvalidReason::Other {
            underlying: SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SelectedPrimaryThresholdFactorsStatusInvalidReason;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
