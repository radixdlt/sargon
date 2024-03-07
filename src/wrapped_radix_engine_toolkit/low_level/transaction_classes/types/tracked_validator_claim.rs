use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedValidatorClaim as RetTrackedValidatorClaim;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TrackedValidatorClaim {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub claim_nft_address: ResourceAddress,
    pub claim_nft_ids: Vec<NonFungibleLocalId>,

    /* Output */
    pub xrd_amount: Decimal192,
}

impl From<(RetTrackedValidatorClaim, NetworkID)> for TrackedValidatorClaim {
    fn from(value: (RetTrackedValidatorClaim, NetworkID)) -> Self {
        let (ret, n) = value;
        Self {
            validator_address: (ret.validator_address, n).into(),
            claim_nft_address: (ret.claim_nft_address, n).into(),
            claim_nft_ids: ret
                .claim_nft_ids
                .into_iter()
                .map(NonFungibleLocalId::from)
                .collect(),
            xrd_amount: ret.xrd_amount.into(),
        }
    }
}
