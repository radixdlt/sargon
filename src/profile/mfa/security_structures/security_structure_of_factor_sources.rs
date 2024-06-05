use crate::prelude::*;

decl_security_structure_of!(
    /// Security structure at `FactorSource` level.
    /// This is what user view, creates and manages.
    ///
    /// Before it gets saved into Profile gets converted into
    /// `SecurityStructureOfFactorSourceIDs`
    FactorSource,
);

impl Identifiable for SecurityStructureOfFactorSources {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

fn factors_from(
    ids: impl IntoIterator<Item = FactorSourceID>,
    from: &FactorSources,
) -> Result<FactorSources> {
    ids.into_iter()
        .map(|id| {
            from.get_id(id.clone())
                .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID {
                    bad_value: id.clone(),
                })
                .map(|x| x.clone())
        })
        .collect::<Result<FactorSources>>()
}

impl TryFrom<(&PrimaryRoleWithFactorSourceIDs, &FactorSources)>
    for PrimaryRoleWithFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&PrimaryRoleWithFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        let (id_level, factor_sources) = value;

        let threshold_factors =
            factors_from(&id_level.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&id_level.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            id_level.threshold,
            override_factors,
        ))
    }
}
impl TryFrom<(&RecoveryRoleWithFactorSourceIDs, &FactorSources)>
    for RecoveryRoleWithFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&RecoveryRoleWithFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        let (id_level, factor_sources) = value;

        let threshold_factors =
            factors_from(&id_level.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&id_level.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            id_level.threshold,
            override_factors,
        ))
    }
}
impl TryFrom<(&ConfirmationRoleWithFactorSourceIDs, &FactorSources)>
    for ConfirmationRoleWithFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&ConfirmationRoleWithFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        let (id_level, factor_sources) = value;

        let threshold_factors =
            factors_from(&id_level.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&id_level.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            id_level.threshold,
            override_factors,
        ))
    }
}

impl TryFrom<(&MatrixOfFactorSourceIDs, &FactorSources)>
    for MatrixOfFactorSources
{
    type Error = CommonError;
    fn try_from(
        value: (&MatrixOfFactorSourceIDs, &FactorSources),
    ) -> Result<Self> {
        let (id_level, factor_sources) = value;
        let primary_role = PrimaryRoleWithFactorSources::try_from((
            &id_level.primary_role,
            factor_sources,
        ))?;

        let recovery_role = RecoveryRoleWithFactorSources::try_from((
            &id_level.recovery_role,
            factor_sources,
        ))?;

        let confirmation_role = ConfirmationRoleWithFactorSources::try_from((
            &id_level.confirmation_role,
            factor_sources,
        ))?;

        Ok(Self::new(primary_role, recovery_role, confirmation_role))
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
        let matrix = MatrixOfFactorSources::try_from((
            &id_level.matrix_of_factors,
            factor_sources,
        ))?;
        Ok(Self::new(
            id_level.metadata.clone(),
            id_level.number_of_epochs_until_auto_confirmation,
            matrix,
        ))
    }
}

impl HasSampleValues for PrimaryRoleWithFactorSources {
    fn sample() -> Self {
        Self::new(
            [
                FactorSource::sample_device_babylon(),
                FactorSource::sample_arculus(),
                FactorSource::sample_off_device(),
            ],
            2,
            [FactorSource::sample_ledger()],
        )
    }
    fn sample_other() -> Self {
        Self::new(
            [
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_arculus_other(),
                FactorSource::sample_off_device_other(),
            ],
            2,
            [FactorSource::sample_ledger_other()],
        )
    }
}

impl HasSampleValues for RecoveryRoleWithFactorSources {
    fn sample() -> Self {
        Self::new(
            [
                FactorSource::sample_trusted_contact_frank(),
                FactorSource::sample_trusted_contact_grace(),
                FactorSource::sample_trusted_contact_judy(),
            ],
            2,
            [FactorSource::sample_ledger()],
        )
    }
    fn sample_other() -> Self {
        Self::new(
            [
                FactorSource::sample_trusted_contact_trudy(),
                FactorSource::sample_trusted_contact_oscar(),
                FactorSource::sample_trusted_contact_radix(),
            ],
            2,
            [FactorSource::sample_ledger_other()],
        )
    }
}

impl HasSampleValues for ConfirmationRoleWithFactorSources {
    fn sample() -> Self {
        Self::new(
            [],
            0,
            [
                FactorSource::sample_security_questions(),
                FactorSource::sample_ledger(),
            ],
        )
    }
    fn sample_other() -> Self {
        Self::new(
            [],
            0,
            [
                FactorSource::sample_security_questions_other(),
                FactorSource::sample_ledger_other(),
            ],
        )
    }
}

impl HasSampleValues for SecurityStructureOfFactorSources {
    fn sample() -> Self {
        Self::new(
            SecurityStructureMetadata::sample(),
            4096, // 14.2 days
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::sample(),
                RecoveryRoleWithFactorSources::sample(),
                ConfirmationRoleWithFactorSources::sample(),
            ),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            SecurityStructureMetadata::sample_other(),
            8192, // 28.4 days
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::sample_other(),
                RecoveryRoleWithFactorSources::sample_other(),
                ConfirmationRoleWithFactorSources::sample_other(),
            ),
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
