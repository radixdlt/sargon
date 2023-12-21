#[cfg(any(test, feature = "placeholder"))]
use crate::v100::factors::factor_sources::{
    device_factor_source::device_factor_source::DeviceFactorSource,
    private_hierarchical_deterministic_factor_source::PrivateHierarchicalDeterministicFactorSource,
};
#[cfg(any(test, feature = "placeholder"))]
use hierarchical_deterministic::{
    bip32::hd_path_component::HDPathValue,
    derivation::mnemonic_with_passphrase::MnemonicWithPassphrase,
};

use hierarchical_deterministic::{
    cap26::cap26_path::paths::is_entity_path::HasEntityPath, derivation::derivation::Derivation,
};
use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, cmp::Ordering, fmt::Display, hash::Hasher};
use wallet_kit_common::network_id::NetworkID;

use crate::v100::{
    address::{account_address::AccountAddress, entity_address::EntityAddress},
    entity::{display_name::DisplayName, entity_flags::EntityFlags},
    entity_security_state::{
        entity_security_state::EntitySecurityState,
        unsecured_entity_control::UnsecuredEntityControl,
    },
    factors::hd_transaction_signing_factor_instance::HDFactorInstanceAccountCreation,
};

use std::hash::Hash;

use super::{
    appearance_id::AppearanceID, on_ledger_settings::on_ledger_settings::OnLedgerSettings,
};

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// The ID of the network this account can be used with.
    #[serde(rename = "networkID")]
    network_id: NetworkID,

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
    address: AccountAddress,

    /// An off-ledger display name or description chosen by the user when she
    /// created this account.
    display_name: RefCell<DisplayName>,

    /// Security state of this account, either "securified" or not.
    security_state: EntitySecurityState,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    #[serde(rename = "appearanceID")]
    appearance_id: RefCell<AppearanceID>,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    #[serde(default)]
    flags: RefCell<EntityFlags>,

    /// The on ledger synced settings for this account, contains e.g.
    /// ThirdPartyDeposit settings, with deposit rules for assets.
    on_ledger_settings: RefCell<OnLedgerSettings>,
}

impl Account {
    pub fn new(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        let address = AccountAddress::from_hd_factor_instance_virtual_entity_creation(
            account_creating_factor_instance.clone(),
        );
        Self {
            network_id: account_creating_factor_instance.network_id(),
            address,
            display_name: RefCell::new(display_name),
            security_state: UnsecuredEntityControl::with_account_creating_factor_instance(
                account_creating_factor_instance,
            )
            .into(),
            appearance_id: RefCell::new(appearance_id),
            flags: RefCell::new(EntityFlags::default()),
            on_ledger_settings: RefCell::new(OnLedgerSettings::default()),
        }
    }
}

impl Identifiable for Account {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address().hash(state);
    }
}

// Getters
impl Account {
    pub fn network_id(&self) -> NetworkID {
        self.network_id.clone()
    }

    pub fn address(&self) -> AccountAddress {
        self.address.clone()
    }

    /// Returns this accounts `display_name` as **a clone**.
    ///
    /// Use [`self::set_display_name()`] to update it.
    pub fn display_name(&self) -> String {
        self.display_name.borrow().clone().to_string()
    }

    pub fn flags(&self) -> EntityFlags {
        self.flags.borrow().clone()
    }

    pub fn appearance_id(&self) -> AppearanceID {
        self.appearance_id.borrow().clone()
    }

    pub fn on_ledger_settings(&self) -> OnLedgerSettings {
        self.on_ledger_settings.borrow().clone()
    }
}

// Setters
impl Account {
    pub fn set_display_name(&self, new: DisplayName) {
        *self.display_name.borrow_mut() = new;
    }

    pub fn set_flags(&self, new: EntityFlags) {
        *self.flags.borrow_mut() = new;
    }

    pub fn set_appearance_id(&self, new: AppearanceID) {
        *self.appearance_id.borrow_mut() = new;
    }

    pub fn set_on_ledger_settings(&self, new: OnLedgerSettings) {
        *self.on_ledger_settings.borrow_mut() = new;
    }

    pub fn update_on_ledger_settings<F>(&self, update: F)
    where
        F: Fn(&mut OnLedgerSettings) -> (),
    {
        update(&mut self.on_ledger_settings.borrow_mut())
    }
}

