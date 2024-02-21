use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub enum DetailedManifestClass {
    General,
    Transfer,

    ValidatorClaim {
        value: ValidatorClaimAnalyzedManifest
    },

    ValidatorStake {
       value: ValidatorStakeAnalyzedManifest
    },

    ValidatorUnstake {
        value: ValidatorUnstakeAnalyzedManifest
    },

    AccountDepositSettingsUpdate {
        value: AccountDepositSettingsUpdateAnalyzedManifest
    },

//     PoolContribution {
//         poolAddresses: Vec<ComponentAddress>,
//         poolContributions: Vec<TrackedPoolContribution>,
//     },

//     PoolRedemption {
//         poolAddresses: Vec<ComponentAddress>,
//         poolContributions: Vec<TrackedPoolRedemption>,
//     },
}
