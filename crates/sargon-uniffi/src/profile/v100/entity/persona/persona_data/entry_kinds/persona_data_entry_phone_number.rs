use crate::prelude::*;

/// A persons telephone number they have chosen to associated with a Persona, e.g.
/// `+46 987 654 321` (don't try calling this number, it does not exist).
///
/// Current implementation does not validate the phone number other than it
/// cannot be empty, since telephone number validation is tricky.
#[derive(
    Clone,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{number}")]
#[debug("{number}")]
pub struct PersonaDataEntryPhoneNumber {
    pub number: String,
}

impl Identifiable for PersonaDataEntryPhoneNumber {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.number.clone()
    }
}

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
