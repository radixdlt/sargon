use crate::prelude::*;

/// Accounts, Personas, Authorized dapps for some Radix Network that user
/// has created and interacted with.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    #[serde(rename = "networkID")]
    pub id: NetworkID,

    /// An ordered set of Accounts on this network.
    pub accounts: Accounts,
}

impl Identifiable for Network {
    type ID = NetworkID;
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    fn id(&self) -> NetworkID {
        self.id.clone()
    }
}

impl Network {
    /// Instantiates a new `Network` from `network_id` and `accounts`.
    ///
    /// Panics if not any account in `accounts` is on another
    /// network than `network_id`
    pub fn new(network_id: NetworkID, accounts: Accounts) -> Self {
        assert!(
            accounts
                .get_all()
                .into_iter()
                .all(|a| a.network_id == network_id),
            "Discrepancy, found accounts on other network than {network_id}"
        );
        Self {
            id: network_id,
            accounts,
        }
    }
}

impl Network {
    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(&mut self, address: &AccountAddress, mutate: F) -> Option<Account>
    where
        F: FnMut(&mut Account) -> (),
    {
        if self.accounts.update_with(address, mutate) {
            self.accounts.get(address).cloned()
        } else {
            None
        }
    }
}

impl HasPlaceholder for Network {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl Network {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::new(NetworkID::Mainnet, Accounts::placeholder_mainnet())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::new(NetworkID::Stokenet, Accounts::placeholder_stokenet())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn inequality() {
        assert_ne!(Network::placeholder(), Network::placeholder_other());
    }

    #[test]
    fn get_id() {
        assert_eq!(Network::placeholder().id(), NetworkID::Mainnet);
    }

    #[test]
    fn get_accounts() {
        let sut = Network::placeholder();
        assert_eq!(sut.accounts, Accounts::placeholder());
    }

    #[test]
    fn duplicate_accounts_are_filtered_out() {
        assert_eq!(
            Network::new(
                NetworkID::Mainnet,
                Accounts::with_accounts(
                    [Account::placeholder(), Account::placeholder()].into_iter()
                )
            )
            .accounts
            .len(),
            1
        )
    }

    #[test]
    #[should_panic(expected = "Discrepancy, found accounts on other network than mainnet")]
    fn panic_when_network_id_mismatch_between_accounts_and_value() {
        Network::new(
            NetworkID::Mainnet,
            Accounts::with_accounts(
                [
                    Account::placeholder_mainnet(),
                    Account::placeholder_stokenet(),
                ]
                .into_iter(),
            ),
        );
    }

    #[test]
    fn json_roundtrip_placeholder_stokenet() {
        let sut = Network::placeholder_stokenet();
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
						"flags": [],
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
						"flags": [],
						"displayName": "Diana",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"flags": [],
						"address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
					}
				]
			}
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_mainnet() {
        let sut = Network::placeholder_mainnet();
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
						"flags": [],
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
						"flags": [],
						"displayName": "Bob",
						"onLedgerSettings": {
							"thirdPartyDeposits": {
								"depositRule": "acceptAll",
								"assetsExceptionList": [],
								"depositorsAllowList": []
							}
						},
						"flags": [],
						"address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
					}
				]
			}
            "#,
        );
    }
}
