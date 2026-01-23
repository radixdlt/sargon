use crate::prelude::*;

/// [`Accounts`], [`Personas`] and [`AuthorizedDapps`] for some [`ProfileNetwork`]
/// which user has created/interacted with, all on the same [Radix Network][`NetworkDefinition`],
/// identified by `id` ([`NetworkID`]).
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[display("{}", self.description())]
pub struct ProfileNetwork {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    #[serde(rename = "networkID")]
    pub id: NetworkID,

    /// An ordered set of [`Accounts`]` on this network, which are [`Account`]s
    /// the user has created on this network.
    pub accounts: Accounts,

    /// An ordered set of [`Personas`] on this network, which are [`Persona`]s
    /// the user has created on this network.
    pub personas: Personas,

    /// An ordered set of [`AuthorizedDapps`] on this network, which are
    /// [`AuthorizedDapp`]s that the user has interacted with.
    #[serde(rename = "authorizedDapps")]
    pub authorized_dapps: AuthorizedDapps,

    /// Configuration related to resources
    #[serde(default)]
    pub resource_preferences: ResourcePreferences,

    /// Pre-derived MFA factor instances
    #[serde(default)]
    pub mfa_factor_instances: MFAFactorInstances,
}

impl IsNetworkAware for ProfileNetwork {
    fn network_id(&self) -> NetworkID {
        self.id
    }
}

impl ProfileNetwork {
    pub fn description(&self) -> String {
        format!(
            r#"
			id: {}
			accounts: {}
			personas: {}
			authorized_dapps: {}
			resource_preferences: {:?}
			mfa_factor_instances: {:?}
			"#,
            self.id,
            self.accounts,
            self.personas,
            self.authorized_dapps,
            self.resource_preferences,
            self.mfa_factor_instances,
        )
    }
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
    /// Instantiates a new `ProfileNetwork` from `network_id`, `accounts`, `personas`
    /// and `authorized_dapps`.
    ///
    /// # Panic
    /// Panics if not all account in `accounts` are on network with id `network_id`,
    /// and same for `personas` and `authorized_dapps`.
    pub fn new(
        network_id: impl Into<NetworkID>,
        accounts: impl IntoIterator<Item = Account>,
        personas: impl IntoIterator<Item = Persona>,
        authorized_dapps: impl Into<AuthorizedDapps>,
        resource_preferences: impl Into<ResourcePreferences>,
        mfa_factor_instances: impl Into<MFAFactorInstances>,
    ) -> Self {
        let network_id = network_id.into();
        let accounts = accounts.into_iter().collect::<Accounts>();
        let personas = personas.into_iter().collect::<Personas>();
        let authorized_dapps = authorized_dapps.into();
        let resource_preferences = resource_preferences.into();
        let mfa_factor_instances = mfa_factor_instances.into();
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
        assert!(
            authorized_dapps
                .get_all()
                .into_iter()
                .all(|d| d.network_id == network_id),
            "Discrepancy, found an AuthorizedDapp on other network than {network_id}"
        );

        assert!(
            resource_preferences
                .get_all()
                .into_iter()
                .all(|d| d.network_id() == network_id),
            "Discrepancy, found a ResourceAppPreference on other network than {network_id}"
        );

        assert!(
            mfa_factor_instances
                .get_all()
                .into_iter()
                .all(|d| d.factor_instance.badge.network_id() == network_id),
            "Discrepancy, found a MFAFactorInstance on other network than {network_id}"
        );

        Self {
            id: network_id,
            accounts,
            personas,
            authorized_dapps,
            resource_preferences,
            mfa_factor_instances,
        }
    }

    /// Instantiates a new empty `ProfileNetwork` from `network_id`, i.e.
    /// Accounts, Personas, AuthorizedDapps all being empty.
    pub fn new_empty_on(network_id: impl Into<NetworkID>) -> Self {
        Self::new(
            network_id,
            Accounts::new(),
            Personas::new(),
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
            MFAFactorInstances::new(),
        )
    }

    /// Instantiates a new `ProfileNetwork` from `network_id` and `accounts`, with all
    /// the rest i.e. Personas, AuthorizedDapps all being empty.
    pub fn new_with_accounts(
        network_id: impl Into<NetworkID>,
        accounts: impl IntoIterator<Item = Account>,
    ) -> Self {
        Self::new(
            network_id,
            accounts,
            Personas::new(),
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
            MFAFactorInstances::new(),
        )
    }
}

