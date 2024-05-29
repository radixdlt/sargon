use crate::prelude::*;

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample() -> ArculusCardFactorSource {
    ArculusCardFactorSource::sample()
}

#[uniffi::export]
pub fn new_arculus_card_factor_source_sample_other() -> ArculusCardFactorSource
{
    ArculusCardFactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ArculusCardFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_arculus_card_factor_source_sample(),
                new_arculus_card_factor_source_sample_other(),
                // duplicates should get removed
                new_arculus_card_factor_source_sample(),
                new_arculus_card_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }
}
