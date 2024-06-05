use crate::prelude::*;

decl_security_shield_at_level!(
    /// A "Schematic Of SecurityShield", is at `FactorSourceID` level.
    /// This is what is saved into Profile, being just references to FactorSources.
    /// The user views, creates and manages SecurityShields, which is at `FactorSource`
    /// level.
    SchematicOfSecurityShield,
    FactorSourceID,
);

impl Identifiable for SchematicOfSecurityShield {
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
            from.get_id(id.clone())
                .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID {
                    bad_value: id.clone(),
                })
                .map(|x| x.clone())
        })
        .collect::<Result<FactorSources>>()
}

impl TryFrom<(&SchematicOfSecurityShieldPrimaryRole, &FactorSources)>
    for SecurityShieldPrimaryRole
{
    type Error = CommonError;
    fn try_from(
        value: (&SchematicOfSecurityShieldPrimaryRole, &FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;

        let threshold_factors =
            factors_from(&schematics.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&schematics.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            schematics.threshold,
            override_factors,
        ))
    }
}
impl From<SecurityShieldPrimaryRole> for SchematicOfSecurityShieldPrimaryRole {
    fn from(value: SecurityShieldPrimaryRole) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl TryFrom<(&SchematicOfSecurityShieldRecoveryRole, &FactorSources)>
    for SecurityShieldRecoveryRole
{
    type Error = CommonError;
    fn try_from(
        value: (&SchematicOfSecurityShieldRecoveryRole, &FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;

        let threshold_factors =
            factors_from(&schematics.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&schematics.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            schematics.threshold,
            override_factors,
        ))
    }
}
impl From<SecurityShieldRecoveryRole>
    for SchematicOfSecurityShieldRecoveryRole
{
    fn from(value: SecurityShieldRecoveryRole) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl TryFrom<(&SchematicOfSecurityShieldConfirmationRole, &FactorSources)>
    for SecurityShieldConfirmationRole
{
    type Error = CommonError;
    fn try_from(
        value: (&SchematicOfSecurityShieldConfirmationRole, &FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;

        let threshold_factors =
            factors_from(&schematics.threshold_factors, factor_sources)?;

        let override_factors =
            factors_from(&schematics.override_factors, factor_sources)?;
        Ok(Self::new(
            threshold_factors,
            schematics.threshold,
            override_factors,
        ))
    }
}
impl From<SecurityShieldConfirmationRole>
    for SchematicOfSecurityShieldConfirmationRole
{
    fn from(value: SecurityShieldConfirmationRole) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl TryFrom<(&SchematicOfSecurityShieldConfiguration, &FactorSources)>
    for SecurityShieldConfiguration
{
    type Error = CommonError;
    fn try_from(
        value: (&SchematicOfSecurityShieldConfiguration, &FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;
        let primary_role = SecurityShieldPrimaryRole::try_from((
            &schematics.primary_role,
            factor_sources,
        ))?;

        let recovery_role = SecurityShieldRecoveryRole::try_from((
            &schematics.recovery_role,
            factor_sources,
        ))?;

        let confirmation_role = SecurityShieldConfirmationRole::try_from((
            &schematics.confirmation_role,
            factor_sources,
        ))?;

        Ok(Self::new(
            primary_role,
            recovery_role,
            confirmation_role,
            schematics.number_of_epochs_until_auto_confirmation,
        ))
    }
}
impl From<SecurityShieldConfiguration>
    for SchematicOfSecurityShieldConfiguration
{
    fn from(value: SecurityShieldConfiguration) -> Self {
        Self::new(
            value.primary_role.into(),
            value.recovery_role.into(),
            value.confirmation_role.into(),
            value.number_of_epochs_until_auto_confirmation,
        )
    }
}
impl From<SecurityShield> for SchematicOfSecurityShield {
    fn from(value: SecurityShield) -> Self {
        Self::new(value.metadata, value.configuration.into())
    }
}

impl TryFrom<(&SchematicOfSecurityShield, &FactorSources)> for SecurityShield {
    type Error = CommonError;
    fn try_from(
        value: (&SchematicOfSecurityShield, &FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;
        let config = SecurityShieldConfiguration::try_from((
            &schematics.configuration,
            factor_sources,
        ))?;
        Ok(Self::new(schematics.metadata.clone(), config))
    }
}

impl HasSampleValues for SchematicOfSecurityShield {
    fn sample() -> Self {
        SecurityShield::sample().into()
    }
    fn sample_other() -> Self {
        SecurityShield::sample_other().into()
    }
}

#[cfg(test)]
mod test_schematic_of_security_shield {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SchematicOfSecurityShield;

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

// #[cfg(test)]
// mod test_schematics_primary_role {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = RoleOfTierReference;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }
// }

// #[cfg(test)]
// mod test_security_structure_reference {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = SecurityStructureReference;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }
// }
