use crate::prelude::*;

pub(crate) type RecoveryRoleWithFactorSources =
    RoleWithFactorSources<{ ROLE_RECOVERY }>;

impl HasSampleValues for RecoveryRoleWithFactorSources {
    fn sample() -> Self {
        let ids = RecoveryRoleWithFactorSourceIds::sample();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
    }

    fn sample_other() -> Self {
        let ids = RecoveryRoleWithFactorSourceIds::sample_other();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RecoveryRoleWithFactorSources;

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
