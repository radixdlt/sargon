use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct ValidatorClaimManifest {
	pub validator_addresses: Vec<ValidatorAddress>,
	pub validator_claims: Vec<TrackedValidatorClaim>,
}