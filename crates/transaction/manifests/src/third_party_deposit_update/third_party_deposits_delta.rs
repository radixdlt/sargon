use crate::prelude::*;

use radix_engine_interface::blueprints::account::{
    AccountAddAuthorizedDepositorInput as ScryptoAccountAddAuthorizedDepositorInput,
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ThirdPartyDepositsDelta {
    pub(crate) deposit_rule: Option<ScryptoDefaultDepositRule>,
    pub(crate) asset_exceptions_to_be_removed: Vec<ScryptoManifestValue>,
    pub(crate) asset_exceptions_to_add_or_update:
        Vec<ScryptoAccountSetResourcePreferenceInput>,
    pub(crate) depositor_addresses_to_remove:
        Vec<ScryptoAccountRemoveAuthorizedDepositorInput>,
    pub(crate) depositor_addresses_to_add:
        Vec<ScryptoAccountAddAuthorizedDepositorInput>,
}

impl ThirdPartyDepositsDelta {
    pub fn new(from: ThirdPartyDeposits, to: ThirdPartyDeposits) -> Self {
        let deposit_rule: Option<DepositRule> =
            if to.deposit_rule != from.deposit_rule {
                Some(to.deposit_rule)
            } else {
                None
            };

        Self {
            deposit_rule: deposit_rule.map(ScryptoDefaultDepositRule::from),
            asset_exceptions_to_be_removed: from
                .assets_exception_list
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    !to.assets_exception_list
                        .clone()
                        .unwrap_or_default()
                        .contains_by_id(x)
                })
                .map(ScryptoManifestValue::from)
                .collect(),
            asset_exceptions_to_add_or_update: to
                .assets_exception_list
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    !from
                        .assets_exception_list
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .any(|w| {
                            if w.id() == x.id() {
                                // Rule update for the same asset
                                w.exception_rule == x.exception_rule
                            } else {
                                // New asset
                                false
                            }
                        })
                })
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect(),
            depositor_addresses_to_remove: from
                .depositors_allow_list
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    !to.depositors_allow_list
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .contains(x)
                })
                .map(ScryptoAccountRemoveAuthorizedDepositorInput::from)
                .collect(),
            depositor_addresses_to_add: to
                .depositors_allow_list
                .clone()
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    !from
                        .depositors_allow_list
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .contains(x)
                })
                .map(ScryptoAccountAddAuthorizedDepositorInput::from)
                .collect(),
        }
    }
}

