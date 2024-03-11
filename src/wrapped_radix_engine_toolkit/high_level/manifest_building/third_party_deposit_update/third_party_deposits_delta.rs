use crate::prelude::*;

use radix_engine_interface::blueprints::account::{
    AccountAddAuthorizedDepositorInput as ScryptoAccountAddAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
    AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ThirdPartyDepositsDelta {
    pub(crate) deposit_rule: Option<ScryptoDefaultDepositRule>,
    pub(crate) asset_exceptions_to_be_removed: Vec<ScryptoManifestValue>,
    pub(crate) asset_exceptions_to_add_or_update:
        Vec<ScryptoAccountSetResourcePreferenceInput>,
    pub(crate) depositor_addresses_to_remove:
        Vec<ScryptoAccountRemoveResourcePreferenceInput>,
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
                .into_iter()
                .filter(|x| !to.assets_exception_list.contains(x))
                .map(ScryptoManifestValue::from)
                .collect(),
            asset_exceptions_to_add_or_update: to
                .assets_exception_list
                .clone()
                .into_iter()
                .filter(|x| {
                    !from
                        .assets_exception_list
                        .clone()
                        .into_iter()
                        .any(|w| w.exception_rule == x.exception_rule)
                })
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect(),
            depositor_addresses_to_remove: from
                .depositors_allow_list
                .clone()
                .into_iter()
                .filter(|x| {
                    !to.depositors_allow_list.clone().into_iter().contains(x)
                })
                .map(ScryptoAccountRemoveResourcePreferenceInput::from)
                .collect(),
            depositor_addresses_to_add: to
                .depositors_allow_list
                .clone()
                .into_iter()
                .filter(|x| {
                    !from.depositors_allow_list.clone().into_iter().contains(x)
                })
                .map(ScryptoAccountAddAuthorizedDepositorInput::from)
                .collect(),
        }
    }
}

impl From<ResourceOrNonFungible> for ScryptoAccountAddAuthorizedDepositorInput {
    fn from(value: ResourceOrNonFungible) -> Self {
        ScryptoAccountAddAuthorizedDepositorInput {
            badge: value.into(),
        }
    }
}
impl From<ResourceOrNonFungible>
    for ScryptoAccountRemoveResourcePreferenceInput
{
    fn from(value: ResourceOrNonFungible) -> Self {
        match value {
            ResourceOrNonFungible::Resource { value } => Self {
                resource_address: value.into(),
            },
            ResourceOrNonFungible::NonFungible { value } => Self {
                resource_address: value.resource_address.into(),
            },
        }
    }
}

impl From<AssetException> for ScryptoAccountSetResourcePreferenceInput {
    fn from(value: AssetException) -> Self {
        Self {
            resource_address: value.address.into(),
            resource_preference: value.exception_rule.into(),
        }
    }
}

impl From<AssetException> for ScryptoManifestValue {
    fn from(value: AssetException) -> Self {
        ScryptoManifestValue::from(value.address)
    }
}

impl From<DepositAddressExceptionRule> for ScryptoResourcePreference {
    fn from(value: DepositAddressExceptionRule) -> Self {
        match value {
            DepositAddressExceptionRule::Allow => {
                ScryptoResourcePreference::Allowed
            }
            DepositAddressExceptionRule::Deny => {
                ScryptoResourcePreference::Disallowed
            }
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
                asset_exceptions.clone(),
                [],
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(
            sut.asset_exceptions_to_add_or_update,
            asset_exceptions
                .clone()
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
                asset_exceptions.clone(),
                [],
            ),
            ThirdPartyDeposits::new(DepositRule::AcceptAll),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(sut.asset_exceptions_to_add_or_update, Vec::new());
        assert_eq!(
            sut.asset_exceptions_to_be_removed,
            asset_exceptions
                .clone()
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
        let asset_exceptions_from = [asset_exception_from.clone()];
        let asset_exception_to = AssetException::sample_other();
        let asset_exceptions_to = [asset_exception_to.clone()];
        let expected_asset_exceptions_to_remove = [asset_exception_from];
        let expected_asset_exceptions_to_add = [asset_exception_to];

        let sut = SUT::new(
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions_from.clone(),
                [],
            ),
            ThirdPartyDeposits::with_rule_and_lists(
                DepositRule::AcceptAll,
                asset_exceptions_to.clone(),
                [],
            ),
        );
        assert_eq!(sut.deposit_rule, None);
        assert_eq!(
            sut.asset_exceptions_to_add_or_update,
            expected_asset_exceptions_to_add
                .clone()
                .into_iter()
                .map(ScryptoAccountSetResourcePreferenceInput::from)
                .collect_vec()
        );
        assert_eq!(
            sut.asset_exceptions_to_be_removed,
            expected_asset_exceptions_to_remove
                .clone()
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
                .map(ScryptoAccountRemoveResourcePreferenceInput::from)
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
                .map(ScryptoAccountRemoveResourcePreferenceInput::from)
                .collect_vec()
        );
    }
}
