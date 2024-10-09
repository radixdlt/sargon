use crate::prelude::*;
use sargon::TrackedValidatorClaim as InternalTrackedValidatorClaim;

/// A validator claim observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct TrackedValidatorClaim {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,

    /* Output */
    pub xrd_amount: Decimal192,
}

impl From<InternalTrackedValidatorClaim> for TrackedValidatorClaim {
    fn from(value: InternalTrackedValidatorClaim) -> Self {
        Self {
            validator_address: value.validator_address.into(),
            claim_nft_address: value.claim_nft_address.into(),
            claim_nft_ids: value.claim_nft_ids.into_iter().map(Into::into).collect(),
            xrd_amount: value.xrd_amount.into(),
        }
    }
}

impl Into<InternalTrackedValidatorClaim> for TrackedValidatorClaim {
    fn into(self) -> InternalTrackedValidatorClaim {
        InternalTrackedValidatorClaim {
            validator_address: self.validator_address.into(),
            claim_nft_address: self.claim_nft_address.into(),
            claim_nft_ids: self.claim_nft_ids.into_iter().map(Into::into).collect(),
            xrd_amount: self.xrd_amount.into(),
        }
    }
}