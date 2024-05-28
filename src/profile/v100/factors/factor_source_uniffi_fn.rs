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
        assert_eq!(factor_source_to_string(&SUT::sample()), "iPhone iPhone device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a")
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
