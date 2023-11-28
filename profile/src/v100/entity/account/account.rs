use hierarchical_deterministic::derivation::derivation::Derivation;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, cmp::Ordering, fmt::Display};
use wallet_kit_common::network_id::NetworkID;

use crate::v100::{
    address::account_address::AccountAddress,
    entity::{display_name::DisplayName, entity_flags::EntityFlags},
    entity_security_state::{
        entity_security_state::EntitySecurityState,
        unsecured_entity_control::UnsecuredEntityControl,
    },
    factors::hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
};

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
    display_name: RefCell<DisplayName>,

    /// Security state of this account, either "securified" or not.
    security_state: EntitySecurityState,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    appearance_id: RefCell<AppearanceID>,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    flags: RefCell<EntityFlags>,

    /// The on ledger synced settings for this account
    on_ledger_settings: RefCell<OnLedgerSettings>,
}

impl Account {
    /// Instantiates an account with a display name, address and appearance id.
    pub fn with_values(
        address: AccountAddress,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        Self {
            network_id: address.network_id,
            address,
            display_name: RefCell::new(display_name),
            appearance_id: RefCell::new(appearance_id),
            flags: RefCell::new(EntityFlags::default()),
            on_ledger_settings: RefCell::new(OnLedgerSettings::default()),
            security_state: EntitySecurityState::Unsecured(UnsecuredEntityControl::new(
                HierarchicalDeterministicFactorInstance::placeholder(),
            )),
        }
    }
}

// Getters
impl Account {
    /// Returns this accounts `display_name` as **a clone**.
    ///
    /// Use [`self::set_display_name()`] to update it.
    pub fn get_display_name(&self) -> String {
        self.display_name.borrow().clone().to_string()
    }

    pub fn get_flags(&self) -> EntityFlags {
        self.flags.borrow().clone()
    }

    pub fn get_appearance_id(&self) -> AppearanceID {
        self.appearance_id.borrow().clone()
    }

    pub fn get_on_ledger_settings(&self) -> OnLedgerSettings {
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
                .transaction_signing
                .derivation_path
                .last_component()
                .cmp(r.transaction_signing.derivation_path.last_component()),
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
        write!(f, "{} | {}", self.get_display_name(), self.address)
    }
}

// CFG test
#[cfg(test)]
impl Account {
    pub fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    pub fn placeholder_mainnet() -> Self {
        Self::with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    pub fn placeholder_stokenet() -> Self {
        Self::with_values(
            "account_tdx_2_12ygsf87pma439ezvdyervjfq2nhqme6reau6kcxf6jtaysaxl7sqvd"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    pub fn placeholder_nebunet() -> Self {
        Self::with_values(
            "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    pub fn placeholder_kisharnet() -> Self {
        Self::with_values(
            "account_tdx_c_1px26p5tyqq65809em2h4yjczxcxj776kaun6sv3dw66sc3wrm6"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }

    pub fn placeholder_adapanet() -> Self {
        Self::with_values(
            "account_tdx_a_1qwv0unmwmxschqj8sntg6n9eejkrr6yr6fa4ekxazdzqhm6wy5"
                .try_into()
                .unwrap(),
            DisplayName::default(),
            AppearanceID::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

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
        let account = Account::with_values(
            address.clone(),
            DisplayName::default(),
            AppearanceID::default(),
        );
        assert_eq!(account.address, address);
    }

    #[test]
    fn appearance_id_get_set() {
        let account = Account::placeholder();
        assert_eq!(account.get_appearance_id(), AppearanceID::default());
        let new_appearance_id = AppearanceID::new(1).unwrap();
        account.set_appearance_id(new_appearance_id);
        assert_eq!(account.get_appearance_id(), new_appearance_id);
    }

    #[test]
    fn display() {
        let account = Account::placeholder();
        assert_eq!(
            format!("{account}"),
            "Unnamed | account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
        );
    }

    #[test]
    fn display_name_get_set() {
        let account = Account::with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.get_display_name(), "Test");
        let new_display_name = DisplayName::new("New").unwrap();
        account.set_display_name(new_display_name.clone());
        assert_eq!(account.get_display_name(), new_display_name.to_string());
    }

    #[test]
    fn flags_get_set() {
        let account = Account::with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(account.get_flags(), EntityFlags::default());
        let new_flags = EntityFlags::with_flag(EntityFlag::DeletedByUser);
        account.set_flags(new_flags.clone());
        assert_eq!(account.get_flags(), new_flags);
    }

    #[test]
    fn on_ledger_settings_get_set() {
        let account = Account::with_values(
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap(),
            DisplayName::new("Test").unwrap(),
            AppearanceID::default(),
        );
        assert_eq!(
            account.get_on_ledger_settings(),
            OnLedgerSettings::default()
        );
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
        assert_eq!(account.get_on_ledger_settings(), new_on_ledger_settings);

        assert_eq!(
            account
                .get_on_ledger_settings()
                .get_third_party_deposits()
                .get_deposit_rule(),
            DepositRule::DenyAll
        );
        account.update_on_ledger_settings(|o| {
            o.update_third_party_deposits(|t| t.set_deposit_rule(DepositRule::AcceptAll))
        });
        assert_eq!(
            account
                .get_on_ledger_settings()
                .get_third_party_deposits()
                .get_deposit_rule(),
            DepositRule::AcceptAll
        );
    }

    // #[test]
    // fn compare() {
    //     let make = |index: HDPathValue| -> Account {
    //         let address: AccountAddress =
    //             "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
    //                 .try_into()
    //                 .unwrap();

    //         let account = Account {
    //             address: address.clone(),
    //             network_id: address.network_id,
    //             display_name: RefCell::new(DisplayName::new("Test").unwrap()),
    //             appearance_id: RefCell::new(AppearanceID::default()),
    //             flags: RefCell::new(EntityFlags::default()),
    //             on_ledger_settings: RefCell::new(OnLedgerSettings::default()),
    //             security_state: EntitySecurityState::Unsecured(UnsecuredEntityControl::new(
    //                 index,
    //                 HierarchicalDeterministicFactorInstance::placeholder(),
    //             )),
    //         };
    //         account
    //     };
    //     let a = make(0);
    //     let b = make(1);
    //     assert!(a < b);
    // }

    // #[test]
    fn json_roundtrip() {
        let model = Account::with_values(
            "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw"
                .try_into()
                .unwrap(),
            "Zaba 0".try_into().unwrap(),
            0.try_into().unwrap(),
        );
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
											"compressedData": "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
										},
										"derivationPath": {
											"scheme": "cap26",
											"path": "m/44H/1022H/14H/525H/1460H/0H"
										}
									},
									"discriminator": "hierarchicalDeterministicPublicKey"
								},
								"discriminator": "virtualSource"
							},
							"factorSourceID": {
								"fromHash": {
									"kind": "device",
									"body": "c9e67a9028fb3150304c77992710c35c8e479d4fa59f7c45a96ce17f6fdf1d2c"
								},
								"discriminator": "fromHash"
							}
						}
					},
					"discriminator": "unsecured"
				},
				"networkID": 14,
				"appearanceID": 0,
				"flags": [],
				"displayName": "Zaba 0",
				"onLedgerSettings": {
					"thirdPartyDeposits": {
						"depositRule": "acceptAll",
						"assetsExceptionList": [],
						"depositorsAllowList": []
					}
				},
				"flags": ["deletedByUser"],
				"address": "account_tdx_e_128vkt2fur65p4hqhulfv3h0cknrppwtjsstlttkfamj4jnnpm82gsw"
			}
            "#,
        );
    }
}
