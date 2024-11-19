use crate::prelude::*;



impl HasSampleValues for MatrixOfFactorSources {
    fn sample() -> Self {
        Self::new(
            PrimaryRoleWithFactorSources::sample(),
            RecoveryRoleWithFactorSources::sample(),
            ConfirmationRoleWithFactorSources::sample(),
        )
        .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(
            PrimaryRoleWithFactorSources::sample_other(),
            RecoveryRoleWithFactorSources::sample_other(),
            ConfirmationRoleWithFactorSources::sample_other(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorSources;

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
