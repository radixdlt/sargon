use crate::prelude::*;

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
}
