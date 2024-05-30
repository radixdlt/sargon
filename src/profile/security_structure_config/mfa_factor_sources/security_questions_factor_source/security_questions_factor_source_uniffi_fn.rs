use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_security_questions_factor_source_sample(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample()
}

#[uniffi::export]
pub fn new_security_questions_factor_source_sample_other(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestions_NOT_PRODUCTION_READY_FactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_security_questions_factor_source_sample(),
                new_security_questions_factor_source_sample_other(),
                // duplicates should get removed
                new_security_questions_factor_source_sample(),
                new_security_questions_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }
}