impl Ord for Account {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.security_state, &other.security_state) {
            (EntitySecurityState::Unsecured(l), EntitySecurityState::Unsecured(r)) => l
                .transaction_signing()
                .derivation_path()
                .last_component()
                .cmp(r.transaction_signing().derivation_path().last_component()),
        }
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.display_name(), self.address)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Account {
    /// Instantiates an account with a display name, address and appearance id.
    pub fn placeholder_with_values(
        address: AccountAddress,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        Self {
            network_id: address.network_id().clone(),
            address,
            display_name: RefCell::new(display_name),
            appearance_id: RefCell::new(appearance_id),
            flags: RefCell::new(EntityFlags::default()),
            on_ledger_settings: RefCell::new(OnLedgerSettings::default()),
            security_state: EntitySecurityState::placeholder(),
        }
    }

    fn placeholder_at_index_name(index: HDPathValue, name: &str) -> Self {
        Self::placeholder_at_index_name_network(NetworkID::Mainnet, index, name)
    }

    fn placeholder_at_index_name_network(
        network_id: NetworkID,
        index: HDPathValue,
        name: &str,
    ) -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let bdfs = DeviceFactorSource::babylon(true, mwp.clone(), "iPhone");
        let private_hd_factor_source = PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);
        let account_creating_factor_instance =
            private_hd_factor_source.derive_account_creation_factor_instance(network_id, index);

        Self::new(
            account_creating_factor_instance,
            DisplayName::new(name).unwrap(),
            AppearanceID::try_from(index as u8).unwrap(),
        )
    }

    /// A `Mainnet` account named "Alice", a placeholder used to facilitate unit tests, with
    /// derivation index 0,
    pub fn placeholder_mainnet_alice() -> Self {
        Self::placeholder_at_index_name(0, "Alice")
    }

    /// A `Mainnet` account named "Bob", a placeholder used to facilitate unit tests, with
    /// derivation index 1.
    pub fn placeholder_mainnet_bob() -> Self {
        Self::placeholder_at_index_name(1, "Bob")
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
    pub fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::placeholder_mainnet_alice()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_carol() -> Self {
        Self::placeholder_at_index_name_network(NetworkID::Stokenet, 0, "Carol")
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_diana() -> Self {
        Self::placeholder_at_index_name_network(NetworkID::Stokenet, 1, "Diana")
    }

    pub fn placeholder_stokenet() -> Self {
        Self::placeholder_stokenet_carol()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_nebunet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_b_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08m9raqq"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_kisharnet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_c_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej0898vkq9"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_adapanet() -> Self {
        Self::placeholder_with_values(
            "account_tdx_a_1286wrrqrfcrfhthfrtdywe8alney8zu0ja5xrhcq2475ej08srjqq0"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, str::FromStr};

    use radix_engine_common::prelude::HashSet;
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::{
        address::account_address::AccountAddress,
        entity::{
            account::{
                appearance_id::AppearanceID,
                on_ledger_settings::{
                    on_ledger_settings::OnLedgerSettings,
                    third_party_deposits::{
                        asset_exception::AssetException,
                        deposit_address_exception_rule::DepositAddressExceptionRule,
                        deposit_rule::DepositRule, depositor_address::DepositorAddress,
                        third_party_deposits::ThirdPartyDeposits,
                    },
                },
            },
            display_name::DisplayName,
            entity_flag::EntityFlag,
            entity_flags::EntityFlags,
        },
    };

    use super::Account;

    #[test]
    fn new_with_address_only() {
        let address: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();
        let account = Account::placeholder_with_values(
            address.clone(),
            DisplayName::default(),
            AppearanceID::default(),
        );
        assert_eq!(account.address, address);
    }

    #[test]
    fn appearance_id_get_set() {
        let account = Account::placeholder();
        assert_eq!(account.appearance_id(), AppearanceID::default());
        let new_appearance_id = AppearanceID::new(1).unwrap();
        account.set_appearance_id(new_appearance_id);
        assert_eq!(account.appearance_id(), new_appearance_id);
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
    fn display_name_get_set() {
        let account = Account::placeholder_with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.display_name(), "Test");
        let new_display_name = DisplayName::new("New").unwrap();
        account.set_display_name(new_display_name.clone());
        assert_eq!(account.display_name(), new_display_name.to_string());
    }

    #[test]
    fn flags_get_set() {
        let account = Account::placeholder_with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.flags(), EntityFlags::default());
        let new_flags = EntityFlags::with_flag(EntityFlag::DeletedByUser);
        account.set_flags(new_flags.clone());
        assert_eq!(account.flags(), new_flags);
    }

    #[test]
    fn on_ledger_settings_get_set() {
        let account = Account::placeholder_with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.on_ledger_settings(), OnLedgerSettings::default());
        let excp1 = AssetException::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let excp2 = AssetException::new(
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let new_third_party_dep = ThirdPartyDeposits::with_rule_and_lists(
            DepositRule::DenyAll,
            BTreeSet::from_iter([excp1, excp2].into_iter()),
            BTreeSet::from_iter(
                [DepositorAddress::ResourceAddress(
                    "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                        .try_into()
                        .unwrap(),
                )]
                .into_iter(),
            ),
        );
        let new_on_ledger_settings = OnLedgerSettings::new(new_third_party_dep);
        account.set_on_ledger_settings(new_on_ledger_settings.clone());
        assert_eq!(account.on_ledger_settings(), new_on_ledger_settings);

        assert_eq!(
            account
                .on_ledger_settings()
                .third_party_deposits()
                .deposit_rule(),
            DepositRule::DenyAll
        );
        account.update_on_ledger_settings(|o| {
            o.update_third_party_deposits(|t| t.set_deposit_rule(DepositRule::AcceptAll))
        });
        assert_eq!(
            account
                .on_ledger_settings()
                .third_party_deposits()
                .deposit_rule(),
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
				"flags": [],
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
				"flags": [],
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
        assert_eq!(account.display_name(), "Olympia|Soft|0"); // soundness
        assert_eq!(account.flags().len(), 0); // assert Default value is empty flags.
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
