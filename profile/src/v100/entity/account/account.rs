use crate::prelude::*;

/// A network unique account with a unique public address and a set of cryptographic
/// factors used to control it.
///
/// Used to own and control assets on the radix network. Uniquely identified by an
/// account address, e.g.
///
/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
///
/// But most commonly users see the address on its abbreviated form:
///
/// `acco...please`
///
/// Accounts have a display name and an appearance id.
///
/// An account can be either controlled by a "Babylon" DeviceFactorSource or a
/// Legacy one imported from Olympia, or a Ledger hardware wallet, which too might
/// have been imported from Olympia.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{display_name} | {address}")]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// The ID of the network this account can be used with.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// A globally unique identifier of this account, being a human readable
    /// address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...please`
    ///
    /// No two addresses will ever be the same even for the same factor source
    /// but on different networks, since the public keys controlling the
    /// accounts depend on the network id.
    pub address: AccountAddress,

    /// An off-ledger display name or description chosen by the user when she
    /// created this account.
    pub display_name: DisplayName,

    /// Security state of this account, either "securified" or not.
    pub security_state: EntitySecurityState,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    #[serde(rename = "appearanceID")]
    pub appearance_id: AppearanceID,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    #[serde(default)]
    pub flags: EntityFlags,

    /// The on ledger synced settings for this account, contains e.g.
    /// ThirdPartyDeposit settings, with deposit rules for assets.
    pub on_ledger_settings: OnLedgerSettings,
}

impl Account {
    pub fn new(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        let address =
            AccountAddress::from_hd_factor_instance_virtual_entity_creation(
                account_creating_factor_instance.clone(),
            );
        Self {
            network_id: account_creating_factor_instance.network_id(),
            address,
            display_name,
            security_state:
                UnsecuredEntityControl::with_entity_creating_factor_instance(
                    account_creating_factor_instance,
                )
                .into(),
            appearance_id,
            flags: EntityFlags::default(),
            on_ledger_settings: OnLedgerSettings::default(),
        }
    }
}

impl Identifiable for Account {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Account {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.security_state, &other.security_state) {
            (
                EntitySecurityState::Unsecured { value: l },
                EntitySecurityState::Unsecured { value: r },
            ) => l
                .transaction_signing
                .derivation_path()
                .last_component()
                .cmp(r.transaction_signing.derivation_path().last_component()),
        }
    }
}

impl HasPlaceholder for Account {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_other()
    }
}

impl Account {
    /// Instantiates an account with a display name, address and appearance id.
    pub fn placeholder_with_values(
        address: AccountAddress,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        Self {
            network_id: address.network_id,
            address,
            display_name,
            appearance_id,
            flags: EntityFlags::default(),
            on_ledger_settings: OnLedgerSettings::default(),
            security_state: EntitySecurityState::placeholder(),
        }
    }

