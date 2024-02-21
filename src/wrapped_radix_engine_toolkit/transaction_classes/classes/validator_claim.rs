use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct ValidatorClaimAnalyzedManifest {
	pub validator_addresses: Vec<ValidatorAddress>,
	pub validator_claims: Vec<TrackedValidatorClaim>,
}