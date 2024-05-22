use crate::prelude::*;

declare_collection_of_identified_entry!(
    /// A collection of [`PersonaDataEntryEmailAddress`]s, which is essentially a tuple of
    /// `(Uuid, PersonaDataIdentifiedEmailAddress)`, each element is identifiable by its ID. Can be empty, can
    /// contain elements with the same value, but under different IDs.
    email_address,   // singular form
    email_addresses, // plural form
    "[alan@turing.hero, satoshi@nakamoto.btc]",
    "[alan@turing.hero - 00000000-0000-0000-0000-000000000001, satoshi@nakamoto.btc - 00000000-0000-0000-0000-000000000002]",
    r#"
    [
        {
            "id": "00000000-0000-0000-0000-000000000001",
            "value": "alan@turing.hero"
        },
        {
            "id": "00000000-0000-0000-0000-000000000002",
            "value": "satoshi@nakamoto.btc"
        }
    ]
    "#
);
