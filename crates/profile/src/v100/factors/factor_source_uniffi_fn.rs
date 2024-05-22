use crate::prelude::*;

#[uniffi::export]
pub fn factor_source_to_string(factor_source: &FactorSource) -> String {
    factor_source.to_string()
}

#[uniffi::export]
pub fn factor_source_supports_olympia(factor_source: &FactorSource) -> bool {
    factor_source.supports_olympia()
}

#[uniffi::export]
pub fn factor_source_supports_babylon(factor_source: &FactorSource) -> bool {
    factor_source.supports_babylon()
}

#[uniffi::export]
pub fn new_factor_source_sample() -> FactorSource {
    FactorSource::sample()
}

#[uniffi::export]
pub fn new_factor_source_sample_other() -> FactorSource {
    FactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSource;

    #[test]
    fn to_string() {
        assert_eq!(factor_source_to_string(&SUT::sample()), "Unknown Name iPhone device:3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240")
    }

    #[test]
    fn hash_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_sample(),
                new_factor_source_sample_other(),
                // duplicates should be removed
                new_factor_source_sample(),
                new_factor_source_sample_other(),
            ])
            .len(),
            2
        )
    }
}
