use crate::{
    identified_vec_via::IdentifiedVecVia,
    v100::{Account, AccountAddress},
};
use identified_vec::{IdentifiedVecOf, IsIdentifiableVecOfVia, IsIdentifiedVec, IsIdentifiedVecOf};

#[cfg(any(test, feature = "placeholder"))]
use wallet_kit_common::HasPlaceholder;

/// An ordered set of Accounts on a specific network, most commonly
/// the set is non-empty.
pub type Accounts = IdentifiedVecVia<Account>;

impl Accounts {
    /// Instantiates a new collection of accounts from
    /// and iterator of accounts.
    pub fn with_accounts<I>(accounts: I) -> Self
    where
        I: Iterator<Item = Account>,
    {
        Self::from_identified_vec_of(IdentifiedVecOf::from_iter(accounts))
    }

    /// Instantiates a new collection of accounts from a
    /// single account.
    pub fn with_account(account: Account) -> Self {
        Self::with_accounts([account].into_iter())
    }
}

// Trait: Default
impl Default for Accounts {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

// Getters
impl Accounts {
    /// Returns a reference to the account identified by `address`, if it exists.
    pub fn get_account_by_address(&self, address: &AccountAddress) -> Option<&Account> {
        self.get(address)
    }

    /// Returns references to **all** accounts, including hidden ones.
    pub fn get_all(&self) -> Vec<&Account> {
        self.elements()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for Accounts {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}
#[cfg(any(test, feature = "placeholder"))]
impl Accounts {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::with_accounts(
            [
                Account::placeholder_mainnet_alice(),
                Account::placeholder_mainnet_bob(),
            ]
            .into_iter(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::with_accounts(
            [
                Account::placeholder_stokenet_carol(),
                Account::placeholder_stokenet_diana(),
            ]
            .into_iter(),
        )
    }
}

#[cfg(test)]
mod tests {
    use identified_vec::IsIdentifiedVec;
    use wallet_kit_common::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use crate::v100::{Account, AccountAddress, Accounts, AppearanceID, DisplayName};

    #[test]
    fn default_is_empty() {
        assert_eq!(Accounts::default().len(), 0);
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            Accounts::with_accounts([Account::placeholder(), Account::placeholder()].into_iter())
                .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(Accounts::with_account(Account::placeholder()).len(), 1)
    }

    #[test]
    fn get_by_address() {
        let address: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();
        let account = Account::placeholder_with_values(
            address.clone(),
            DisplayName::default(),
            AppearanceID::default(),
        );
        let accounts = Accounts::with_account(account.clone());
        assert_eq!(accounts.get_account_by_address(&address), Some(&account));
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = Accounts::placeholder_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            "#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Accounts::placeholder_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            "#,
        );
    }
}
