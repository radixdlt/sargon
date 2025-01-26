use crate::prelude::*;

pub type MatrixOfFactorSources = AbstractMatrixBuilt<FactorSource>;

impl MatrixOfFactorSources {
    pub fn new(
        matrix: MatrixOfFactorSourceIds,
        factor_sources: &FactorSources,
    ) -> Result<Self, CommonError> {
        let primary_role =
            RoleWithFactorSources::new(matrix.primary_role, factor_sources)?;

        let recovery_role =
            RoleWithFactorSources::new(matrix.recovery_role, factor_sources)?;

        let confirmation_role = RoleWithFactorSources::new(
            matrix.confirmation_role,
            factor_sources,
        )?;

        if primary_role.role() != RoleKind::Primary
            || recovery_role.role() != RoleKind::Recovery
            || confirmation_role.role() != RoleKind::Confirmation
        {
            unreachable!("Programmer error!")
        }

        let built = unsafe {
            Self::unbuilt_with_roles_and_days(
                primary_role,
                recovery_role,
                confirmation_role,
                matrix.time_until_delayed_confirmation_is_callable,
            )
        };

        Ok(built)
    }
}

impl HasSampleValues for MatrixOfFactorSources {
    fn sample() -> Self {
        let ids = MatrixOfFactorSourceIds::sample();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
    }

    fn sample_other() -> Self {
        let ids = MatrixOfFactorSourceIds::sample_other();
        let factor_sources = FactorSources::sample_values_all();
        Self::new(ids, &factor_sources).unwrap()
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
