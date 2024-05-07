use crate::prelude::*;

decl_ordered_map!(
    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    ProfileNetwork
);

impl ProfileNetworks {
    pub fn get_account(&self, address: &AccountAddress) -> Option<Account> {
        self.get(&address.network_id())
            .and_then(|n| n.accounts.get_id(address))
            .cloned()
    }

    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mut mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        self.update_with(&address.network_id(), |n| {
            _ = n.update_account(address, |a| mutate(a))
        });
        self.get_account(address)
    }
}

impl ProfileNetworks {
    pub fn content_hint(&self) -> ContentHint {
        let number_of_accounts =
            self.iter().fold(0, |acc, x| acc + x.accounts.len());
        ContentHint::with_counters(number_of_accounts, 0, self.len())
    }
}

impl HasSampleValues for ProfileNetworks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_iter([
            ProfileNetwork::sample_mainnet(),
            ProfileNetwork::sample_stokenet(),
        ])
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::just(ProfileNetwork::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inequality() {
        assert_ne!(ProfileNetworks::sample(), ProfileNetworks::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(ProfileNetworks::sample(), ProfileNetworks::sample());
        assert_eq!(
            ProfileNetworks::sample_other(),
            ProfileNetworks::sample_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            ProfileNetworks::from_iter(
                [ProfileNetwork::sample(), ProfileNetwork::sample()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn duplicates_are_prevented_and_first_added_is_retained() {
        let mut sut = ProfileNetworks::from_iter([ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::from_iter([
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob(),
            ]),
            Personas::default(),
            AuthorizedDapps::default(),
        )]);
        assert!(
            !sut.append(ProfileNetwork::new(
                NetworkID::Mainnet,
                Accounts::from_iter([Account::sample_mainnet_carol()]),
                Personas::default(),
                AuthorizedDapps::default(),
            ))
            .0
        );

        assert_eq!(
            sut.get(&NetworkID::Mainnet).unwrap().accounts.items(),
            [
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob()
            ]
        );
    }

    #[test]
    fn update_account() {
        let mut sut = ProfileNetworks::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample().address;
        assert_eq!(
            sut.get(id)
                .unwrap()
                .accounts
                .get(&account_address)
                .unwrap()
                .display_name
                .value,
            "Alice"
        );

        sut.update_account(&account_address, |a| {
            a.display_name = DisplayName::new("Stella").unwrap()
        });

        assert_eq!(
            sut.get(id)
                .unwrap()
                .accounts
                .get(&account_address)
                .unwrap()
                .display_name
                .value,
            "Stella"
        );
    }

    #[test]
    fn update_account_unknown_network() {
        let mut sut = ProfileNetworks::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample_nebunet().address;
        assert_eq!(sut.get(id).unwrap().accounts.get(&account_address), None);

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, ProfileNetworks::sample());
    }

    #[test]
    fn update_account_unknown_account() {
        let mut sut = ProfileNetworks::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample_mainnet_carol().address;
        assert_eq!(sut.get(id).unwrap().accounts.get(&account_address), None);

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, ProfileNetworks::sample());
    }

    #[test]
    fn with_network() {
        let network = ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::just(Account::sample_mainnet()),
            Personas::default(),
            AuthorizedDapps::default(),
        );
        assert_eq!(ProfileNetworks::just(network).len(), 1);
    }

    #[test]
    fn content_hint() {
        assert_eq!(
            ProfileNetworks::sample().content_hint(),
            ContentHint::with_counters(4, 0, 2)
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = ProfileNetworks::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
				{
					"networkID": 1,
					"accounts": [
						{
							"networkID": 1,
							"address": "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8",
							"displayName": "Alice",
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
														"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
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
							"address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69",
							"displayName": "Bob",
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
														"compressedData": "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
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
									"identityAddress": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
									"lastLogin": "2024-01-31T14:23:45.000Z",
									"sharedAccounts": {
										"request": {
											"quantifier": "exactly",
											"quantity": 2
										},
										"ids": [
											"account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8",
											"account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
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
									"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
									"lastLogin": "2024-01-31T14:23:45.000Z",
									"sharedAccounts": {
										"request": {
											"quantifier": "atLeast",
											"quantity": 1
										},
										"ids": [
											"account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
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
									"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
									"lastLogin": "2024-01-31T14:23:45.000Z",
									"sharedAccounts": {
										"request": {
											"quantifier": "atLeast",
											"quantity": 1
										},
										"ids": [
											"account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
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
					]
				},
				{
					"networkID": 2,
					"accounts": [
						{
							"networkID": 2,
							"address": "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
							"displayName": "Nadia",
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
														"compressedData": "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9"
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
							"address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr",
							"displayName": "Olivia",
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
														"compressedData": "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23"
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
					]
				}
			]
        	"#,
        );
    }
}
