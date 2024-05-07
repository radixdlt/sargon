use crate::prelude::*;

/// Controls the ability of third-parties to deposit into a certain account, this is
/// useful for users who wish to not be able to receive airdrops.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    pub deposit_rule: DepositRule,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub assets_exception_list: Option<AssetsExceptionList>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub depositors_allow_list: Option<DepositorsAllowList>,
}

impl HasSampleValues for ThirdPartyDeposits {
    fn sample() -> Self {
        Self::with_rule_and_lists(
            DepositRule::AcceptKnown,
            [AssetException::sample(), AssetException::sample_other()],
            [
                ResourceOrNonFungible::sample(),
                ResourceOrNonFungible::sample_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::with_rule_and_lists(
            DepositRule::DenyAll,
            [AssetException::sample_other()],
            [ResourceOrNonFungible::sample_other()],
        )
    }
}

impl Default for ThirdPartyDeposits {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl ThirdPartyDeposits {
    /// Instantiates a new `ThirdPartyDeposits` with the specified
    /// `DepositRule` and empty `assets_exception` and
    /// `depositors_allow` lists.
    pub fn new(deposit_rule: DepositRule) -> Self {
        Self {
            deposit_rule,
            assets_exception_list: Some(AssetsExceptionList::new()),
            depositors_allow_list: Some(DepositorsAllowList::new()),
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
        J: IntoIterator<Item = ResourceOrNonFungible>,
    {
        Self {
            deposit_rule,
            assets_exception_list: Some(AssetsExceptionList::from_iter(
                assets_exception_list,
            )),
            depositors_allow_list: Some(DepositorsAllowList::from_iter(
                depositors_allow_list,
            )),
        }
    }

    /// Adds an `AssetException` to the `assets_exception_list` (set).
    ///
    /// Returns whether the `exception`` was newly inserted. That is:
    ///
    /// If the set did not previously contain an equal value, true is returned.
    /// If the set already contained an equal value, false is returned, and the entry is not updated.
    pub fn add_asset_exception(&mut self, exception: AssetException) -> bool {
        if let Some(mut list) = self.assets_exception_list.clone() {
            let added = list.append(exception).0;
            self.assets_exception_list = Some(list);
            added
        } else {
            self.assets_exception_list =
                Some(AssetsExceptionList::just(exception));
            true
        }
    }

    // If the set contains an element equal to `exception`, removes it from the set and drops it. Returns whether such an element was present.
    pub fn remove_asset_exception(
        &mut self,
        exception: &AssetException,
    ) -> bool {
        if let Some(mut list) = self.assets_exception_list.clone() {
            let was_present = list.remove(exception).is_some();
            self.assets_exception_list = Some(list);
            was_present
        } else {
            false
        }
    }

    /// Adds a `DepositorAddress` to the `depositors_allow_list` (set).
    ///
    /// Returns whether the `depositor`` was newly inserted. That is:
    ///
    /// If the set did not previously contain an equal value, true is returned.
    /// If the set already contained an equal value, false is returned, and the entry is not updated.
    pub fn allow_depositor(
        &mut self,
        depositor: ResourceOrNonFungible,
    ) -> bool {
        if let Some(mut list) = self.depositors_allow_list.clone() {
            let added = list.append(depositor).0;
            self.depositors_allow_list = Some(list);
            added
        } else {
            self.depositors_allow_list =
                Some(DepositorsAllowList::just(depositor));
            true
        }
    }

    // If the set contains an element equal to `DepositorAddress`, removes it from the set and drops it. Returns whether such an element was present.
    pub fn remove_allowed_depositor(
        &mut self,
        depositor: &ResourceOrNonFungible,
    ) -> bool {
        if let Some(mut list) = self.depositors_allow_list.clone() {
            let was_present = list.remove(depositor).is_some();
            self.depositors_allow_list = Some(list);
            was_present
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ThirdPartyDeposits;

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
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "depositRule": "acceptKnown",
                "assetsExceptionList": [
                  {
                    "address": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
                    "exceptionRule": "allow"
                  },
                  {
                    "address": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j",
                    "exceptionRule": "deny"
                  }
                ],
                "depositorsAllowList": [
                  {
                    "discriminator": "resourceAddress",
                    "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                  },
                  {
                    "discriminator": "nonFungibleGlobalID",
                    "value": "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>"
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
                .parse()
                .unwrap(),
            DepositAddressExceptionRule::Deny,
        );
        assert!(settings.add_asset_exception(exception));
        assert_eq!(settings.assets_exception_list.clone().unwrap().len(), 2);
        assert!(settings.remove_asset_exception(&exception));
        assert_eq!(settings.assets_exception_list.clone().unwrap().len(), 1);
        settings.assets_exception_list =
            Some(AssetsExceptionList::from_iter([exception]));

        assert!(
            !settings.add_asset_exception(exception),
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

        let depositor = ResourceOrNonFungible::NonFungible {
            value: "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>"
                .parse()
                .unwrap(),
        };
        assert!(settings.allow_depositor(depositor.clone()));
        assert_eq!(settings.depositors_allow_list.clone().unwrap().len(), 1);
        assert!(settings.remove_allowed_depositor(&depositor));
        assert_eq!(settings.depositors_allow_list.clone().unwrap().len(), 0);

        settings.depositors_allow_list =
            Some(DepositorsAllowList::from_iter([depositor.clone()]));
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
        assert_eq!(
            ThirdPartyDeposits::default().assets_exception_list,
            Some(AssetsExceptionList::new())
        );
    }

    #[test]
    fn empty_depositors_allow_list_is_default() {
        assert_eq!(
            ThirdPartyDeposits::default().depositors_allow_list,
            Some(DepositorsAllowList::new())
        );
    }

    #[test]
    fn change_rule() {
        let mut settings = ThirdPartyDeposits::new(DepositRule::AcceptAll);
        assert_eq!(settings.deposit_rule, DepositRule::AcceptAll);
        settings.deposit_rule = DepositRule::DenyAll;
        assert_eq!(settings.deposit_rule, DepositRule::DenyAll);
    }

    #[test]
    fn test_add_to_asset_exception_list_when_nil() {
        let mut sut = SUT {
            deposit_rule: DepositRule::AcceptAll,
            assets_exception_list: None,
            depositors_allow_list: None,
        };
        assert!(sut.add_asset_exception(AssetException::sample()));
        assert!(sut
            .assets_exception_list
            .unwrap()
            .contains(&AssetException::sample()));
    }

    #[test]
    fn test_add_to_depositors_list_when_nil() {
        let mut sut = SUT {
            deposit_rule: DepositRule::AcceptAll,
            assets_exception_list: None,
            depositors_allow_list: None,
        };
        assert!(sut.allow_depositor(ResourceOrNonFungible::sample()));
        assert!(sut
            .depositors_allow_list
            .unwrap()
            .contains(&ResourceOrNonFungible::sample()));
    }

    #[test]
    fn test_remove_non_existing_asset_exception() {
        let mut sut = SUT::default();
        assert!(!sut.remove_asset_exception(&AssetException::sample()))
    }

    #[test]
    fn test_remove_non_existing_depositor() {
        let mut sut = SUT::default();
        assert!(!sut.remove_allowed_depositor(&ResourceOrNonFungible::sample()));
    }
}
