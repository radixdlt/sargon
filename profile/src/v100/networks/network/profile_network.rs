use crate::prelude::*;

/// Accounts, Personas, Authorized dapps for some Radix Network that user
/// has created and interacted with.
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct ProfileNetwork {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    #[serde(rename = "networkID")]
    pub id: NetworkID,

    /// An ordered set of Accounts on this network.
    pub accounts: Accounts,

    /// An ordered set of Personas on this network.
    pub personas: Personas,

    #[serde(rename = "authorizedDapps")]
    pub authorized_dapps: AuthorizedDapps,
}

impl Identifiable for ProfileNetwork {
    type ID = NetworkID;
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    fn id(&self) -> NetworkID {
        self.id
    }
}

impl ProfileNetwork {
    /// Instantiates a new `Network` from `network_id` and `accounts`.
    ///
    /// Panics if not any account in `accounts` is on another
    /// network than `network_id`
    pub fn new(
        network_id: NetworkID,
        accounts: Accounts,
        personas: Personas,
        authorized_dapps: AuthorizedDapps,
    ) -> Self {
        assert!(
            accounts
                .get_all()
                .into_iter()
                .all(|a| a.network_id == network_id),
            "Discrepancy, found an Account on other network than {network_id}"
        );
        assert!(
            personas
                .get_all()
                .into_iter()
                .all(|p| p.network_id == network_id),
            "Discrepancy, found a Persona on other network than {network_id}"
        );
        Self {
            id: network_id,
            accounts,
            personas,
            authorized_dapps,
        }
    }
}

impl ProfileNetwork {
    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        if self.accounts.update_with(address, mutate) {
            self.accounts.get(address).cloned()
        } else {
            None
        }
    }
}

