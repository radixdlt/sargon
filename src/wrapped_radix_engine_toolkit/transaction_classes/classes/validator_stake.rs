use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct ValidatorStakeAnalyzedManifest {
    pub validator_addresses: Vec<ValidatorAddress>,
    pub validator_stakes: Vec<TrackedValidatorStake>,
}
