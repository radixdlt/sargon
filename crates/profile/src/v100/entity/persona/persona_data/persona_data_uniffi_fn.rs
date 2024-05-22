use crate::prelude::*;

#[uniffi::export]
pub fn new_persona_data_sample() -> PersonaData {
    PersonaData::sample()
}

#[uniffi::export]
pub fn new_persona_data_sample_other() -> PersonaData {
    PersonaData::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaData;

    #[test]
    fn test_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_data_sample(),
                new_persona_data_sample_other(),
                // duplicates should get removed
                new_persona_data_sample(),
                new_persona_data_sample_other(),
            ])
            .len(),
            2
        );
    }
}
