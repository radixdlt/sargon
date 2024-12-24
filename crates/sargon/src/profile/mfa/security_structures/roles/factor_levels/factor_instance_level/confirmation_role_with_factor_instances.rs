use crate::prelude::*;

pub type ConfirmationRoleWithFactorInstances =
    RoleWithFactorInstances<{ ROLE_CONFIRMATION }>;

impl HasSampleValues for ConfirmationRoleWithFactorInstances {
    fn sample() -> Self {
        MatrixOfFactorInstances::sample().confirmation_role
    }

    fn sample_other() -> Self {
        MatrixOfFactorInstances::sample_other().confirmation_role
    }
}

#[cfg(test)]
mod confirmation_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ConfirmationRoleWithFactorInstances;

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
