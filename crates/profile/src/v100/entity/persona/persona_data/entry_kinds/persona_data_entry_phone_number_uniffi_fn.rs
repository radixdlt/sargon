use crate::prelude::*;

json_string_convertible!(PersonaDataEntryPhoneNumber);

#[uniffi::export]
pub fn new_persona_data_entry_phone_number_sample(
) -> PersonaDataEntryPhoneNumber {
    PersonaDataEntryPhoneNumber::sample()
}

#[uniffi::export]
pub fn new_persona_data_entry_phone_number_sample_other(
) -> PersonaDataEntryPhoneNumber {
    PersonaDataEntryPhoneNumber::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataEntryPhoneNumber;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_data_entry_phone_number_sample(),
                new_persona_data_entry_phone_number_sample_other(),
                // duplicates should get removed
                new_persona_data_entry_phone_number_sample(),
                new_persona_data_entry_phone_number_sample_other(),
            ])
            .len(),
            2
        );
    }
}
