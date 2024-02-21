use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct ValidatorUnstakeAnalyzedManifest {
    pub validator_addresses: Vec<ValidatorAddress>,
    pub validator_unstakes: Vec<TrackedValidatorUnstake>,
    pub claims_non_fungible_data: Vec<UnstakeDataEntry>,
}