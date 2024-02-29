use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct StakeClaim {
    pub validator_address: ValidatorAddress,
    pub resource_address: ResourceAddress,
    pub ids: Vec<NonFungibleLocalId>,
    /// The summed claim amount across ids
    pub amount: Decimal192,
}
