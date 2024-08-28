use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_matrix_of_factor_sources_sample() -> MatrixOfFactorSources {
    MatrixOfFactorSources::sample()
}

#[uniffi::export]
pub fn new_matrix_of_factor_sources_sample_other() -> MatrixOfFactorSources {
    MatrixOfFactorSources::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorSources;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_matrix_of_factor_sources_sample(),
                new_matrix_of_factor_sources_sample_other(),
                // duplicates should get removed
                new_matrix_of_factor_sources_sample(),
                new_matrix_of_factor_sources_sample_other(),
            ])
            .len(),
            2
        );
    }
}
