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
pub struct TrackedValidatorClaim {
    pub validator_address: ValidatorAddress,
    /* Input */
    pub claim_nft_address: ValidatorAddress,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,
    /* Output */
    pub xrd_amount: Decimal192,
}