impl HasPlaceholder for ProfileNetwork {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl ProfileNetwork {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::new(
            NetworkID::Mainnet,
            Accounts::placeholder_mainnet(),
            Personas::placeholder_mainnet(),
            AuthorizedDapps::placeholder_mainnet(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::new(
            NetworkID::Stokenet,
            Accounts::placeholder_stokenet(),
            Personas::placeholder_stokenet(),
            AuthorizedDapps::placeholder_stokenet(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn inequality() {
        assert_ne!(
            ProfileNetwork::placeholder(),
            ProfileNetwork::placeholder_other()
        );
    }

    #[test]
    fn get_id() {
        assert_eq!(ProfileNetwork::placeholder().id(), NetworkID::Mainnet);
    }

    #[test]
    fn get_accounts() {
        let sut = ProfileNetwork::placeholder();
        assert_eq!(sut.accounts, Accounts::placeholder());
    }

    #[test]
    fn duplicate_accounts_are_filtered_out() {
        assert_eq!(
            ProfileNetwork::new(
                NetworkID::Mainnet,
                Accounts::with_accounts(
                    [Account::placeholder(), Account::placeholder()]
                        .into_iter()
                ),
                Personas::default(),
                AuthorizedDapps::default(),
            )
            .accounts
            .len(),
            1
        )
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found an Account on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_accounts_and_value() {
        ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::with_accounts([
                Account::placeholder_mainnet(),
                Account::placeholder_stokenet(),
            ]),
            Personas::default(),
            AuthorizedDapps::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found a Persona on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_persona_and_value() {
        ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::placeholder_mainnet(),
            Personas::from_iter([Persona::placeholder_stokenet_hermione()]),
            AuthorizedDapps::default(),
        );
    }

    #[test]
    fn json_roundtrip_placeholder_mainnet() {
        let sut = ProfileNetwork::placeholder_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
				"networkID": 1,
				"accounts": [
					{
						"securityState": {
							"unsecuredEntityControl": {
								"transactionSigning": {
									"badge": {
										"virtualSource": {
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/525H/1460H/0H"
												}
											},
											"discriminator": "hierarchicalDeterministicPublicKey"
										},
										"discriminator": "virtualSource"
									},
									"factorSourceID": {
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										},
										"discriminator": "fromHash"
									}
								}
							},
							"discriminator": "unsecured"
						},
						"networkID": 1,
						"appearanceID": 0,
						"flags": [],
						"displayName": "Alice",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"address": "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
					},
					{
						"securityState": {
							"unsecuredEntityControl": {
								"transactionSigning": {
									"badge": {
										"virtualSource": {
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/525H/1460H/1H"
												}
											},
											"discriminator": "hierarchicalDeterministicPublicKey"
										},
										"discriminator": "virtualSource"
									},
									"factorSourceID": {
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										},
										"discriminator": "fromHash"
									}
								}
							},
							"discriminator": "unsecured"
						},
						"networkID": 1,
						"appearanceID": 1,
						"flags": ["deletedByUser"],
						"displayName": "Bob",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
					}
				],
				"personas": [
					{
						"networkID": 1,
						"address": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
						"displayName": "Satoshi",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "983ab1d3a77dd6b30bb8a5d59d490a0380cc0aa9ab464983d3fc581fcf64543f"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/618H/1460H/0H"
												}
											}
										}
									}
								}
							}
						},
						"flags": [],
						"personaData": {
							"name": {
								"id": "00000000-0000-0000-0000-000000000000",
								"value": {
									"variant": "eastern",
									"familyName": "Nakamoto",
									"givenNames": "Satoshi",
									"nickname": "Satoshi"
								}
							},
							"phoneNumbers": [
								{
									"id": "00000000-0000-0000-0000-000000000001",
									"value": "+46123456789"
								},
								{
									"id": "00000000-0000-0000-0000-000000000002",
									"value": "+44987654321"
								}
							],
							"emailAddresses": [
								{
									"id": "00000000-0000-0000-0000-000000000003",
									"value": "sat@os.hi"
								},
								{
									"id": "00000000-0000-0000-0000-000000000004",
									"value": "satoshi@nakamoto.btc"
								}
							]
						}
					},
					{
						"networkID": 1,
						"address": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
						"displayName": "Batman",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "1fe80badc0520334ee339e4010491d417ca3aed0c9621698b10655529f0ee506"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/618H/1460H/1H"
												}
											}
										}
									}
								}
							}
						},
						"flags": ["deletedByUser"],
						"personaData": {
							"name": {
								"id": "00000000-0000-0000-0000-000000000000",
								"value": {
									"variant": "western",
									"familyName": "Wayne",
									"givenNames": "Bruce",
									"nickname": "Batman"
								}
							},
							"phoneNumbers": [
								{
									"id": "00000000-0000-0000-0000-000000000001",
									"value": "+1 13 371 337"
								}
							],
							"emailAddresses": [
								{
									"id": "00000000-0000-0000-0000-000000000002",
									"value": "bat@m.an"
								}
							]
						}
					}
				],
				"authorizedDapps": [
					{
						"networkID": 1,
						"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
						"displayName": "Radix Dashboard",
						"referencesToAuthorizedPersonas": [
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
								}
							},
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
								}
							}
						]
					},
					{
						"networkID": 1,
						"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
						"displayName": "Gumball Club",
						"referencesToAuthorizedPersonas": [
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
								}
							}
						]
					}
				]
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_placeholder_stokenet() {
        let sut = ProfileNetwork::placeholder_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
				"networkID": 2,
				"accounts": [
					{
						"securityState": {
							"unsecuredEntityControl": {
								"transactionSigning": {
									"badge": {
										"virtualSource": {
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/525H/1460H/0H"
												}
											},
											"discriminator": "hierarchicalDeterministicPublicKey"
										},
										"discriminator": "virtualSource"
									},
									"factorSourceID": {
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										},
										"discriminator": "fromHash"
									}
								}
							},
							"discriminator": "unsecured"
						},
						"networkID": 2,
						"appearanceID": 0,
						"flags": [],
						"displayName": "Carol",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"address": "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql"
					},
					{
						"securityState": {
							"unsecuredEntityControl": {
								"transactionSigning": {
									"badge": {
										"virtualSource": {
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/525H/1460H/1H"
												}
											},
											"discriminator": "hierarchicalDeterministicPublicKey"
										},
										"discriminator": "virtualSource"
									},
									"factorSourceID": {
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										},
										"discriminator": "fromHash"
									}
								}
							},
							"discriminator": "unsecured"
						},
						"networkID": 2,
						"appearanceID": 1,
						"flags": ["deletedByUser"],
						"displayName": "Diana",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
					}
				],
				"personas": [
					{
						"networkID": 2,
						"address": "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc",
						"displayName": "Skywalker",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "3c4d6f1267485854313c1ed81aea193b8f750cd081e3aa4dea29b93c34ca2261"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/618H/1460H/0H"
												}
											}
										}
									}
								}
							}
						},
						"flags": [],
						"personaData": {
							"name": {
								"id": "00000000-0000-0000-0000-000000000000",
								"value": {
									"variant": "eastern",
									"familyName": "Skywalker",
									"givenNames": "Leia",
									"nickname": "Princess Leia"
								}
							},
							"phoneNumbers": [
								{
									"id": "00000000-0000-0000-0000-000000000001",
									"value": "+42 3 456 789"
								}
							],
							"emailAddresses": [
								{
									"id": "00000000-0000-0000-0000-000000000002",
									"value": "leia@sky.walker"
								}
							]
						}
					},
					{
						"networkID": 2,
						"address": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
						"displayName": "Granger",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "b6885032393165d56cce19850c2a3dbb80733d21c78c7314223e9c3a75f64c8d"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/618H/1460H/1H"
												}
											}
										}
									}
								}
							}
						},
						"flags": ["deletedByUser"],
						"personaData": {
							"name": {
								"id": "00000000-0000-0000-0000-000000000000",
								"value": {
									"variant": "western",
									"familyName": "Granger",
									"givenNames": "Hermione",
									"nickname": "Hermy"
								}
							},
							"phoneNumbers": [
								{
									"id": "00000000-0000-0000-0000-000000000001",
									"value": "+44 123 456 77"
								}
							],
							"emailAddresses": [
								{
									"id": "00000000-0000-0000-0000-000000000002",
									"value": "granger.h@hogwarts.uk.co"
								}
							]
						}
					}
				],
				"authorizedDapps": [
					{
						"networkID": 2,
						"dAppDefinitionAddress": "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4",
						"displayName": "Dev Console",
						"referencesToAuthorizedPersonas": [
							{
								"identityAddress": "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
										"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
									]
								}
							},
							{
								"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
									]
								}
							}
						]
					},
					{
						"networkID": 2,
						"dAppDefinitionAddress": "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
						"displayName": "Sandbox",
						"referencesToAuthorizedPersonas": [
							{
								"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
									]
								}
							}
						]
					}
				]
			}
            "#,
        )
    }
}
