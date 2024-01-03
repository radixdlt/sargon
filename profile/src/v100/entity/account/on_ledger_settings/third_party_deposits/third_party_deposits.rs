use identified_vec::{Identifiable, IsIdentifiedVec};
use serde::{Deserialize, Serialize};

use crate::IdentifiedVecVia;

use super::{
    asset_exception::AssetException, deposit_rule::DepositRule, depositor_address::DepositorAddress,
};

/// Controls the ability of third-parties to deposit into a certain account, this is
/// useful for users who wish to not be able to receive airdrops.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    pub deposit_rule: DepositRule,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    pub assets_exception_list: IdentifiedVecVia<AssetException>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    pub depositors_allow_list: IdentifiedVecVia<DepositorAddress>,
}

impl Default for ThirdPartyDeposits {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Identifiable for DepositorAddress {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl ThirdPartyDeposits {
    /// Instantiates a new `ThirdPartyDeposits` with the specified
    /// `DepositRule` and empty `assets_exception` and
    /// `depositors_allow` lists.
    pub fn new(deposit_rule: DepositRule) -> Self {
        Self {
            deposit_rule,
            assets_exception_list: IdentifiedVecVia::new(),
            depositors_allow_list: IdentifiedVecVia::new(),
        }
    }

    /// Instantiates a new `ThirdPartyDeposits` with the provided
    /// rule and lists.
    pub fn with_rule_and_lists<I, J>(
        deposit_rule: DepositRule,
        assets_exception_list: I,
        depositors_allow_list: J,
    ) -> Self
    where
        I: IntoIterator<Item = AssetException>,
        J: IntoIterator<Item = DepositorAddress>,
    {
        Self {
            deposit_rule,
            assets_exception_list: IdentifiedVecVia::from_iter(assets_exception_list),
            depositors_allow_list: IdentifiedVecVia::from_iter(depositors_allow_list),
        }
    }

    /// Adds an `AssetException` to the `assets_exception_list` (set).
    ///
    /// Returns whether the `exception`` was newly inserted. That is:
    ///
    /// If the set did not previously contain an equal value, true is returned.
    /// If the set already contained an equal value, false is returned, and the entry is not updated.
    pub fn add_asset_exception(&mut self, exception: AssetException) -> bool {
        self.assets_exception_list.append(exception).0
    }

    // If the set contains an element equal to `exception`, removes it from the set and drops it. Returns whether such an element was present.
    pub fn remove_asset_exception(&mut self, exception: &AssetException) -> bool {
        self.assets_exception_list.remove(exception).is_some()
    }

    /// Adds a `DepositorAddress` to the `depositors_allow_list` (set).
    ///
    /// Returns whether the `depositor`` was newly inserted. That is:
    ///
    /// If the set did not previously contain an equal value, true is returned.
    /// If the set already contained an equal value, false is returned, and the entry is not updated.
    pub fn allow_depositor(&mut self, depositor: DepositorAddress) -> bool {
        self.depositors_allow_list.append(depositor).0
    }

    // If the set contains an element equal to `DepositorAddress`, removes it from the set and drops it. Returns whether such an element was present.
    pub fn remove_allowed_depositor(&mut self, depositor: &DepositorAddress) -> bool {
        self.depositors_allow_list.remove(depositor).is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{assert_eq_after_json_roundtrip, IdentifiedVecVia};

    use crate::v100::{
        entity::account::on_ledger_settings::third_party_deposits::{
            asset_exception::AssetException,
            deposit_address_exception_rule::DepositAddressExceptionRule, deposit_rule::DepositRule,
            depositor_address::DepositorAddress,
        },
        NonFungibleGlobalId,
    };

    use super::ThirdPartyDeposits;

    #[test]
    fn json_roundtrip() {
        let excp1 = AssetException::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Deny,
        );
        let excp2 = AssetException::new(
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        let model = ThirdPartyDeposits::with_rule_and_lists(
            DepositRule::AcceptKnown,
            BTreeSet::from_iter([excp1, excp2].into_iter()),
            BTreeSet::from_iter(
                [DepositorAddress::NonFungibleGlobalID { value: NonFungibleGlobalId::try_from_str("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>").unwrap()}]
                .into_iter(),
            ),
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
            	"depositRule" : "acceptKnown",
            	"assetsExceptionList" : [
            		{
			            "address" : "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq",
			            "exceptionRule" : "deny"
            		},
            		{
            			"address" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
			            "exceptionRule" : "allow"
            		}
            	],
                "depositorsAllowList" : [
            		{
            			"value" : "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>",
            			"discriminator" : "nonFungibleGlobalID"
            		}
               	]
            }
            "#,
        );
    }

