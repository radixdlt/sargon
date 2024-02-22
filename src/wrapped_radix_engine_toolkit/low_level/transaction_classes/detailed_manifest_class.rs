use crate::prelude::*;

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
