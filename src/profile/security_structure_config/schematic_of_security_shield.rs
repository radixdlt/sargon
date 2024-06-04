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

impl TryFrom<&(SchematicOfSecurityShieldPrimaryRole, FactorSources)>
    for SecurityShieldPrimaryRole
{
    type Error = CommonError;
    fn try_from(
        value: &(SchematicOfSecurityShieldPrimaryRole, FactorSources),
    ) -> Result<Self> {
        let (schematics, factor_sources) = value;
        // Self::new(
        //     value.threshold_factors.iter().map(|x| x.factor_source_id()),
        //     value.threshold,
        //     value.override_factors.iter().map(|x| x.factor_source_id()),
        // )
        let threshold_factors = schematics
            .threshold_factors
            .iter()
            .map(|id| {
                factor_sources
                    .get_id(id)
                    .ok_or(CommonError::Unknown)
                    .map(|x| x.clone())
            })
            .collect::<Result<Vec<FactorSource>>>()?;
        todo!()
        // Self::new(threshold_factors, threshold, override_factors)
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
