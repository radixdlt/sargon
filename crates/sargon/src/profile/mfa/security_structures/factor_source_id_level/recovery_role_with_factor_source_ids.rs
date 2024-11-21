use crate::prelude::*;

impl From<RecoveryRoleWithFactorSources> for RecoveryRoleWithFactorSourceIDs {
    fn from(value: RecoveryRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
        .expect("RecoveryRoleWithFactorSources has already been validated.")
    }
}

impl HasSampleValues for RecoveryRoleWithFactorSourceIDs {
    fn sample() -> Self {
        Self::threshold_factors_only([FactorSourceID::sample_other()], 1)
            .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(
            [FactorSourceID::sample()],
            1,
            [FactorSourceID::sample_other()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RecoveryRoleWithFactorSourceIDs;

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
