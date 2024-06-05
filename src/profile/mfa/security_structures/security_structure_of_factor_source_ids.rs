use crate::prelude::*;

decl_security_structure_of!(
    /// A security structure at FactorSourceID level, this is
    /// what is serialized and store into Profile, we convert
    /// into this structure from `SecurityStructureOfFactorSources`.
    FactorSourceID,
);

impl Identifiable for SecurityStructureOfFactorSourceIDs {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

impl From<PrimaryRoleWithFactorSources> for PrimaryRoleWithFactorSourceIDs {
    fn from(value: PrimaryRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl From<RecoveryRoleWithFactorSources> for RecoveryRoleWithFactorSourceIDs {
    fn from(value: RecoveryRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl From<ConfirmationRoleWithFactorSources>
    for ConfirmationRoleWithFactorSourceIDs
{
    fn from(value: ConfirmationRoleWithFactorSources) -> Self {
        Self::new(
            value.threshold_factors.iter().map(|x| x.factor_source_id()),
            value.threshold,
            value.override_factors.iter().map(|x| x.factor_source_id()),
        )
    }
}

impl From<MatrixOfFactorSources> for MatrixOfFactorSourceIDs {
    fn from(value: MatrixOfFactorSources) -> Self {
        Self::new(
            value.primary_role.into(),
            value.recovery_role.into(),
            value.confirmation_role.into(),
        )
    }
}
impl From<SecurityStructureOfFactorSources>
    for SecurityStructureOfFactorSourceIDs
{
    fn from(value: SecurityStructureOfFactorSources) -> Self {
        Self::new(
            value.metadata,
            value.number_of_epochs_until_auto_confirmation,
            value.configuration.into(),
        )
    }
}

impl HasSampleValues for SecurityStructureOfFactorSourceIDs {
    fn sample() -> Self {
        SecurityStructureOfFactorSources::sample().into()
    }
    fn sample_other() -> Self {
        SecurityStructureOfFactorSources::sample_other().into()
    }
}

#[cfg(test)]
mod test_schematic_of_security_shield {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorSourceIDs;

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
