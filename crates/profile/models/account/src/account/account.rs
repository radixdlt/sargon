use std::ops::{Deref, DerefMut};

use profile_base_entity::BaseEntity;

use crate::prelude::*;

/// A network unique account with a unique public address and a set of cryptographic
/// factors used to control it.
///
/// Used to own and control assets on the radix network. Uniquely identified by an
/// account address, e.g.
///
/// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
///
/// But most commonly users see the address on its abbreviated form:
///
/// `acco...nvjdwr`
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
)]
#[display("{} | {}", self.display_name, self.address)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(flatten)]
    base: BaseEntity<AccountAddress>,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    #[serde(rename = "appearanceID")]
    pub appearance_id: AppearanceID,

    /// The on ledger synced settings for this account, contains e.g.
    /// ThirdPartyDeposit settings, with deposit rules for assets.
    pub on_ledger_settings: OnLedgerSettings,
}

impl Account {
    pub fn with(
        network_id: impl Into<NetworkID>,
        address: impl Into<AccountAddress>,
        display_name: impl Into<DisplayName>,
        security_state: impl Into<EntitySecurityState>,
        flags: impl IntoIterator<Item = EntityFlag>,
        appearance_id: impl Into<AppearanceID>,
        on_ledger_settings: impl Into<OnLedgerSettings>,
    ) -> Self {
        Self {
            base: BaseEntity::new(
                network_id,
                address,
                display_name,
                security_state,
                flags,
            ),
            appearance_id: appearance_id.into(),
            on_ledger_settings: on_ledger_settings.into(),
        }
    }
}

impl Deref for Account {
    type Target = BaseEntity<AccountAddress>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Account {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl From<Account> for AccountForDisplay {
    fn from(value: Account) -> Self {
        Self::new(value.address, value.display_name, value.appearance_id)
    }
}

impl HasEntityKind for Account {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}

impl HasFactorInstances for Account {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_state().unique_tx_signing_factor_instances()
    }
}

impl HasSecurityState for Account {
    fn security_state(&self) -> EntitySecurityState {
        self.security_state.clone()
    }
    fn set_security_state_unchecked(&mut self, new_state: EntitySecurityState) {
        self.security_state = new_state;
    }
}

impl IsSecurityStateAware for Account {
    fn is_securified(&self) -> bool {
        self.security_state().is_securified()
    }
}

impl IsNetworkAware for Account {
    fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }
}

impl IsBaseBaseEntity for Account {
    type Address = AccountAddress;

    fn address(&self) -> Self::Address {
        self.address
    }
    fn flags(&self) -> EntityFlags {
        self.flags.clone()
    }
}

impl IsEntityWithoutConcreteTypes for Account {
    type Path = AccountPath;

    fn with_veci_and_name(
        veci: HDFactorInstanceTransactionSigning<Self::Path>,
        name: DisplayName,
    ) -> Self {
        let address =
            AccountAddress::from_hd_factor_instance_virtual_entity_creation(
                veci.clone(),
            );

        let appearance_id = AppearanceID::from_number_of_accounts_on_network(
            u32::from(veci.path.index().index_in_local_key_space()) as usize,
        );

        Self::with(
            address.network_id(),
            address,
            name,
            UnsecuredEntityControl::with_entity_creating_factor_instance(veci),
            EntityFlags::default(),
            appearance_id,
            OnLedgerSettings::default(),
        )
    }
}

impl Account {
    pub fn new(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
        display_name: impl Into<DisplayName>,
        appearance_id: impl Into<AppearanceID>,
    ) -> Self {
        let mut self_ = Self::with_veci_and_name(
            account_creating_factor_instance,
            display_name.into(),
        );
        self_.appearance_id = appearance_id.into();
        self_
    }
}

impl Identifiable for Account {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

impl HasSampleValues for Account {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl Account {
    /// Instantiates an account with a display name, address and appearance id.
    pub fn sample_with_values(
        address: AccountAddress,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        Self {
            base: BaseEntity::new(
                address.network_id(),
                address,
                display_name,
                EntitySecurityState::sample(),
                EntityFlags::default(),
            ),
            appearance_id,
            on_ledger_settings: OnLedgerSettings::default(),
        }
    }

    fn sample_at_index_name_network(
        network_id: NetworkID,
        index: u32,
        name: &str,
        entity_flag: Option<EntityFlag>,
    ) -> Self {
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::sample();
        let derivation_index =
            HDPathComponent::unsecurified_hardened(index).unwrap();
        let account_creating_factor_instance = private_hd_factor_source
            ._derive_entity_creation_factor_instance(
                network_id,
                derivation_index,
            );

        let mut account = Self::new(
            account_creating_factor_instance,
            DisplayName::new(name).unwrap(),
            AppearanceID::try_from(index as u8).unwrap(),
        );
        if let Some(flag) = entity_flag {
            account.flags.insert(flag);
        }
        account
    }

    fn sample_at_index_name(
        index: u32,
        name: &str,
        entity_flag: Option<EntityFlag>,
    ) -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Mainnet,
            index,
            name,
            entity_flag,
        )
    }

    /// A `Mainnet` account named "Alice", a sample used to facilitate unit tests, with
    /// derivation index 0,
    pub fn sample_mainnet_alice() -> Self {
        Self::sample_at_index_name(0, "Alice", None)
    }

