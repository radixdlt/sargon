use std::cell::{Ref, RefCell};

use serde::{Deserialize, Serialize};

use super::ThirdPartyDeposits;

/// Account settings that user has set on the account component
/// On-Ledger, that is set via a transaction mutating the state
/// on the network.
///
/// This settings include third-party deposits, controlling who
/// can send which assets to this account.
///
/// These settings SHOULD be kept in sync between local state
/// (in Profile) and On-Ledger.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct OnLedgerSettings {
    /// Controls the ability of third-parties to deposit into this account
    third_party_deposits: RefCell<ThirdPartyDeposits>,
}

impl OnLedgerSettings {
    /// Instantiates a new `OnLedgerSettings` with the specified `ThirdPartyDeposits``
    pub fn new(third_party_deposits: ThirdPartyDeposits) -> Self {
        Self {
            third_party_deposits: RefCell::new(third_party_deposits),
        }
    }
}

impl OnLedgerSettings {
    /// Returns the `ThirdPartyDeposits` as a reference.
    pub fn third_party_deposits(&self) -> Ref<ThirdPartyDeposits> {
        self.third_party_deposits.borrow()
    }

    /// Replaces the `ThirdPartyDeposits` with the `new` value.
    pub fn set_third_party_deposits(&self, new: ThirdPartyDeposits) {
        *self.third_party_deposits.borrow_mut() = new;
    }

    /// Updates the `ThirdPartyDeposits` by calling the `update` closure.
    pub fn update_third_party_deposits<F>(&self, update: F)
    where
        F: Fn(&mut ThirdPartyDeposits) -> (),
    {
        update(&mut self.third_party_deposits.borrow_mut())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use wallet_kit_common::{
        assert_eq_after_json_roundtrip, assert_json_roundtrip, assert_ne_after_json_roundtrip,
    };

    use crate::v100::{
        AssetException, DepositAddressExceptionRule, DepositRule, DepositorAddress,
        ThirdPartyDeposits,
    };

    use super::OnLedgerSettings;

    #[test]
    fn get_third_party_deposits_then_mutate() {
        let settings = OnLedgerSettings::default();
        assert_eq!(
            settings.third_party_deposits().deposit_rule(),
            DepositRule::AcceptAll
        );
        settings
            .third_party_deposits()
            .set_deposit_rule(DepositRule::DenyAll);
        assert_eq!(
            settings.third_party_deposits().deposit_rule(),
            DepositRule::DenyAll
        );
    }

    #[test]
    fn set_third_party_deposits_then_mutate() {
        let settings = OnLedgerSettings::default();
        assert_eq!(
            settings.third_party_deposits().deposit_rule(),
            DepositRule::AcceptAll
        );
        settings.set_third_party_deposits(ThirdPartyDeposits::new(DepositRule::DenyAll));
        assert_eq!(
            settings.third_party_deposits().deposit_rule(),
            DepositRule::DenyAll
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = OnLedgerSettings::default();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
            	"thirdPartyDeposits" : {
            		"assetsExceptionList" : [],
            		"depositorsAllowList" : [],
            		"depositRule" : "acceptAll"
            	}
            }
            "#,
        );
        assert_json_roundtrip(&model);
        assert_ne_after_json_roundtrip(
            &model,
            r#"
            {
            	"thirdPartyDeposits" : {
            		"assetsExceptionList" : [],
            		"depositorsAllowList" : [],
            		"depositRule" : "acceptKnown"
            	}
            }
            "#,
        );
    }

    #[test]
    fn json_decode_deny_all_with_exceptions() {
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
        let model = ThirdPartyDeposits::with_rule_and_lists(
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

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
            	"depositRule" : "denyAll",
            	"assetsExceptionList" : [
        			{
						"address" : "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq",
						"exceptionRule" : "allow"
					},
					{
						"address" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
						"exceptionRule" : "allow"
					}
				],
				"depositorsAllowList" : [
					{
						"value" : "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq",
						"discriminator" : "resourceAddress"
					}
				]
			}
            "#,
        )
    }
}