impl HasSampleValues for ProfileNetwork {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl ProfileNetwork {
    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        Self::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::sample_mainnet(),
            AuthorizedDapps::sample_mainnet(),
            ResourcePreferences::sample_mainnet(),
            MFAFactorInstances::sample_mainnet(),
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        Self::new(
            NetworkID::Stokenet,
            Accounts::sample_stokenet(),
            Personas::sample_stokenet(),
            AuthorizedDapps::sample_stokenet(),
            ResourcePreferences::sample_stokenet(),
            MFAFactorInstances::sample_stokenet(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn get_id() {
        assert_eq!(SUT::sample().id(), NetworkID::Mainnet);
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn get_accounts() {
        let sut = SUT::sample();
        assert_eq!(sut.accounts, Accounts::sample());
    }

    #[test]
    fn get_resources() {
        assert_eq!(
            SUT::sample().resource_preferences,
            ResourcePreferences::sample()
        )
    }

    #[test]
    fn duplicate_accounts_are_filtered_out() {
        assert_eq!(
            SUT::new(
                NetworkID::Mainnet,
                Accounts::from_iter(
                    [Account::sample(), Account::sample()].into_iter()
                ),
                Personas::default(),
                AuthorizedDapps::default(),
                ResourcePreferences::default(),
                MFAFactorInstances::default(),
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
        SUT::new(
            NetworkID::Mainnet,
            Accounts::just(Account::sample_stokenet()),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found a Persona on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_persona_and_value() {
        SUT::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::just(Persona::sample_stokenet()),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found an AuthorizedDapp on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_authorized_dapp_and_value() {
        SUT::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::sample_mainnet(),
            AuthorizedDapps::just(AuthorizedDapp::sample_stokenet()),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found a ResourceAppPreference on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_resource_preferences_and_value() {
        SUT::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::sample_mainnet(),
            AuthorizedDapps::sample_mainnet(),
            ResourcePreferences::from_iter([
                ResourceAppPreference::sample_non_fungible_stokenet(),
            ]),
            MFAFactorInstances::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found an AuthorizedDapp on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_authorized_dapps_and_value() {
        SUT::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::sample_mainnet(),
            AuthorizedDapps::just(AuthorizedDapp::sample_stokenet()),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, found an MFAFactorInstance on other network than mainnet"
    )]
    fn panic_when_network_id_mismatch_between_mfa_factor_instances_and_value() {
        SUT::new(
            NetworkID::Mainnet,
            Accounts::sample_mainnet(),
            Personas::sample_mainnet(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
            MFAFactorInstances::just(
                MFAFactorInstance::sample_stokenet_account_securified_idx_0(),
            ),
        );
    }

    #[test]
    fn json_roundtrip_sample_mainnet() {
        let sut = SUT::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
				"networkID": 1,
				"accounts": [
					{
						"networkID": 1,
						"address": "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
						"displayName": "Alice",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/525H/1460H/0H"
												}
											}
										}
									}
								}
							}
						},
						"appearanceID": 0,
						"flags": [],
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						}
					},
					{
						"networkID": 1,
						"address": "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7",
						"displayName": "Bob",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "a3a14ce3c0e549ac35f1875738c243bb6f4037f08d7d2a52ef749091a92a0c71"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/1H/525H/1460H/1H"
												}
											}
										}
									}
								}
							}
						},
						"appearanceID": 1,
						"flags": [],
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						}
					}
				],
				"personas": [
					{
						"networkID": 1,
						"address": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
						"displayName": "Satoshi",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "e284e28bfca2103d554854d7cce822a2682610eb16b4c27bcd1b9cbd78bb931a"
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
						"address": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
						"displayName": "Batman",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "675aa54df762f24df8f6b38122e75058a18fe55a3dbb030b4c0bb504bacc7e81"
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
						"flags": [],
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
							},
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
						]
					},
					{
						"networkID": 1,
						"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
						"displayName": "Gumball Club",
						"referencesToAuthorizedPersonas": [
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
						]
					}
				],
                "resource_preferences": [
                    {
                        "resource": {
                            "kind": "fungible",
                            "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                        },
                        "visibility": "hidden"
                    },
                    {
                        "resource": {
                            "kind": "nonFungible",
                            "value": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                        },
                        "visibility": "visible"
                    }
                ]
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_sample_stokenet() {
        let sut = SUT::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
				"networkID": 2,
				"accounts": [
					{
						"networkID": 2,
						"address": "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
						"displayName": "Nadia",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "535e0b74beffc99d96acd36ae73444c0e35ebb5707f077f9bf1120b1bb8894c0"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/525H/1460H/0H"
												}
											}
										}
									}
								}
							}
						},
						"appearanceID": 0,
						"flags": [],
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						}
					},
					{
						"networkID": 2,
						"address": "account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp",
						"displayName": "Olivia",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "436c67c678713be6a4306bf2a64d62d29c9bccb92a776175e5cb6e95e87be55d"
												},
												"derivationPath": {
													"scheme": "cap26",
													"path": "m/44H/1022H/2H/525H/1460H/1H"
												}
											}
										}
									}
								}
							}
						},
						"appearanceID": 1,
						"flags": [
							"deletedByUser"
						],
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						}
					}
				],
				"personas": [
					{
						"networkID": 2,
						"address": "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8",
						"displayName": "Skywalker",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "d3dd2992834813ba76d6619021560b759e81f7391a5cdbb8478feb3bfa8cb9e4"
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
						"address": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
						"displayName": "Granger",
						"securityState": {
							"discriminator": "unsecured",
							"unsecuredEntityControl": {
								"transactionSigning": {
									"factorSourceID": {
										"discriminator": "fromHash",
										"fromHash": {
											"kind": "device",
											"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
										}
									},
									"badge": {
										"discriminator": "virtualSource",
										"virtualSource": {
											"discriminator": "hierarchicalDeterministicPublicKey",
											"hierarchicalDeterministicPublicKey": {
												"publicKey": {
													"curve": "curve25519",
													"compressedData": "c287e135eac194e4d6b6c65a2545988686b941509043bab026ef9717fd6b4f4e"
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
						"flags": [
							"deletedByUser"
						],
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
								"identityAddress": "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "exactly",
										"quantity": 2
									},
									"ids": [
										"account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
										"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
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
							},
							{
								"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
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
						]
					},
					{
						"networkID": 2,
						"dAppDefinitionAddress": "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
						"displayName": "Sandbox",
						"referencesToAuthorizedPersonas": [
							{
								"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
								"lastLogin": "2024-01-31T14:23:45.000Z",
								"sharedAccounts": {
									"request": {
										"quantifier": "atLeast",
										"quantity": 1
									},
									"ids": [
										"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
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
						]
					}
				],
                "resource_preferences": [
                    {
                        "resource": {
                            "kind": "nonFungible",
                            "value": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
                        },
                        "visibility": "visible"
                    }
                ]
			}
            "#,
        )
    }
}
