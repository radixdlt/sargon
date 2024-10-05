use crate::prelude::*;

/// Personal information a user has associated with a certain Persona, of different kinds, such as name,
/// email address(es) or phone number(s). This information is only ever stored in Profile and is never
/// uploaded to the Radix Network.
///
/// These entries of different kinds can be queried for in a request sent by a dApp, e.g.
/// Radix Dashboard might ask "Give me ongoing access to Name and 2 Email addresses for
/// a Persona" (just a silly example, Radix Dashboard would never ask for that and why 2 email addresses?).
///
/// The Profile will then use the fact that each Persona Data Entry has a stable ID so that Profile can
/// refer the entry just by the ID, and Profile can thus record which Persona Data Entry a user has selected
/// to share with the dApp, without duplicating the value of that entry (just like how we use FactorSourceIDs).
/// Since a dApp can ask for *ongoing* access next time the user interacts with the same dApp, if user has
/// not revoked the dApps access, the wallet clients will automatically send back the Persona Data Entry values
/// even if they have been updated - the value might have changed but their IDs have not. Thus if a user
/// deletes a Persona Data Entry (e.g. a phone number), and later re-inputs the same phone number, even
/// it the exact same value is used, it will still be treated as a new entry since its ID is new, meaning
/// that the next time the user interacts with a previously authorized dApp the wallet cannot automatically
/// respond back to dApp with the PersonaData, but user will have to re-authorize the request for ongoing
/// access for the requested PersonaData entries.
#[derive(
    Clone,
    Default,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.string_representation(false))]
#[debug("{}", self.string_representation(true))]
pub struct PersonaData {
    /// A persons name they have chosen to associated with a Persona, e.g. "Bruce 'Batman' Wayne" using Western name variant,
    /// or `"Jun-fan 'Bruce' Lee"` using Eastern name variant (family name comes before given name(s)).
    ///
    /// Note that the type is Option of `PersonaDataIdentifiedName` and not of type [`PersonaDataEntryName`][name],
    /// `PersonaDataIdentifiedName` is essentially a tuple of `(Uuid, PersonaDataEntryName)`.
    ///
    /// [name]: PersonaDataEntryName
    pub name: Option<PersonaDataIdentifiedName>,

    /// A collection of [`PersonaDataIdentifiedPhoneNumber`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataEntryPhoneNumber)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    pub phone_numbers: CollectionOfPhoneNumbers,

    /// A collection of [`PersonaDataEntryEmailAddress`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataIdentifiedEmailAddress)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    pub email_addresses: CollectionOfEmailAddresses,
}

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
