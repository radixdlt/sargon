use crate::prelude::*;

/// Simple data representation of a Persona the user has shared with a Dapp.
/// Simple meaning "the bare minimum amount of data" that enabled `Sargon` to
/// be able to reconstruct a `AuthorizedPersonaDetailed` value, used to populate
/// views.
///
/// N.B. as of 2024-01-31 of `Sargon` we have not yet implemented the struct
/// `AuthorizedPersonaDetailed` since it is not JSON, but logic, and we have yet
/// to migrate `Sargon` into iOS/Android clients, thus we will defer the work
/// of mapping `AuthorizedPersonaSimple` -> `AuthorizedPersonaDetailed`.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedPersonaSimple {
    /// The globally unique identifier of a Persona is its address, used
    /// to lookup persona
    pub identity_address: IdentityAddress,

    /// Date of last login for this persona.
    pub last_login: Timestamp,

    /// List of "ongoing accountAddresses" that user given the dApp access to.
    pub shared_accounts: Option<SharedAccounts>,

    /// ID to PersonaData entries to user has shared with a Dapp.
    pub shared_persona_data: SharedPersonaData,
}

impl AuthorizedPersonaSimple {
    pub fn new(
        identity_address: IdentityAddress,
        last_login: Timestamp,
        shared_accounts: Option<SharedAccounts>,
        shared_persona_data: SharedPersonaData,
    ) -> Self {
        Self {
            identity_address,
            last_login,
            shared_accounts,
            shared_persona_data,
        }
    }
}

impl Identifiable for AuthorizedPersonaSimple {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address.clone()
    }
}

impl AuthorizedPersonaSimple {
    pub fn placeholder_mainnet() -> Self {
        Self::new(
            IdentityAddress::placeholder_mainnet(),
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedAccounts::placeholder_mainnet()),
            SharedPersonaData::placeholder(),
        )
    }
    pub fn placeholder_mainnet_other() -> Self {
        Self::new(
            IdentityAddress::placeholder_mainnet_other(),
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedAccounts::placeholder_other()),
            SharedPersonaData::placeholder_other(),
        )
    }

    pub fn placeholder_stokenet() -> Self {
        Self::new(
            IdentityAddress::placeholder_stokenet(),
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedAccounts::placeholder_stokenet()),
            SharedPersonaData::placeholder(),
        )
    }
    pub fn placeholder_stokenet_other() -> Self {
        Self::new(
            IdentityAddress::placeholder_stokenet_other(),
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedAccounts::placeholder_stokenet_other()),
            SharedPersonaData::placeholder_other(),
        )
    }
}
impl HasPlaceholder for AuthorizedPersonaSimple {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaSimple;

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
        let model = AuthorizedPersonaSimple::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"identityAddress": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
				"lastLogin": "2024-01-31T14:23:45.000Z",
				"sharedAccounts": {
					"request": {
						"quantifier": "exactly",
						"quantity": 2
					},
					"ids": [
						"account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease",
						"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
					]
				},
				"sharedPersonaData": {
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
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let model = AuthorizedPersonaSimple::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            			{
				"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
				"lastLogin": "2024-01-31T14:23:45.000Z",
				"sharedAccounts": {
					"request": {
						"quantifier": "atLeast",
						"quantity": 1
					},
					"ids": [
						"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master"
					]
				},
				"sharedPersonaData": {
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
			}
            "#,
        );
    }
}
