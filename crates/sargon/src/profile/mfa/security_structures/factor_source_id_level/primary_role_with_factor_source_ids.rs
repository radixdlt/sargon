use crate::prelude::*;

impl From<PrimaryRoleWithFactorSources> for PrimaryRoleWithFactorSourceIDs {
    fn from(value: PrimaryRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
        .expect("PrimaryRoleWithFactorSources has already been validated.")
    }
}

impl HasSampleValues for PrimaryRoleWithFactorSourceIDs {
    fn sample() -> Self {
        Self::threshold_factors_only(
            [FactorSourceID::sample(), FactorSourceID::sample_other()],
            2,
        )
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
    type SUT = PrimaryRoleWithFactorSourceIDs;

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
