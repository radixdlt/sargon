use crate::prelude::*;

/// Represents the status of `SecurityShieldBuilder`.
/// Used for UI representation in host applications.
#[derive(Clone, Debug, PartialEq)]
pub enum SecurityShieldBuilderStatus {
    /// The selected factor sources form a strong combination
    /// in the Security Shield building process.
    Strong,

    /// The selected factor sources form a weak combination
    /// in the Security Shield building process.
    Weak {
        /// The reason why the resulting shield would be unsafe.
        reason: SecurityShieldBuilderRuleViolationReason,
    },

    /// The selected factor sources form an invalid combination
    /// in the Security Shield building process.
    /// Example: Each role must have at least one factor.
    Invalid {
        reason: SecurityShieldBuilderStatusInvalidReason,
    },
}

/// Represents the reason why the shield builder has invalid status.
#[derive(Clone, Debug, PartialEq)]
pub struct SecurityShieldBuilderStatusInvalidReason {
    primary: Option<SecurityShieldBuilderStatusInvalidReasonError>,
    auth_signing: Option<SecurityShieldBuilderStatusInvalidReasonError>,
    recovery: Option<SecurityShieldBuilderStatusInvalidReasonError>,
    confirmation: Option<SecurityShieldBuilderStatusInvalidReasonError>,
}

impl SecurityShieldBuilderStatusInvalidReason {
    pub fn new(
        auth_signing: Option<SecurityShieldBuilderStatusInvalidReasonError>,
        primary: Option<SecurityShieldBuilderStatusInvalidReasonError>,
        recovery: Option<SecurityShieldBuilderStatusInvalidReasonError>,
        confirmation: Option<SecurityShieldBuilderStatusInvalidReasonError>,
    ) -> Result<Self> {
        if auth_signing.is_none()
            && primary.is_none()
            && recovery.is_none()
            && confirmation.is_none()
        {
            return Err(CommonError::FailedToCreateSecurityShieldBuilderStatusInvalidReason);
        }

        Ok(Self {
            primary,
            auth_signing,
            recovery,
            confirmation,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SecurityShieldBuilderStatusInvalidReasonError {
    MissingFactor,
}

impl TryFrom<SecurityShieldBuilderRuleViolationReason>
    for SecurityShieldBuilderStatusInvalidReasonError
{
    type Error = CommonError;

    fn try_from(
        reason: SecurityShieldBuilderRuleViolationReason,
    ) -> Result<Self, Self::Error> {
        match reason {
            SecurityShieldBuilderRuleViolationReason::MissingAuthSigningFactor |
            SecurityShieldBuilderRuleViolationReason::PrimaryRoleMustHaveAtLeastOneFactor |
            SecurityShieldBuilderRuleViolationReason::RecoveryRoleMustHaveAtLeastOneFactor |
            SecurityShieldBuilderRuleViolationReason::ConfirmationRoleMustHaveAtLeastOneFactor=> Ok(SecurityShieldBuilderStatusInvalidReasonError::MissingFactor),
            _ => Err(CommonError::FailedToMapSecurityShieldBuilderViolationReasonToInvalidReasonError),
        }
    }
}

impl HasSampleValues for SecurityShieldBuilderStatus {
    fn sample() -> Self {
        SecurityShieldBuilderStatus::Strong
    }

    fn sample_other() -> Self {
        SecurityShieldBuilderStatus::Weak {
            reason: SecurityShieldBuilderRuleViolationReason::RecoveryAndConfirmationFactorsOverlap
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilderStatus;

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
