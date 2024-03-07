use crate::prelude::*;

use radix_engine::types::indexmap::IndexSet;
use radix_engine_toolkit::transaction_types::DetailedManifestClass as RetDetailedManifestClass;

use radix_engine_common::types::ComponentAddress as ScryptoComponentAddress;

fn map<T>(
    addresses: IndexSet<ScryptoComponentAddress>,
    network_id: NetworkID,
) -> Vec<T>
where
    T: From<(ScryptoComponentAddress, NetworkID)>,
{
    vec![]
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
                pool_addresses: map(pool_addresses, n),
                pool_contributions: pool_contributions.into_iter().map(Into::<TrackedPoolContribution>::into).collect_vec(),
            },
            RetDetailedManifestClass::PoolRedemption {
                pool_addresses,
                pool_redemptions,
            } => todo!(),
            RetDetailedManifestClass::ValidatorStake {
                validator_addresses,
                validator_stakes,
            } => todo!(),
            RetDetailedManifestClass::ValidatorUnstake {
                validator_addresses,
                validator_unstakes,
                claims_non_fungible_data,
            } => todo!(),
            RetDetailedManifestClass::ValidatorClaim {
                validator_addresses,
                validator_claims,
            } => todo!(),
            RetDetailedManifestClass::AccountDepositSettingsUpdate {
                resource_preferences_updates,
                deposit_mode_updates,
                authorized_depositors_updates,
            } => todo!(),
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
        claims_non_fungible_data: Vec<UnstakeDataEntry>,
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
