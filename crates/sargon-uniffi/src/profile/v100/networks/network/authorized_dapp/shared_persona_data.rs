use crate::prelude::*;

/// Identities for PersonaData entry values a user have shared with a dApp.
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.description())]
pub struct SharedPersonaData {
    /// ID of a `PersonaDataEntryName` the user has shared with some dApp on some network,
    /// can be `None`.
    pub name: Option<PersonaDataEntryID>,

    /// IDs of a `PersonaDataEntryEmailAddress`es the user has shared with some dApp on some network
    /// can be `None`, or can be `Some(<EMPTY>)`.
    pub email_addresses: Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,

    /// IDs of a `PersonaDataEntryPhoneNumber`s the user has shared with some dApp on some network
    /// can be `None`, or can be `Some(<EMPTY>)`.
    pub phone_numbers: Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,
}

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
