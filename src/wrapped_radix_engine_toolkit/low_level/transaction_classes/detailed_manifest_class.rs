use crate::prelude::*;

use radix_engine::types::indexmap::IndexSet;
use radix_engine_toolkit::transaction_types::{
    DetailedManifestClass as RetDetailedManifestClass,
    Operation as RetOperation,
};

/// The execution summary process not only determines the class of the manifest,
/// but also includes additional information about this class that the wallet
/// requires to display to the user.
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, uniffi::Enum)]
pub enum DetailedManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    ///
    /// No additional information is required beyond what the execution summary
    /// will provide.
    General,

    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer {
        /// When `true`, then this is a one-to-one transfer and the wallet can
        /// regard this as a "simple transfer" and communicate this information
        /// to the ledger hardware wallet. Otherwise, if `false`, then this is
        /// not a one-to-one transfer.
        is_one_to_one: bool,
    },

    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The claims observed in the transaction
        validator_claims: Vec<TrackedValidatorClaim>,
    },

    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The stake observed in the transaction
        validator_stakes: Vec<TrackedValidatorStake>,
    },

    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,

        /// The data associated with the various claim NFTs
        claims_non_fungible_data: HashMap<NonFungibleGlobalId, UnstakeData>,
    },

    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate {
        /// Updates to the resource preferences of the account deposit settings.
        /// account_address -> (resource_address -> Update<new_preference>)
        resource_preferences_updates: HashMap<
            AccountAddress,
            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
        >,
        /// Changes to the account's deposit mode.
        /// account_address -> new_default_deposit_mode
        deposit_mode_updates: HashMap<AccountAddress, DepositRule>,
        /// Additions to the authorized depositors
        authorized_depositors_added:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
        /// Removals from the authorized depositors
        authorized_depositors_removed:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
    },

    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,
        /// The contribution observed in the transaction
        pool_contributions: Vec<TrackedPoolContribution>,
    },

    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,

        /// The redemptions observed in the transaction
        pool_contributions: Vec<TrackedPoolRedemption>,
    },
}

impl From<(RetDetailedManifestClass, NetworkID)> for DetailedManifestClass {
    fn from(value: (RetDetailedManifestClass, NetworkID)) -> Self {
        let n = value.1;
        match value.0 {
            RetDetailedManifestClass::General => Self::General,

            RetDetailedManifestClass::Transfer { is_one_to_one } => {
                Self::Transfer { is_one_to_one }
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