    fn placeholder_at_index_name_network(
        network_id: NetworkID,
        index: HDPathValue,
        name: &str,
        is_hidden: bool,
    ) -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let bdfs = DeviceFactorSource::babylon(
            true,
            mwp.clone(),
            WalletClientModel::Iphone,
        );
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);
        let account_creating_factor_instance = private_hd_factor_source
            .derive_entity_creation_factor_instance(network_id, index);

        let mut account = Self::new(
            account_creating_factor_instance,
            DisplayName::new(name).unwrap(),
            AppearanceID::try_from(index as u8).unwrap(),
        );
        if is_hidden {
            account.flags.insert_flag(EntityFlag::DeletedByUser);
        }
        account
    }

    fn placeholder_at_index_name(
        index: HDPathValue,
        name: &str,
        is_hidden: bool,
    ) -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Mainnet,
            index,
            name,
            is_hidden,
        )
    }

    /// A `Mainnet` account named "Alice", a placeholder used to facilitate unit tests, with
    /// derivation index 0,
    pub fn placeholder_mainnet_alice() -> Self {
        Self::placeholder_at_index_name(0, "Alice", false)
    }

    /// A `Mainnet` account named "Bob", a placeholder used to facilitate unit tests, with
    /// derivation index 1.
    pub fn placeholder_mainnet_bob() -> Self {
        Self::placeholder_at_index_name(1, "Bob", true)
    }

    /// A `Mainnet` account named "Carol", a placeholder used to facilitate unit tests, with
    /// derivation index 2.
    pub fn placeholder_mainnet_carol() -> Self {
        Self::placeholder_at_index_name(2, "Carol", false)
    }

    /// A `Mainnet` account named "Alice", a placeholder used to facilitate unit tests, with
    /// derivation index 0,
    pub fn placeholder_alice() -> Self {
        Self::placeholder_mainnet_alice()
    }

    /// A `Mainnet` account named "Bob", a placeholder used to facilitate unit tests, with
    /// derivation index 1.
    pub fn placeholder_bob() -> Self {
        Self::placeholder_mainnet_bob()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::placeholder_mainnet_alice()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet_other() -> Self {
        Self::placeholder_mainnet_bob()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_carol() -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            0,
            "Carol",
            false,
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_diana() -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            1,
            "Diana",
            true,
        )
    }

    pub fn placeholder_stokenet() -> Self {
        Self::placeholder_stokenet_carol()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_nebunet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_b_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08m9raqq"
                .parse()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_kisharnet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_c_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej0898vkq9"
                .parse()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_adapanet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_a_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08srjqq0"
                .parse()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        assert_eq_after_json_roundtrip, AssetException,
        DepositAddressExceptionRule, DepositRule, DepositorAddress, EntityFlag,
        EntityFlags, HasPlaceholder, OnLedgerSettings, ThirdPartyDeposits,
    };
    use identified_vec::IsIdentifiedVec;
    use radix_engine_common::prelude::HashSet;

    use crate::v100::{AccountAddress, AppearanceID, DisplayName};

    use super::Account;

    #[test]
    fn equality() {
        assert_eq!(Account::placeholder(), Account::placeholder());
        assert_eq!(Account::placeholder_other(), Account::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Account::placeholder(), Account::placeholder_other());
    }

    #[test]
    fn new_with_address_only() {
        let address: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .parse()
                .unwrap();
        let account = Account::placeholder_with_values(
            address.clone(),
            DisplayName::default(),
            AppearanceID::default(),
        );
        assert_eq!(account.address, address);
    }

    #[test]
    fn display() {
        let account = Account::placeholder();
        assert_eq!(
            format!("{account}"),
            "Alice | account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
        );
    }

    #[test]
    fn compare() {
        assert!(Account::placeholder_alice() < Account::placeholder_bob());
    }

    #[test]
    fn update() {
        let mut account = Account::placeholder();
        assert_eq!(account.display_name.value, "Alice");
        account.display_name = DisplayName::new("Satoshi").unwrap();
        assert_eq!(account.display_name.value, "Satoshi");
    }

    #[test]
    fn on_ledger_settings_get_set() {
        let mut account = Account::placeholder_with_values(
            AccountAddress::placeholder_alice(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.on_ledger_settings, OnLedgerSettings::default());
        let excp1 = AssetException::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let excp2 = AssetException::new(
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let new_third_party_dep = ThirdPartyDeposits::with_rule_and_lists(
            DepositRule::DenyAll,
            [excp1, excp2],
            [DepositorAddress::ResourceAddress {
                value: "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                    .parse()
                    .unwrap(),
            }],
        );
        let new_on_ledger_settings = OnLedgerSettings::new(new_third_party_dep);
        account.on_ledger_settings = new_on_ledger_settings.clone();
        assert_eq!(account.on_ledger_settings, new_on_ledger_settings);

        assert_eq!(
            account.on_ledger_settings.third_party_deposits.deposit_rule,
            DepositRule::DenyAll
        );

        account.on_ledger_settings.third_party_deposits.deposit_rule =
            DepositRule::AcceptAll;
        assert_eq!(
            account.on_ledger_settings.third_party_deposits.deposit_rule,
            DepositRule::AcceptAll
        );
    }

    #[test]
    fn json_roundtrip_mainnet_alice() {
        let model = Account::placeholder_mainnet_alice();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet_bob() {
        let model = Account::placeholder_mainnet_bob();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_carol() {
        let model = Account::placeholder_stokenet_carol();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_diana() {
        let model = Account::placeholder_stokenet_diana();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
            "#,
        );
    }

    #[test]
    fn json_deserialization_works_without_flags_as_version_1_0_0_of_app() {
        let json = serde_json::Value::from_str(
            r#"
            {
                    "securityState":
                    {
                        "unsecuredEntityControl":
                        {
                            "transactionSigning":
                            {
                                "badge":
                                {
                                    "virtualSource":
                                    {
                                        "hierarchicalDeterministicPublicKey":
                                        {
                                            "publicKey":
                                            {
                                                "curve": "secp256k1",
                                                "compressedData": "02f669a43024d90fde69351ccc53022c2f86708d9b3c42693640733c5778235da5"
                                            },
                                            "derivationPath":
                                            {
                                                "scheme": "bip44Olympia",
                                                "path": "m/44H/1022H/0H/0/0H"
                                            }
                                        },
                                        "discriminator": "hierarchicalDeterministicPublicKey"
                                    },
                                    "discriminator": "virtualSource"
                                },
                                "factorSourceID":
                                {
                                    "fromHash":
                                    {
                                        "kind": "device",
                                        "body": "8bfacfe888d4e3819c6e9528a1c8f680a4ba73e466d7af4ee204591093006589"
                                    },
                                    "discriminator": "fromHash"
                                }
                            },
                            "entityIndex": 3
                        },
                        "discriminator": "unsecured"
                    },
                    "networkID": 14,
                    "appearanceID": 3,
                    "displayName": "Olympia|Soft|0",
                    "onLedgerSettings":
                    {
                        "thirdPartyDeposits":
                        {
                            "depositRule": "acceptAll",
                            "assetsExceptionList":
                            [],
                            "depositorsAllowList":
                            []
                        }
                    },
                    "address": "account_tdx_e_169s2cfz044euhc4yjg4xe4pg55w97rq2c6jh50zsdcpuz5gk6cag6v"
                }
            "#,
        ).unwrap();
        let account = serde_json::from_value::<Account>(json).unwrap();
        assert_eq!(account.display_name.value, "Olympia|Soft|0".to_string()); // soundness
        assert_eq!(account.flags.len(), 0); // assert Default value is empty flags.
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::from_iter([
                Account::placeholder(),
                Account::placeholder_stokenet(),
                Account::placeholder_nebunet(),
                Account::placeholder_kisharnet(),
                Account::placeholder_adapanet(),
            ])
            .len(),
            5
        );
    }
}
