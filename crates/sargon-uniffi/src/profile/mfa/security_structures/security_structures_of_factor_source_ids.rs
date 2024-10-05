use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityStructureOfFactorSourceIDs`]
    SecurityStructuresOfFactorSourceIDs,
    SecurityStructureOfFactorSourceIDs
);

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample(
) -> SecurityStructureOfFactorSourceIDs {
    SecurityStructureOfFactorSourceIDs::sample()
}

#[uniffi::export]
pub fn new_security_structure_of_factor_source_ids_sample_other(
) -> SecurityStructureOfFactorSourceIDs {
    SecurityStructureOfFactorSourceIDs::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorSourceIDs;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_security_structure_of_factor_source_ids_sample(),
                new_security_structure_of_factor_source_ids_sample_other(),
                // duplicates should get removed
                new_security_structure_of_factor_source_ids_sample(),
                new_security_structure_of_factor_source_ids_sample_other(),
            ])
            .len(),
            2
        );
    }
}
