use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of [`Account`]s on a specific network, most commonly
    /// the set is non-empty, since wallets guide user to create a first
    /// Account.
    Account
);



impl HasSampleValues for Accounts {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl Accounts {
    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        Self::from_iter([
            Account::sample_mainnet(),
            Account::sample_mainnet_other(),
        ])
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        Self::from_iter([
            Account::sample_stokenet_nadia(),
            Account::sample_stokenet_olivia(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Accounts;

    #[test]
    fn default_is_empty() {
        assert_eq!(SUT::default().len(), 0);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            SUT::from_iter([Account::sample(), Account::sample()].into_iter())
                .len(),
            1
        )
    }

    #[test]
    fn test_assert_elements_on_same_network_empty_is_ok_none() {
        assert_eq!(SUT::new().assert_elements_on_same_network(), Ok(None))
    }

    #[test]
    fn test_assert_elements_not_empty_and_on_same_network_err_if_empty() {
        assert_eq!(
            SUT::new().assert_elements_not_empty_and_on_same_network(),
            Err(CommonError::ExpectedNonEmptyCollection)
        )
    }

    #[test]
    fn on_same_network_mainnet() {
        assert_eq!(
            SUT::sample()
                .assert_elements_not_empty_and_on_same_network()
                .unwrap(),
            NetworkID::Mainnet
        )
    }

    #[test]
    fn on_same_network_throws_error_if_on_different_mainnet_first() {
        assert_eq!(
            SUT::from_iter([
                Account::sample_mainnet(),
                Account::sample_stokenet()
            ])
            .assert_elements_not_empty_and_on_same_network(),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Mainnet,
                actual: NetworkID::Stokenet
            })
        )
    }

    #[test]
    fn on_same_network_throws_error_if_on_different_stokenet_first() {
        assert_eq!(
            SUT::from_iter([
                Account::sample_stokenet(),
                Account::sample_mainnet()
            ])
            .assert_elements_not_empty_and_on_same_network(),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Stokenet,
                actual: NetworkID::Mainnet
            })
        )
    }

    #[test]
    fn on_same_network_stokenet() {
        assert_eq!(
            SUT::sample_other()
                .assert_elements_not_empty_and_on_same_network()
                .unwrap(),
            NetworkID::Stokenet
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(SUT::just(Account::sample()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(SUT::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let address = AccountAddress::sample();
        let account = Account::sample_with_values(
            address,
            DisplayName::default(),
            AppearanceID::default(),
        );
        let accounts = SUT::just(account.clone());
        assert_eq!(accounts.get_id(address), Some(&account));
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = SUT::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
			]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Accounts::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
			]
            "#,
        );
    }
}
