use crate::prelude::*;

pub type SecurityStructureOfFactorSources =
    AbstractSecurityStructure<FactorSource>;

impl HasSampleValues for SecurityStructureOfFactorSources {
    fn sample() -> Self {
        let metadata = SecurityStructureMetadata::sample();
        Self::with_metadata(metadata, MatrixOfFactorSources::sample())
    }

    fn sample_other() -> Self {
        let metadata = SecurityStructureMetadata::sample_other();
        Self::with_metadata(metadata, MatrixOfFactorSources::sample_other())
    }
}

pub type MatrixOfFactorSourceIDs = MatrixOfFactorSourceIds;

impl TryFrom<(&MatrixOfFactorSourceIDs, &FactorSources)>
    for MatrixOfFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&MatrixOfFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        Self::new(value.0.clone(), value.1)
    }
}

impl TryFrom<(&SecurityStructureOfFactorSourceIDs, &FactorSources)>
    for SecurityStructureOfFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&SecurityStructureOfFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        let (id_level, factor_sources) = value;
        let matrix_of_factors = MatrixOfFactorSources::try_from((
            &id_level.matrix_of_factors,
            factor_sources,
        ))?;
        Ok(Self {
            metadata: id_level.metadata.clone(),
            matrix_of_factors,
        })
    }
}

impl From<SecurityStructureOfFactorSources>
    for SecurityStructureOfFactorSourceIDs
{
    fn from(value: SecurityStructureOfFactorSources) -> Self {
        Self {
            metadata: value.metadata,
            matrix_of_factors: value.matrix_of_factors.into(),
        }
    }
}

impl<const R: u8> From<AbstractRoleBuilderOrBuilt<R, FactorSource, ()>>
    for AbstractRoleBuilderOrBuilt<R, FactorSourceID, ()>
{
    fn from(value: AbstractRoleBuilderOrBuilt<R, FactorSource, ()>) -> Self {
        Self::with_factors(
            value.get_threshold(),
            value
                .get_threshold_factors()
                .iter()
                .map(|f| f.factor_source_id()),
            value
                .get_override_factors()
                .iter()
                .map(|f| f.factor_source_id()),
        )
    }
}

impl From<MatrixOfFactorSources> for MatrixOfFactorSourceIDs {
    fn from(value: MatrixOfFactorSources) -> Self {
        Self::_unvalidated_with_roles_and_days(
            PrimaryRoleWithFactorSourceIds::from(value.primary_role),
            RecoveryRoleWithFactorSourceIds::from(value.recovery_role),
            ConfirmationRoleWithFactorSourceIds::from(value.confirmation_role),
            value.number_of_days_until_auto_confirm,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorSources;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_into_id_level_and_back() {
        let factor_sources = FactorSources::sample_values_all();
        let sut = SUT::sample();
        let id_level = SecurityStructureOfFactorSourceIDs::from(sut.clone());
        let detailed = SUT::try_from((&id_level, &factor_sources)).unwrap();
        assert_eq!(detailed, sut);
    }
}
