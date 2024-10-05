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

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample(
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::sample()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_sample_other(
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::sample_other()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_sources_auto_in_days(
    metadata: SecurityStructureMetadata,
    number_of_days_until_auto_confirmation: u16,
    matrix_of_factors: MatrixOfFactorSources,
) -> SecurityStructureOfFactorSources {
    SecurityStructureOfFactorSources::new_with_days(
        metadata,
        number_of_days_until_auto_confirmation,
        matrix_of_factors,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorSources;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_security_structure_of_factor_sources_sample(),
                new_security_structure_of_factor_sources_sample_other(),
                // duplicates should get removed
                new_security_structure_of_factor_sources_sample(),
                new_security_structure_of_factor_sources_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_security_structure_of_factor_sources_auto_in_days() {
        assert_eq!(
            new_security_structure_of_factor_sources_auto_in_days(
                SecurityStructureMetadata::sample(),
                1,
                MatrixOfFactorSources::sample()
            )
            .number_of_epochs_until_auto_confirmation,
            288
        );
    }
}
