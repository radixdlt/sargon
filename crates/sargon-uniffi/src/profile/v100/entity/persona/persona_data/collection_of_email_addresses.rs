use crate::prelude::*;

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataEntryEmailAddress`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataIdentifiedEmailAddress)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    EmailAddress,   // singular form
    EmailAddresses, // plural form
);
