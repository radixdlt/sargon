use crate::prelude::*;

pub(crate) type ConfirmationRoleWithFactorSources =
    RoleWithFactorSources<{ ROLE_CONFIRMATION }>;

impl HasSampleValues for ConfirmationRoleWithFactorSources {
    fn sample() -> Self {
        let ids = ConfirmationRoleWithFactorSourceIds::sample();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
    }

    fn sample_other() -> Self {
        let ids = ConfirmationRoleWithFactorSourceIds::sample_other();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ConfirmationRoleWithFactorSources;

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
