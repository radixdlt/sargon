use crate::prelude::*;

use radix_engine::types::indexmap::IndexSet;
use radix_engine_toolkit::transaction_types::{
    DetailedManifestClass as RetDetailedManifestClass,
    Operation as RetOperation,
};

use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;

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

pub(crate) fn to_hashmap_network_aware_key<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<(K, NetworkID)>,
    U: From<V>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from((k, network_id)), U::from(v)))
        .collect::<HashMap<L, U>>()
}

pub(crate) fn filter_try_to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: TryFrom<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::try_from)
        .filter_map(Result::ok)
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
            } => {
                let pool_contributions =
                    to_vec_network_aware(pool_contributions, n);
                let pool_addresses = to_vec_network_aware(pool_addresses, n);

                Self::PoolContribution {
                    pool_addresses,
                    pool_contributions,
                }
            }

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
            } => {
                let deposit_mode_updates: HashMap<AccountAddress, DepositRule> =
                    to_hashmap_network_aware_key(deposit_mode_updates, n);

                let resource_preferences_updates = resource_preferences_updates
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            AccountAddress::from((k, n)),
                            v.into_iter()
                                .map(|(k, v)| {
                                    (
                                        ResourceAddress::from((k, n)),
                                        ResourcePreferenceUpdate::from(v),
                                    )
                                })
                                .collect::<HashMap<
                                    ResourceAddress,
                                    ResourcePreferenceUpdate,
                                >>(),
                        )
                    })
                    .collect::<HashMap<
                        AccountAddress,
                        HashMap<ResourceAddress, ResourcePreferenceUpdate>,
                    >>();

                let split_map_auth_dep = |o: RetOperation| {
                    authorized_depositors_updates.clone().into_iter().map(|(k, v)| {
                            (
                                AccountAddress::from((k, n)),
                                v.into_iter().filter(|x| x.1 == o).map(|x| (x.0, n)).map(ResourceOrNonFungible::from).collect_vec()
                            )
                        }).collect::<HashMap<
                        AccountAddress,
                        Vec<ResourceOrNonFungible>,
                    >>()
                };

                let authorized_depositors_added =
                    split_map_auth_dep(RetOperation::Added);
                let authorized_depositors_removed =
                    split_map_auth_dep(RetOperation::Removed);

                Self::AccountDepositSettingsUpdate {
                    resource_preferences_updates,
                    deposit_mode_updates,
                    authorized_depositors_added,
                    authorized_depositors_removed,
                }
            }
        }
    }
}

impl From<(ScryptoResourceOrNonFungible, NetworkID)> for ResourceOrNonFungible {
    fn from(value: (ScryptoResourceOrNonFungible, NetworkID)) -> Self {
        let (resource_or_non_fungible, network_id) = value;
        match resource_or_non_fungible {
            ScryptoResourceOrNonFungible::NonFungible(nf) => {
                Self::NonFungible {
                    value: (nf, network_id).into(),
                }
            }
            ScryptoResourceOrNonFungible::Resource(resource_address) => {
                Self::Resource {
                    value: (resource_address, network_id).into(),
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, uniffi::Enum)]
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
        pool_addresses: Vec<PoolAddress>,
        pool_contributions: Vec<TrackedPoolContribution>,
    },
    PoolRedemption {
        pool_addresses: Vec<PoolAddress>,
        pool_contributions: Vec<TrackedPoolRedemption>,
    },
}
