use crate::prelude::*;

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataIdentifiedPhoneNumber`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataEntryPhoneNumber)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    PhoneNumber,  // singular form
    PhoneNumbers, // plural form
);
