use crate::prelude::*;
use sargon::PersonaData as InternalPersonaData;

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
    PartialEq,
    Hash,
    Eq,
     uniffi::Record,
)]
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

impl From<InternalPersonaData> for PersonaData {
    fn from(value: InternalPersonaData) -> Self {
        Self {
            name: value.name.map(Into::into),
            phone_numbers: value.phone_numbers.into(),
            email_addresses: value.email_addresses.into(),
        }
    }
}

impl Into<InternalPersonaData> for PersonaData {
    fn into(self) -> InternalPersonaData {
        InternalPersonaData {
            name: self.name.map(Into::into),
            phone_numbers: self.phone_numbers.into(),
            email_addresses: self.email_addresses.into(),
        }
    }
}

#[uniffi::export]
pub fn new_persona_data_sample() -> PersonaData {
    InternalPersonaData::sample().into()
}

#[uniffi::export]
pub fn new_persona_data_sample_other() -> PersonaData {
    InternalPersonaData::sample_other().into()
}

