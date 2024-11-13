use crate::prelude::*;

decl_security_structure_of!(
    /// Security structure at `FactorSource` level.
    /// This is what user view, creates and manages.
    ///
    /// Before it gets saved into Profile gets converted into
    /// `SecurityStructureOfFactorSourceIDs`
    FactorSource,
);

pub const MINUTES_PER_DAY: u64 = 24 * 60;
pub const MINUTES_PER_EPOCH: u64 = 5;
pub const EPOCHS_PER_DAY: u64 = MINUTES_PER_DAY / MINUTES_PER_EPOCH;

pub fn days_to_epochs(days: u16) -> u64 {
    let days = days as u64;
    days * EPOCHS_PER_DAY
}

impl SecurityStructureOfFactorSources {
    pub fn new_with_days(
        metadata: SecurityStructureMetadata,
        number_of_days_until_auto_confirmation: u16,
        matrix_of_factors: MatrixOfFactorSources,
    ) -> Self {
        Self::new(
            metadata,
            days_to_epochs(number_of_days_until_auto_confirmation),
            matrix_of_factors,
        )
    }
}

impl Identifiable for SecurityStructureOfFactorSources {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

fn factors_from(
    ids: &[FactorSourceID],
    from: &FactorSources,
) -> Result<FactorSources> {
    ids.iter()
        .map(|id| {
            from.get_id(*id)
                .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID {
                    bad_value: *id,
                })
                .cloned()
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
        Self::new(threshold_factors, id_level.threshold, override_factors)
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
        Self::new(threshold_factors, id_level.threshold, override_factors)
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
        Self::new(threshold_factors, id_level.threshold, override_factors)
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

        Self::new(primary_role, recovery_role, confirmation_role)
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

impl HasSampleValues for SecurityStructureOfFactorSources {
    fn sample() -> Self {
        Self::new_with_days(
            SecurityStructureMetadata::sample(),
            14,
            MatrixOfFactorSources::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::new_with_days(
            SecurityStructureMetadata::sample_other(),
            28,
            MatrixOfFactorSources::sample_other(),
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
    fn test_epochs_per_day() {
        assert_eq!(EPOCHS_PER_DAY, 288);
    }

    #[test]
    fn test_days_to_epochs() {
        assert_eq!(days_to_epochs(0), 0);
        assert_eq!(days_to_epochs(10), 2880);
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
