use crate::prelude::*;

/// A validator claim observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl From<(RetValidatorClaimOperation, NetworkID)> for TrackedValidatorClaim {
    fn from(value: (RetValidatorClaimOperation, NetworkID)) -> Self {
        let (ret, n) = value;

        Self::new(
            (ret.validator_address, n),
            (ret.claim_nft_address, n),
            ret.claim_nft_ids.into_iter().map(NonFungibleLocalId::from),
            ret.xrd_amount,
        )
    }
}

impl HasSampleValues for TrackedValidatorClaim {
    fn sample() -> Self {
        Self::new(
            ValidatorAddress::sample(),
            ResourceAddress::sample(),
            Vec::<NonFungibleLocalId>::sample(),
            Decimal192::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ValidatorAddress::sample_other(),
            ResourceAddress::sample_other(),
            Vec::<NonFungibleLocalId>::sample_other(),
            Decimal192::sample_other(),
        )
    }
}
