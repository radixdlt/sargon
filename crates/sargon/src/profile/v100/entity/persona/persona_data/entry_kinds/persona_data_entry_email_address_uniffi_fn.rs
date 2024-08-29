use crate::prelude::*;

json_string_convertible!(PersonaDataEntryEmailAddress);

#[uniffi::export]
pub fn new_persona_data_entry_email_address_sample(
) -> PersonaDataEntryEmailAddress {
    PersonaDataEntryEmailAddress::sample()
}

#[uniffi::export]
pub fn new_persona_data_entry_email_address_sample_other(
) -> PersonaDataEntryEmailAddress {
    PersonaDataEntryEmailAddress::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataEntryEmailAddress;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_data_entry_email_address_sample(),
                new_persona_data_entry_email_address_sample_other(),
                // duplicates should get removed
                new_persona_data_entry_email_address_sample(),
                new_persona_data_entry_email_address_sample_other(),
            ])
            .len(),
            2
        );
    }
}
