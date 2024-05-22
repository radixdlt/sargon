use crate::prelude::*;

json_data_convertible!(PersonaDataEntryName);

#[uniffi::export]
pub fn new_persona_data_entry_name_sample() -> PersonaDataEntryName {
    PersonaDataEntryName::sample()
}

#[uniffi::export]
pub fn new_persona_data_entry_name_sample_other() -> PersonaDataEntryName {
    PersonaDataEntryName::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataEntryName;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_data_entry_name_sample(),
                new_persona_data_entry_name_sample_other(),
                // duplicates should get removed
                new_persona_data_entry_name_sample(),
                new_persona_data_entry_name_sample_other(),
            ])
            .len(),
            2
        );
    }
}
