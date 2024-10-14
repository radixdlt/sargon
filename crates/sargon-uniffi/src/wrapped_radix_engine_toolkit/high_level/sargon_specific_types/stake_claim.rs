use crate::prelude::*;
use sargon::StakeClaim as InternalStakeClaim;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct StakeClaim {
    pub validator_address: ValidatorAddress,
    pub resource_address: NonFungibleResourceAddress,
    pub ids: Vec<NonFungibleLocalId>,
    /// The summed claim amount across ids
    pub amount: Decimal192,
}

#[uniffi::export]
pub fn new_stake_claim_sample() -> StakeClaim {
    InternalStakeClaim::sample().into()
}

#[uniffi::export]
pub fn new_stake_claim_sample_other() -> StakeClaim {
    InternalStakeClaim::sample_other().into()
}
