use crate::prelude::*;
use sargon::TrackedValidatorClaim as InternalTrackedValidatorClaim;

/// A validator claim observed in the transaction
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TrackedValidatorClaim {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,

    /* Output */
    pub xrd_amount: Decimal192,
}