    #[test]
    fn change_asset_exception_list() {
        let mut settings: ThirdPartyDeposits = serde_json::from_str(
            r#"
            {
            	"depositRule" : "acceptKnown",
            	"assetsExceptionList" : [
            		{
            			"address" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
			            "exceptionRule" : "allow"
            		}
            	],
                "depositorsAllowList" : [
            		{
            			"value" : "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>",
            			"discriminator" : "nonFungibleGlobalID"
            		}
               	]
            }
            "#
            ).unwrap();

        let exception = AssetException::new(
            "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                .try_into()
                .unwrap(),
            DepositAddressExceptionRule::Deny,
        );
        assert!(settings.add_asset_exception(exception.clone()));
        assert_eq!(settings.assets_exception_list.len(), 2);
        assert!(settings.remove_asset_exception(&exception));
        assert_eq!(settings.assets_exception_list.len(), 1);
        // settings.set_assets_exception_list(BTreeSet::from_iter([exception.clone()]));
        settings.assets_exception_list = IdentifiedVecVia::from_iter([exception.clone()]);

        assert!(
            !settings.add_asset_exception(exception.clone()),
            "Expected `false` since already present."
        );
    }

    #[test]
    fn change_allowed_depositor() {
        let mut settings: ThirdPartyDeposits = serde_json::from_str(
            r#"
            {
            	"depositRule" : "acceptKnown",
            	"assetsExceptionList" : [
            		{
            			"address" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
			            "exceptionRule" : "allow"
            		}
            	],
                "depositorsAllowList" : []
            }
            "#,
        )
        .unwrap();

        let depositor =
            DepositorAddress::NonFungibleGlobalID {
                value: NonFungibleGlobalId::try_from_str(
                    "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>",
                )
                .unwrap(),
            };
        assert!(settings.allow_depositor(depositor.clone()));
        assert_eq!(settings.depositors_allow_list.len(), 1);
        assert!(settings.remove_allowed_depositor(&depositor));
        assert_eq!(settings.depositors_allow_list.len(), 0);
        assert_eq!(
            settings.depositors_allow_list,
            IdentifiedVecVia::from_iter([depositor.clone()]),
        );
        assert!(
            !settings.allow_depositor(depositor.clone()),
            "Expected `false` since already present."
        );
    }

    #[test]
    fn accept_all_is_default() {
        assert_eq!(
            ThirdPartyDeposits::default().deposit_rule,
            DepositRule::AcceptAll
        );
    }

    #[test]
    fn empty_assets_exception_list_is_default() {
        assert!(ThirdPartyDeposits::default()
            .assets_exception_list
            .is_empty(),);
    }

    #[test]
    fn empty_depositors_allow_list_is_default() {
        assert!(ThirdPartyDeposits::default()
            .depositors_allow_list
            .is_empty(),);
    }

    #[test]
    fn change_rule() {
        let mut settings = ThirdPartyDeposits::new(DepositRule::AcceptAll);
        assert_eq!(settings.deposit_rule, DepositRule::AcceptAll);
        settings.deposit_rule = DepositRule::DenyAll;
        assert_eq!(settings.deposit_rule, DepositRule::DenyAll);
    }
}
