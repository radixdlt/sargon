use crate::prelude::*;

impl From<MatrixOfFactorSources> for MatrixOfFactorSourceIDs {
    fn from(value: MatrixOfFactorSources) -> Self {
        Self::new(
            value.primary_role.into(),
            value.recovery_role.into(),
            value.confirmation_role.into(),
        )
        .unwrap()
    }
}

impl HasSampleValues for MatrixOfFactorSourceIDs {
    fn sample() -> Self {
        Self::new(
            PrimaryRoleWithFactorSourceIDs::sample(),
            RecoveryRoleWithFactorSourceIDs::sample(),
            ConfirmationRoleWithFactorSourceIDs::sample(),
        )
        .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(
            PrimaryRoleWithFactorSourceIDs::sample_other(),
            RecoveryRoleWithFactorSourceIDs::sample_other(),
            ConfirmationRoleWithFactorSourceIDs::sample_other(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorSourceIDs;

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
