use core_misc::decl_bool_type;
use std::ops::Not;

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
        /// The reason why the built shield would be weak.
        reason: SecurityShieldBuilderRuleViolation,
    },

    /// The selected factor sources form an invalid combination
    /// in the Security Shield building process.
    /// Example: Each role must have at least one factor.
    Invalid {
        reason: SecurityShieldBuilderStatusInvalidReason,
    },
}

impl HasSampleValues for SecurityShieldBuilderStatus {
    fn sample() -> Self {
        SecurityShieldBuilderStatus::Strong
    }

    fn sample_other() -> Self {
        SecurityShieldBuilderStatus::Weak {
            reason: SecurityShieldBuilderRuleViolation::RecoveryAndConfirmationFactorsOverlap
        }
    }
}

decl_bool_type!(IsPrimaryRoleFactorListEmpty, false);
decl_bool_type!(IsRecoveryRoleFactorListEmpty, false);
decl_bool_type!(IsConfirmationRoleFactorListEmpty, false);
decl_bool_type!(IsAuthSigningFactorMissing, false);

/// Represents the reason why the `SecurityShieldBuilder` has an invalid status.
/// This struct contains the specific reasons for each component of the security shield builder
/// being invalid. The components include:
/// - Primary role
/// - Recovery role
/// - Confirmation role
/// - Authentication signing
#[derive(Clone, Debug, PartialEq)]
pub struct SecurityShieldBuilderStatusInvalidReason {
    pub is_primary_role_factor_list_empty: IsPrimaryRoleFactorListEmpty,
    pub is_recovery_role_factor_list_empty: IsRecoveryRoleFactorListEmpty,
    pub is_confirmation_role_factor_list_empty:
        IsConfirmationRoleFactorListEmpty,
    pub is_auth_signing_factor_missing: IsAuthSigningFactorMissing,
}

impl SecurityShieldBuilderStatusInvalidReason {
    pub fn new(
        is_primary_role_factor_list_empty: IsPrimaryRoleFactorListEmpty,
        is_recovery_role_factor_list_empty: IsRecoveryRoleFactorListEmpty,
        is_confirmation_role_factor_list_empty: IsConfirmationRoleFactorListEmpty,
        is_auth_signing_factor_missing: IsAuthSigningFactorMissing,
    ) -> Option<Self> {
        if is_primary_role_factor_list_empty.not()
            && is_recovery_role_factor_list_empty.not()
            && is_confirmation_role_factor_list_empty.not()
            && is_auth_signing_factor_missing.not()
        {
            return None;
        }

        Some(Self {
            is_primary_role_factor_list_empty,
            is_recovery_role_factor_list_empty,
            is_confirmation_role_factor_list_empty,
            is_auth_signing_factor_missing,
        })
    }
}

impl HasSampleValues for SecurityShieldBuilderStatusInvalidReason {
    fn sample() -> Self {
        SecurityShieldBuilderStatusInvalidReason::new(
            IsPrimaryRoleFactorListEmpty(true),
            IsRecoveryRoleFactorListEmpty::default(),
            IsConfirmationRoleFactorListEmpty::default(),
            IsAuthSigningFactorMissing::default(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        SecurityShieldBuilderStatusInvalidReason::new(
            IsPrimaryRoleFactorListEmpty::default(),
            IsRecoveryRoleFactorListEmpty(true),
            IsConfirmationRoleFactorListEmpty(true),
            IsAuthSigningFactorMissing::default(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod status_tests {
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

#[cfg(test)]
mod reason_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilderStatusInvalidReason;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn new() {
        let invalid_reason = SecurityShieldBuilderStatusInvalidReason::new(
            IsPrimaryRoleFactorListEmpty::default(),
            IsRecoveryRoleFactorListEmpty::default(),
            IsConfirmationRoleFactorListEmpty::default(),
            IsAuthSigningFactorMissing::default(),
        );

        assert!(invalid_reason.is_none());

        let invalid_reason = SecurityShieldBuilderStatusInvalidReason::new(
            IsPrimaryRoleFactorListEmpty(true),
            IsRecoveryRoleFactorListEmpty::default(),
            IsConfirmationRoleFactorListEmpty::default(),
            IsAuthSigningFactorMissing::default(),
        );

        assert!(invalid_reason.is_some());
    }
}
