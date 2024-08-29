use crate::prelude::*;

#[uniffi::export]
pub fn new_shared_persona_data_sample() -> SharedPersonaData {
    SharedPersonaData::sample()
}

#[uniffi::export]
pub fn new_shared_persona_data_sample_other() -> SharedPersonaData {
    SharedPersonaData::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SharedPersonaData;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_shared_persona_data_sample(),
                new_shared_persona_data_sample_other(),
                // duplicates should get removed
                new_shared_persona_data_sample(),
                new_shared_persona_data_sample_other(),
            ])
            .len(),
            2
        );
    }
}