    /// A `Mainnet` account named "Bob", a sample used to facilitate unit tests, with
    /// derivation index 1.
    pub fn sample_mainnet_bob() -> Self {
        Self::sample_at_index_name(1, "Bob", None)
    }

    /// A `Mainnet` account named "Carol", a sample used to facilitate unit tests, with
    /// derivation index 2.
    pub fn sample_mainnet_carol() -> Self {
        Self::sample_at_index_name(2, "Carol", None)
    }

    /// A HIDDEN `Mainnet` account named "Diana", a sample used to facilitate unit tests, with
    /// derivation index 3.
    pub fn sample_mainnet_diana() -> Self {
        Self::sample_at_index_name(3, "Diana", Some(EntityFlag::HiddenByUser))
    }

    /// A tombestoned account named Erin, with derivation index 4.
    pub fn sample_mainnet_erin() -> Self {
        Self::sample_at_index_name(
            4,
            "Erin",
            Some(EntityFlag::TombstonedByUser),
        )
    }

    /// A `Mainnet` account named "Alice", a sample used to facilitate unit tests, with
    /// derivation index 0,
    pub fn sample_alice() -> Self {
        Self::sample_mainnet_alice()
    }

    /// A `Mainnet` account named "Bob", a sample used to facilitate unit tests, with
    /// derivation index 1.
    pub fn sample_bob() -> Self {
        Self::sample_mainnet_bob()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_alice()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_bob()
    }

    pub fn sample_mainnet_third() -> Self {
        Self::sample_mainnet_carol()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet_nadia() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            0,
            "Nadia",
            None,
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet_olivia() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            1,
            "Olivia",
            Some(EntityFlag::HiddenByUser),
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet_paige() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            2,
            "Paige",
            None,
        )
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_nadia()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_olivia()
    }

    pub fn sample_stokenet_third() -> Self {
        Self::sample_stokenet_paige()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_nebunet() -> Self {
        Self::sample_with_values(
            "account_tdx_b_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08m9raqq"
                .parse()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_kisharnet() -> Self {
        Self::sample_with_values(
            "account_tdx_c_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej0898vkq9"
                .parse()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_adapanet() -> Self {
        Self::sample_with_values(
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
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

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
    fn test_is_hidden() {
        assert!(!SUT::sample_mainnet_alice().is_hidden());
        assert!(SUT::sample_mainnet_diana().is_hidden());
    }

    #[test]
    fn new_with_address_only() {
        let address: AccountAddress =
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
                .parse()
                .unwrap();
        let account = SUT::sample_with_values(
            address,
            DisplayName::default(),
            AppearanceID::default(),
        );
        assert_eq!(account.address, address);
    }

    #[test]
    fn display() {
        let account = SUT::sample();
        assert_eq!(
            format!("{account}"),
            "Alice | account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87"
        );
    }

    #[test]
    fn update() {
        let mut account = SUT::sample();
        assert_eq!(account.display_name.value(), "Alice");
        account.display_name = DisplayName::new("Satoshi").unwrap();
        assert_eq!(account.display_name.value(), "Satoshi");
    }

    #[test]
    fn on_ledger_settings_get_set() {
        let mut account = SUT::sample_with_values(
            AccountAddress::sample(),
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
            [ResourceOrNonFungible::Resource {
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
    fn to_account_for_display() {
        let lhs = AccountForDisplay::from(Account::sample());
        assert_eq!(
            lhs,
            AccountForDisplay::new(
                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87".parse::<AccountAddress>().unwrap(),
                DisplayName::new("Alice").unwrap(),
                AppearanceID::new(0).unwrap(),
            )
        )
    }

    #[test]
    fn json_roundtrip_mainnet_alice() {
        let model = SUT::sample_mainnet_alice();
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
											"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
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
									"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
				"address": "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87"
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet_bob() {
        let model = SUT::sample_mainnet_bob();
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
											"compressedData": "a3a14ce3c0e549ac35f1875738c243bb6f4037f08d7d2a52ef749091a92a0c71"
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
									"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
				"address": "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_carol() {
        let model = SUT::sample_stokenet_nadia();
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
											"compressedData": "535e0b74beffc99d96acd36ae73444c0e35ebb5707f077f9bf1120b1bb8894c0"
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
									"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
				"displayName": "Nadia",
				"onLedgerSettings": {
					"thirdPartyDeposits": {
						"depositRule": "acceptAll",
						"assetsExceptionList": [],
						"depositorsAllowList": []
					}
				},
				"address": "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m"
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_olivia() {
        let model = SUT::sample_stokenet_olivia();
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
											"compressedData": "436c67c678713be6a4306bf2a64d62d29c9bccb92a776175e5cb6e95e87be55d"
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
									"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
				"displayName": "Olivia",
				"onLedgerSettings": {
					"thirdPartyDeposits": {
						"depositRule": "acceptAll",
						"assetsExceptionList": [],
						"depositorsAllowList": []
					}
				},
				"address": "account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
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
        assert_eq!(account.display_name.value(), "Olympia|Soft|0".to_string()); // soundness
        assert_eq!(account.flags.len(), 0); // assert Default value is empty flags.
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Account>::from_iter([
                SUT::sample(),
                SUT::sample_stokenet(),
                SUT::sample_nebunet(),
                SUT::sample_kisharnet(),
                SUT::sample_adapanet(),
            ])
            .len(),
            5
        );
    }
}
