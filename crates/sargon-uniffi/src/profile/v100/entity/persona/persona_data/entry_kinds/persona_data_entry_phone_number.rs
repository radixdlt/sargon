use crate::prelude::*;
use sargon::PersonaDataEntryPhoneNumber as InternalPersonaDataEntryPhoneNumber;

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
    uniffi::Record,
)]
pub struct PersonaDataEntryPhoneNumber {
    pub number: String,
}

impl From<InternalPersonaDataEntryPhoneNumber> for PersonaDataEntryPhoneNumber {
    fn from(value: InternalPersonaDataEntryPhoneNumber) -> Self {
        Self {
            number: value.number,
        }
    }
}

impl Into<InternalPersonaDataEntryPhoneNumber> for PersonaDataEntryPhoneNumber {
    fn into(self) -> InternalPersonaDataEntryPhoneNumber {
        InternalPersonaDataEntryPhoneNumber {
            number: self.number,
        }
    }
}

json_string_convertible!(PersonaDataEntryPhoneNumber);

#[uniffi::export]
pub fn new_persona_data_entry_phone_number_sample(
) -> PersonaDataEntryPhoneNumber {
    InternalPersonaDataEntryPhoneNumber::sample().into()
}

#[uniffi::export]
pub fn new_persona_data_entry_phone_number_sample_other(
) -> PersonaDataEntryPhoneNumber {
    InternalPersonaDataEntryPhoneNumber::sample_other().into()
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
