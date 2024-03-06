use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct StakeClaim {
    pub validator_address: ValidatorAddress,
    pub resource_address: NonFungibleResourceAddress,
    pub ids: Vec<NonFungibleLocalId>,
    /// The summed claim amount across ids
    pub amount: Decimal192,
}

impl StakeClaim {
    pub fn new<I>(
        validator_address: ValidatorAddress,
        resource_address: NonFungibleResourceAddress,
        ids: I,
        amount: impl Into<Decimal192>,
    ) -> Self
    where
        I: IntoIterator<Item = NonFungibleLocalId>,
    {
        Self {
            validator_address,
            resource_address,
            ids: ids.into_iter().collect_vec(),
            amount: amount.into(),
        }
    }
}

impl HasSampleValues for StakeClaim {
    fn sample() -> Self {
        Self::new(
            ValidatorAddress::sample(),
            NonFungibleResourceAddress::sample(),
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            1337,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ValidatorAddress::sample_other(),
            NonFungibleResourceAddress::sample_other(),
            [NonFungibleLocalId::sample_other()],
            237,
        )
    }
}
