use crate::prelude::*;

use radix_engine::types::indexmap::IndexSet;
use radix_engine_toolkit::transaction_types::DetailedManifestClass as RetDetailedManifestClass;

use radix_engine_common::types::ComponentAddress as ScryptoComponentAddress;

pub(crate) fn to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: From<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::from)
        .collect_vec()
}

impl From<(RetDetailedManifestClass, NetworkID)> for DetailedManifestClass {
    fn from(value: (RetDetailedManifestClass, NetworkID)) -> Self {
        let n = value.1;
        match value.0 {
            RetDetailedManifestClass::General => Self::General,

            RetDetailedManifestClass::Transfer { is_one_to_one: _ } => {
                Self::Transfer
            }

            RetDetailedManifestClass::PoolContribution {
                pool_addresses,
                pool_contributions,
            } => Self::PoolContribution {
                pool_addresses: to_vec_network_aware(pool_addresses, n),
                pool_contributions: to_vec_network_aware(pool_contributions, n),
            },

            RetDetailedManifestClass::PoolRedemption {
                pool_addresses,
                pool_redemptions,
            } => Self::PoolRedemption {
                pool_addresses: to_vec_network_aware(pool_addresses, n),
                pool_contributions: to_vec_network_aware(pool_redemptions, n),
            },

            RetDetailedManifestClass::ValidatorStake {
                validator_addresses,
                validator_stakes,
            } => Self::ValidatorStake {
                validator_addresses: to_vec_network_aware(
                    validator_addresses,
                    n,
                ),
                validator_stakes: to_vec_network_aware(validator_stakes, n),
            },

            RetDetailedManifestClass::ValidatorUnstake {
                validator_addresses,
                validator_unstakes: _,
                claims_non_fungible_data,
            } => Self::ValidatorUnstake {
                validator_addresses: to_vec_network_aware(
                    validator_addresses,
                    n,
                ),
                claims_non_fungible_data: claims_non_fungible_data
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            NonFungibleGlobalId::from((k, n)),
                            UnstakeData::from(v),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            },

            RetDetailedManifestClass::ValidatorClaim {
                validator_addresses,
                validator_claims,
            } => Self::ValidatorClaim {
                validator_addresses: to_vec_network_aware(
                    validator_addresses,
                    n,
                ),
                validator_claims: to_vec_network_aware(validator_claims, n),
            },

            RetDetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_updates,
            } => Self::AccountDepositSettingsUpdate { resource_preferences_updates: todo!(), deposit_mode_updates: to_hashmap_network_aware_key(deposit_mode_updates, n), authorized_depositors_added: authorized_depositors_updates.clone().into_iter().filter_map(|x| x.), authorized_depositors_removed: () },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum DetailedManifestClass {
    General,
    Transfer,

    ValidatorClaim {
        validator_addresses: Vec<ValidatorAddress>,
        validator_claims: Vec<TrackedValidatorClaim>,
    },

    ValidatorStake {
        validator_addresses: Vec<ValidatorAddress>,
        validator_stakes: Vec<TrackedValidatorStake>,
    },

    ValidatorUnstake {
        validator_addresses: Vec<ValidatorAddress>,
        claims_non_fungible_data: HashMap<NonFungibleGlobalId, UnstakeData>,
    },

    AccountDepositSettingsUpdate {
        resource_preferences_updates: HashMap<
            AccountAddress,
            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
        >,
        deposit_mode_updates: HashMap<AccountAddress, DepositRule>,
        authorized_depositors_added:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
        authorized_depositors_removed:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
    },
    PoolContribution {
        pool_addresses: Vec<ComponentAddress>,
        pool_contributions: Vec<TrackedPoolContribution>,
    },
    PoolRedemption {
        pool_addresses: Vec<ComponentAddress>,
        pool_contributions: Vec<TrackedPoolRedemption>,
    },
}
