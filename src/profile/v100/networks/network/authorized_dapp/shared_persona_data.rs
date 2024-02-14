use crate::prelude::*;

/// Identities for PersonaData entry values a user have shared with a dApp.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
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

impl SharedPersonaData {
    pub fn new(
        name: impl Into<Option<PersonaDataEntryID>>,
        email_addresses: impl Into<
            Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,
        >,
        phone_numbers: impl Into<
            Option<SharedToDappWithPersonaIDsOfPersonaDataEntries>,
        >,
    ) -> Self {
        Self {
            name: name.into(),
            email_addresses: email_addresses.into(),
            phone_numbers: phone_numbers.into(),
        }
    }
}

impl HasPlaceholder for SharedPersonaData {
    fn placeholder() -> Self {
        let id = IDStepper::<PersonaDataEntryID>::starting_at(0);
        unsafe {
            Self::new(
                id.next(),
                SharedToDappWithPersonaIDsOfPersonaDataEntries::new(
                    RequestedQuantity::exactly(2),
                    IdentifiedVecVia::from_iter([id.next(), id.next()]),
                ),
                SharedToDappWithPersonaIDsOfPersonaDataEntries::new(
                    RequestedQuantity::at_least(1),
                    IdentifiedVecVia::from_iter([id.next(), id.next()]),
                ),
            )
        }
    }

    fn placeholder_other() -> Self {
        let id = IDStepper::<PersonaDataEntryID>::starting_at(0xf0);
        unsafe {
            Self::new(
                id.next(),
                SharedToDappWithPersonaIDsOfPersonaDataEntries::new(
                    RequestedQuantity::exactly(2),
                    IdentifiedVecVia::from_iter([id.next(), id.next()]),
                ),
                SharedToDappWithPersonaIDsOfPersonaDataEntries::new(
                    RequestedQuantity::at_least(1),
                    IdentifiedVecVia::from_iter([id.next(), id.next()]),
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SharedPersonaData;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let model = SharedPersonaData::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"name": "00000000-0000-0000-0000-000000000000",
				"emailAddresses": {
					"request": {
						"quantifier": "exactly",
						"quantity": 2
					},
					"ids": [
						"00000000-0000-0000-0000-000000000001",
						"00000000-0000-0000-0000-000000000002"
					]
				},
				"phoneNumbers": {
					"request": {
						"quantifier": "atLeast",
						"quantity": 1
					},
					"ids": [
						"00000000-0000-0000-0000-000000000003",
						"00000000-0000-0000-0000-000000000004"
					]
				}
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = SharedPersonaData::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
				"name": "00000000-0000-0000-0000-0000000000f0",
				"emailAddresses": {
					"request": {
						"quantifier": "exactly",
						"quantity": 2
					},
					"ids": [
						"00000000-0000-0000-0000-0000000000f1",
						"00000000-0000-0000-0000-0000000000f2"
					]
				},
				"phoneNumbers": {
					"request": {
						"quantifier": "atLeast",
						"quantity": 1
					},
					"ids": [
						"00000000-0000-0000-0000-0000000000f3",
						"00000000-0000-0000-0000-0000000000f4"
					]
				}
			}
            "#,
        );
    }
}
