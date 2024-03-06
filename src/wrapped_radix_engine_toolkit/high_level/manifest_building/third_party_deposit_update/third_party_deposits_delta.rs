use crate::prelude::*;

use radix_engine_interface::blueprints::account::AccountAddAuthorizedDepositorInput as ScryptoAccountAddAuthorizedDepositorInput;
use radix_engine_interface::blueprints::account::AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput;
use radix_engine_interface::blueprints::account::AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput;
use radix_engine_interface::blueprints::account::DefaultDepositRule as ScryptoDefaultDepositRule;
use radix_engine_interface::blueprints::account::ResourcePreference as ScryptoResourcePreference;
use transaction::prelude::ManifestValue as ScryptoManifestValue;

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
            deposit_rule: deposit_rule.map(Into::into),
            asset_exceptions_to_be_removed: from
                .assets_exception_list
                .clone()
                .into_iter()
                .filter(|x| !to.assets_exception_list.contains(x))
                .map(|x| x.address)
                .map(Into::<ScryptoManifestValue>::into)
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
                .map(Into::<ScryptoAccountSetResourcePreferenceInput>::into)
                .collect(),
            depositor_addresses_to_remove: from
                .depositors_allow_list
                .clone()
                .into_iter()
                .filter(|x| {
                    !to.depositors_allow_list.clone().into_iter().contains(x)
                })
                .map(Into::<ScryptoAccountRemoveResourcePreferenceInput>::into)
                .collect(),
            depositor_addresses_to_add: to
                .depositors_allow_list
                .clone()
                .into_iter()
                .filter(|x| {
                    !from.depositors_allow_list.clone().into_iter().contains(x)
                })
                .map(Into::<ScryptoAccountAddAuthorizedDepositorInput>::into)
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
                resource_address: value.resource_address.into(), // IS THIS CORRECT?
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

impl From<DepositRule> for ScryptoDefaultDepositRule {
    fn from(value: DepositRule) -> Self {
        match value {
            DepositRule::AcceptKnown => {
                ScryptoDefaultDepositRule::AllowExisting
            }
            DepositRule::AcceptAll => ScryptoDefaultDepositRule::Accept,
            DepositRule::DenyAll => ScryptoDefaultDepositRule::Reject,
        }
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
