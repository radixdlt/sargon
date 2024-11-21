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
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.description())]
pub struct AuthorizedPersonaSimple {
    /// The globally unique identifier of a Persona is its address, used
    /// to lookup persona
    pub identity_address: IdentityAddress,

    /// Date of last login for this persona.
    pub last_login: Timestamp,

    /// List of "ongoing accountAddresses" that user given the dApp access to.
    pub shared_accounts: Option<SharedToDappWithPersonaAccountAddresses>,

    /// ID to PersonaData entries to user has shared with a Dapp.
    pub shared_persona_data: SharedPersonaData,
}

impl AuthorizedPersonaSimple {
    /// Removes the referenced account from the shared accounts
    pub(crate) fn remove_shared_account(
        &mut self,
        account_address: &AccountAddress,
    ) -> bool {
        if let Some(shared_accounts) = self.shared_accounts.as_mut() {
            shared_accounts.remove_entry(account_address);
            true
        } else {
            false
        }
    }
}

impl AuthorizedPersonaSimple {
    pub fn description(&self) -> String {
        format!(
            r#"
			identity_address: {}
			last_login: {}
			shared_accounts: {}
			shared_persona_data: {}
			"#,
            self.identity_address,
            self.last_login,
            self.shared_accounts
                .clone()
                .map(|s| s.to_string())
                .unwrap_or("<NONE>".to_owned()),
            self.shared_persona_data,
        )
    }

    pub fn new(
        identity_address: IdentityAddress,
        last_login: Timestamp,
        shared_accounts: impl Into<Option<SharedToDappWithPersonaAccountAddresses>>,
        shared_persona_data: SharedPersonaData,
    ) -> Self {
        Self {
            identity_address,
            last_login,
            shared_accounts: shared_accounts.into(),
            shared_persona_data,
        }
    }
}

impl Identifiable for AuthorizedPersonaSimple {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address
    }
}

impl IsNetworkAware for AuthorizedPersonaSimple {
    fn network_id(&self) -> NetworkID {
        self.identity_address.network_id()
    }
}

impl PersonaData {
    pub(crate) fn shared_everything(&self) -> SharedPersonaData {
        SharedPersonaData::new(
            self.name.clone().map(|x| x.id),
            SharedToDappWithPersonaIDsOfPersonaDataEntries::exactly(
                self.email_addresses.ids().into_iter().cloned(),
            ),
            SharedToDappWithPersonaIDsOfPersonaDataEntries::exactly(
                self.phone_numbers.ids().into_iter().cloned(),
            ),
        )
    }
}

impl AuthorizedPersonaSimple {
    pub fn sample_mainnet() -> Self {
        let persona = Persona::sample_mainnet();
        Self::new(
            persona.address,
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedToDappWithPersonaAccountAddresses::sample_mainnet()),
            persona.persona_data.shared_everything(),
        )
    }
    pub fn sample_mainnet_other() -> Self {
        let persona = Persona::sample_mainnet_other();
        Self::new(
            persona.address,
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedToDappWithPersonaAccountAddresses::sample_other()),
            persona.persona_data.shared_everything(),
        )
    }

    pub fn sample_stokenet() -> Self {
        let persona = Persona::sample_stokenet();
        Self::new(
            persona.address,
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(SharedToDappWithPersonaAccountAddresses::sample_stokenet()),
            persona.persona_data.shared_everything(),
        )
    }
    pub fn sample_stokenet_other() -> Self {
        let persona = Persona::sample_stokenet_other();
        Self::new(
            persona.address,
            Timestamp::parse("2024-01-31T14:23:45Z").unwrap(),
            Some(
                SharedToDappWithPersonaAccountAddresses::sample_stokenet_other(
                ),
            ),
            persona.persona_data.shared_everything(),
        )
    }
}
impl HasSampleValues for AuthorizedPersonaSimple {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaSimple;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = AuthorizedPersonaSimple::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
                "identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
                "lastLogin": "2024-01-31T14:23:45.000Z",
                "sharedAccounts": {
                    "request": {
                        "quantifier": "exactly",
                        "quantity": 2
                    },
                    "ids": [
                        "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
                        "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
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
                            "00000000-0000-0000-0000-000000000003",
                            "00000000-0000-0000-0000-000000000004"
                        ]
                    },
                    "phoneNumbers": {
                        "request": {
                            "quantifier": "exactly",
                            "quantity": 2
                        },
                        "ids": [
                            "00000000-0000-0000-0000-000000000001",
                            "00000000-0000-0000-0000-000000000002"
                        ]
                    }
                }
            }
            "#,
        );
    }

    #[test]
    fn json_roundtrip_sample_other() {
        let model = AuthorizedPersonaSimple::sample_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
                "lastLogin": "2024-01-31T14:23:45.000Z",
                "sharedAccounts": {
                    "request": {
                        "quantifier": "atLeast",
                        "quantity": 1
                    },
                    "ids": [
                        "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
                    ]
                },
                "sharedPersonaData": {
                    "name": "00000000-0000-0000-0000-000000000000",
                    "emailAddresses": {
                        "request": {
                            "quantifier": "exactly",
                            "quantity": 1
                        },
                        "ids": [
                            "00000000-0000-0000-0000-000000000002"
                        ]
                    },
                    "phoneNumbers": {
                        "request": {
                            "quantifier": "exactly",
                            "quantity": 1
                        },
                        "ids": [
                            "00000000-0000-0000-0000-000000000001"
                        ]
                    }
                }
            }
            "#,
        );
    }
}
