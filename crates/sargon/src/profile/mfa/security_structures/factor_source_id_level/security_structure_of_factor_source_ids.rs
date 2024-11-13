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

impl From<SecurityStructureOfFactorSources>
    for SecurityStructureOfFactorSourceIDs
{
    fn from(value: SecurityStructureOfFactorSources) -> Self {
        Self::new(
            value.metadata,
            value.number_of_epochs_until_auto_confirmation,
            value.matrix_of_factors.into(),
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
