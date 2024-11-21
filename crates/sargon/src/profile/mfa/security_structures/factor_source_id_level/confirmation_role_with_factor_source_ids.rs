use crate::prelude::*;

impl From<ConfirmationRoleWithFactorSources>
    for ConfirmationRoleWithFactorSourceIDs
{
    fn from(value: ConfirmationRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
        .expect("ConfirmationRoleWithFactorSources has already been validated.")
    }
}

impl HasSampleValues for ConfirmationRoleWithFactorSourceIDs {
    fn sample() -> Self {
        Self::new(
            [FactorSourceID::sample()],
            1,
            [FactorSourceID::sample_other()],
        )
        .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(
            [FactorSourceID::sample_other()],
            0,
            [FactorSourceID::sample()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ConfirmationRoleWithFactorSourceIDs;

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