impl HasSampleValues for ThirdPartyDepositsDelta {
    fn sample() -> Self {
        Self::new(
            ThirdPartyDeposits::sample(),
            ThirdPartyDeposits::sample_other(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ThirdPartyDeposits::sample_other(),
            ThirdPartyDeposits::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use radix_engine_interface::blueprints::account::AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ThirdPartyDepositsDelta;

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
    fn scrypto_account_remove_resource_preference_input_from_resource_or_nf() {
        assert_eq!(
            ScryptoAccountRemoveResourcePreferenceInput::from(
                ResourceOrNonFungible::Resource {
                    value: ResourceAddress::sample()
                }
            )
            .resource_address
            .into_node_id(),
            ResourceAddress::sample().node_id()
        );

        assert_eq!(
            ScryptoAccountRemoveResourcePreferenceInput::from(
                ResourceOrNonFungible::NonFungible {
                    value: NonFungibleGlobalId::sample()
                }
            )
            .resource_address
            .into_node_id(),
            NonFungibleResourceAddress::sample().node_id()
        );
    }

    #[test]
    fn deposit_rule_into_scrypto() {
        assert_eq!(
            ScryptoDefaultDepositRule::from(DepositRule::AcceptAll),
            ScryptoDefaultDepositRule::Accept
        );
        assert_eq!(
            ScryptoDefaultDepositRule::from(DepositRule::AcceptKnown),
            ScryptoDefaultDepositRule::AllowExisting
        );
        assert_eq!(
            ScryptoDefaultDepositRule::from(DepositRule::DenyAll),
            ScryptoDefaultDepositRule::Reject
        );
    }

    #[test]
    fn default_is_empty() {
        let sut = SUT::default();
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_between_same_is_empty() {
        assert_eq!(
            SUT::new(
                ThirdPartyDeposits::sample(),
                ThirdPartyDeposits::sample()
            ),
            SUT::default()
        );
        assert_eq!(
            SUT::new(
                ThirdPartyDeposits::sample_other(),
                ThirdPartyDeposits::sample_other()
            ),
            SUT::default()
        );
    }

    #[test]
    fn delta_deposit_rule() {
        let new_deposit_rule = DepositRule::DenyAll;
        let sut = SUT::new(
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
            ThirdPartyDeposits::new(new_deposit_rule),
        );
        assert_eq!(sut.deposit_rule, Some(new_deposit_rule.into()));
        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_asset_exceptions_to_add_or_update() {
        let asset_exceptions = [AssetException::sample()];
        let sut = SUT::new(
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions,
                [],
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(
            sut.asset_exceptions_to_add_or_update,
            asset_exceptions
                .into_iter()
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect_vec()
        );
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_asset_exceptions_to_add_for_same_rule() {
        let same_rule = DepositAddressExceptionRule::Allow;
        let existing_exception =
            AssetException::new(ResourceAddress::sample(), same_rule);
        let new_exception =
            AssetException::new(ResourceAddress::sample_other(), same_rule);

        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [existing_exception],
                [],
            ),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [existing_exception, new_exception],
                [],
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(
            sut.asset_exceptions_to_add_or_update,
            [new_exception]
                .into_iter()
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect_vec()
        );
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_asset_exceptions_to_remove() {
        let asset_exceptions = [AssetException::sample()];
        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions,
                [],
            ),
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(
            sut.asset_exceptions_to_be_removed,
            asset_exceptions
                .into_iter()
                .map(ScryptoManifestValue::from)
                .collect_vec()
        );
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_asset_exceptions_to_remove_and_to_add() {
        let asset_exception_from = AssetException::sample();
        let asset_exceptions_from = [asset_exception_from];
        let asset_exception_to = AssetException::sample_other();
        let asset_exceptions_to = [asset_exception_to];
        let expected_asset_exceptions_to_remove = [asset_exception_from];
        let expected_asset_exceptions_to_add = [asset_exception_to];

        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions_from,
                [],
            ),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions_to,
                [],
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(
            sut.asset_exceptions_to_add_or_update,
            expected_asset_exceptions_to_add
                .into_iter()
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect_vec()
        );
        assert_eq!(
            sut.asset_exceptions_to_be_removed,
            expected_asset_exceptions_to_remove
                .into_iter()
                .map(ScryptoManifestValue::from)
                .collect_vec()
        );
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_depositor_addresses_to_add() {
        let depositor_addresses = [
            ResourceOrNonFungible::sample(),
            ResourceOrNonFungible::sample_other(),
        ];
        let sut = SUT::new(
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [],
                depositor_addresses.clone(),
            ),
        );
        assert_eq!(sut.deposit_rule, None);

        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(
            sut.depositor_addresses_to_add,
            depositor_addresses
                .clone()
                .into_iter()
                .map(ScryptoAccountAddAuthorizedDepositorInput::from)
                .collect_vec()
        );
        assert_eq!(sut.depositor_addresses_to_remove, Vec::new());
    }

    #[test]
    fn delta_depositor_addresses_to_remove() {
        let depositor_addresses = [ResourceOrNonFungible::sample()];
        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [],
                depositor_addresses.clone(),
            ),
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
        );
        assert_eq!(sut.deposit_rule, None);

        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());
        assert_eq!(sut.depositor_addresses_to_add, Vec::new());
        assert_eq!(
            sut.depositor_addresses_to_remove,
            depositor_addresses
                .clone()
                .into_iter()
                .map(ScryptoAccountRemoveAuthorizedDepositorInput::from)
                .collect_vec()
        );
    }

    #[test]
    fn delta_depositor_addresses_to_remove_and_to_add() {
        let depositor_address_from = ResourceOrNonFungible::sample();
        let depositor_addresses_from = [depositor_address_from.clone()];
        let depositor_address_to = ResourceOrNonFungible::sample_other();
        let depositor_addresses_to = [depositor_address_to.clone()];

        let expected_depositor_addresses_to_remove = [depositor_address_from];
        let expected_depositor_addresses_to_add = [depositor_address_to];

        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [],
                depositor_addresses_from.clone(),
            ),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                [],
                depositor_addresses_to.clone(),
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(sut.asset_exceptions_to_be_removed, Vec::new());

        assert_eq!(
            sut.depositor_addresses_to_add,
            expected_depositor_addresses_to_add
                .clone()
                .into_iter()
                .map(ScryptoAccountAddAuthorizedDepositorInput::from)
                .collect_vec()
        );

        assert_eq!(
            sut.depositor_addresses_to_remove,
            expected_depositor_addresses_to_remove
                .clone()
                .into_iter()
                .map(ScryptoAccountRemoveAuthorizedDepositorInput::from)
                .collect_vec()
        );
    }
}
