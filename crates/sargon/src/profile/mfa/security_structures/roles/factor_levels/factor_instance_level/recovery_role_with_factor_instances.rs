use crate::prelude::*;

pub type RecoveryRoleWithFactorInstances =
    RoleWithFactorInstances<{ ROLE_RECOVERY }>;

impl HasSampleValues for RecoveryRoleWithFactorInstances {
    fn sample() -> Self {
        MatrixOfFactorInstances::sample().recovery_role
    }

    fn sample_other() -> Self {
        MatrixOfFactorInstances::sample_other().recovery_role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RecoveryRoleWithFactorInstances;

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
