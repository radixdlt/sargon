use crate::prelude::*;
use sargon::DetailedManifestClass as InternalDetailedManifestClass;

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
        pool_redemptions: Vec<TrackedPoolRedemption>,
    },
}

impl From<InternalDetailedManifestClass> for DetailedManifestClass {
    fn from(value: InternalDetailedManifestClass) -> Self {
        match value {
            InternalDetailedManifestClass::General => DetailedManifestClass::General,
            InternalDetailedManifestClass::Transfer { is_one_to_one } => {
                DetailedManifestClass::Transfer { is_one_to_one }
            }
            InternalDetailedManifestClass::ValidatorClaim {
                validator_addresses,
                validator_claims,
            } => DetailedManifestClass::ValidatorClaim {
                validator_addresses: validator_addresses.into(),
                validator_claims: validator_claims.into(),
            },
            InternalDetailedManifestClass::ValidatorStake {
                validator_addresses,
                validator_stakes,
            } => DetailedManifestClass::ValidatorStake {
                validator_addresses: validator_addresses.into(),
                validator_stakes: validator_stakes.into(),
            },
            InternalDetailedManifestClass::ValidatorUnstake {
                validator_addresses,
                claims_non_fungible_data,
            } => DetailedManifestClass::ValidatorUnstake {
                validator_addresses: validator_addresses.into(),
                claims_non_fungible_data: claims_non_fungible_data.into(),
            },
            InternalDetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_added,
                authorized_depositors_removed,
            } => DetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates: resource_preferences_updates.into(),
                deposit_mode_updates: deposit_mode_updates.into(),
                authorized_depositors_added: authorized_depositors_added.into(),
                authorized_depositors_removed: authorized_depositors_removed.into(),
            },
            InternalDetailedManifestClass::PoolContribution {
                pool_addresses,
                pool_contributions,
            } => DetailedManifestClass::PoolContribution {
                pool_addresses: pool_addresses.into(),
                pool_contributions: pool_contributions.into(),
            },
            InternalDetailedManifestClass::PoolRedemption {
                pool_addresses,
                pool_redemptions,
            } => DetailedManifestClass::PoolRedemption {
                pool_addresses: pool_addresses.into(),
                pool_redemptions: pool_redemptions.into(),
            },
        }
    }
}

impl Into<InternalDetailedManifestClass> for DetailedManifestClass {
    fn into(self) -> InternalDetailedManifestClass {
        match self {
            DetailedManifestClass::General => InternalDetailedManifestClass::General,
            DetailedManifestClass::Transfer { is_one_to_one } => {
                InternalDetailedManifestClass::Transfer { is_one_to_one }
            }
            DetailedManifestClass::ValidatorClaim {
                validator_addresses,
                validator_claims,
            } => InternalDetailedManifestClass::ValidatorClaim {
                validator_addresses: validator_addresses.into(),
                validator_claims: validator_claims.into(),
            },
            DetailedManifestClass::ValidatorStake {
                validator_addresses,
                validator_stakes,
            } => InternalDetailedManifestClass::ValidatorStake {
                validator_addresses: validator_addresses.into(),
                validator_stakes: validator_stakes.into(),
            },
            DetailedManifestClass::ValidatorUnstake {
                validator_addresses,
                claims_non_fungible_data,
            } => InternalDetailedManifestClass::ValidatorUnstake {
                validator_addresses: validator_addresses.into(),
                claims_non_fungible_data: claims_non_fungible_data.into(),
            },
            DetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_added,
                authorized_depositors_removed,
            } => InternalDetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates: resource_preferences_updates.into(),
                deposit_mode_updates: deposit_mode_updates.into(),
                authorized_depositors_added: authorized_depositors_added.into(),
                authorized_depositors_removed: authorized_depositors_removed.into(),
            },
            DetailedManifestClass::PoolContribution {
                pool_addresses,
                pool_contributions,
            } => InternalDetailedManifestClass::PoolContribution {
                pool_addresses: pool_addresses.into(),
                pool_contributions: pool_contributions.into(),
            },
            DetailedManifestClass::PoolRedemption {
                pool_addresses,
                pool_redemptions,
            } => InternalDetailedManifestClass::PoolRedemption {
                pool_addresses: pool_addresses.into(),
                pool_redemptions: pool_redemptions.into(),
            },
        }
    }
}