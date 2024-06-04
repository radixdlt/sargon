use crate::prelude::*;

#[uniffi::export]
pub fn new_factor_source_common_sample() -> FactorSourceCommon {
    FactorSourceCommon::sample()
}

#[uniffi::export]
pub fn new_factor_source_common_sample_other() -> FactorSourceCommon {
    FactorSourceCommon::sample_other()
}

#[uniffi::export]
pub fn new_factor_source_common_olympia() -> FactorSourceCommon {
    FactorSourceCommon::new_olympia()
}

#[uniffi::export]
pub fn new_factor_source_common_babylon() -> FactorSourceCommon {
    FactorSourceCommon::new_babylon()
}

#[uniffi::export]
pub fn new_factor_source_common_bdfs(is_main: bool) -> FactorSourceCommon {
    FactorSourceCommon::new_bdfs(is_main)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceCommon;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_common_sample(),
                new_factor_source_common_sample_other(),
                // duplicates should get removed
                new_factor_source_common_sample(),
                new_factor_source_common_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn hash_of_new() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_common_babylon(),
                new_factor_source_common_olympia(),
                new_factor_source_common_bdfs(false),
                // duplicates should get removed
                new_factor_source_common_babylon(),
                new_factor_source_common_olympia(),
                new_factor_source_common_bdfs(false),
            ])
            .len(),
            3
        );
    }
}
