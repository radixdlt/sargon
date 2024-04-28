use crate::prelude::*;

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaIDsOfPersonaDataEntries,
    PersonaDataEntryID,
    "AtLeast: 2 - #3 ids shared",
    "AtLeast: 2 - shared ids: [00000000-0000-0000-0000-000000000001, 00000000-0000-0000-0000-000000000002, 00000000-0000-0000-0000-000000000004]",
    r#"
    {
        "request": {
            "quantifier": "atLeast",
            "quantity": 2
        },
        "ids": ["00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002", "00000000-0000-0000-0000-000000000004"]
    }
    "#
);

impl HasSampleValues for SharedToDappWithPersonaIDsOfPersonaDataEntries {
    fn sample() -> Self {
        Self::new(
            RequestedQuantity::at_least(2),
            IdentifiedVecVia::from_iter([
                PersonaDataEntryID::sample_one(),
                PersonaDataEntryID::sample_two(),
                PersonaDataEntryID::sample_four(),
            ]),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            RequestedQuantity::exactly(1),
            IdentifiedVecVia::from_iter([PersonaDataEntryID::sample_one()]),
        )
    }
}
