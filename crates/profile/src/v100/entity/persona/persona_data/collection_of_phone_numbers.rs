use crate::prelude::*;

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataIdentifiedPhoneNumber`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataEntryPhoneNumber)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    phone_number,  // singular form
    phone_numbers, // plural form
    "[+46123456789, +44987654321]",
    "[+46123456789 - 00000000-0000-0000-0000-000000000001, +44987654321 - 00000000-0000-0000-0000-000000000002]",
    r#"
    [
        {
            "id": "00000000-0000-0000-0000-000000000001",
            "value": "+46123456789"
        },
        {
            "id": "00000000-0000-0000-0000-000000000002",
            "value": "+44987654321"
        }
    ]
    "#
);
