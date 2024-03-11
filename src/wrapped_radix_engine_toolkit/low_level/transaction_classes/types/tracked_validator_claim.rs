use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedValidatorClaim as RetTrackedValidatorClaim;

/// A validator claim observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TrackedValidatorClaim {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,

    /* Output */
    pub xrd_amount: Decimal192,
}

impl TrackedValidatorClaim {
    pub fn new(
        validator_address: impl Into<ValidatorAddress>,
        claim_nft_address: impl Into<ResourceAddress>,
        claim_nft_ids: impl IntoIterator<Item = NonFungibleLocalId>,
        xrd_amount: impl Into<Decimal>,
    ) -> Self {
        Self {
            validator_address: validator_address.into(),
            claim_nft_address: claim_nft_address.into(),
            claim_nft_ids: claim_nft_ids.into_iter().collect(),
            xrd_amount: xrd_amount.into(),
        }
    }
}

impl From<(RetTrackedValidatorClaim, NetworkID)> for TrackedValidatorClaim {
    fn from(value: (RetTrackedValidatorClaim, NetworkID)) -> Self {
        let (ret, n) = value;

        Self::new(
            (ret.validator_address, n),
            (ret.claim_nft_address, n),
            ret.claim_nft_ids.into_iter().map(NonFungibleLocalId::from),
            ret.xrd_amount,
        )
    }
}
