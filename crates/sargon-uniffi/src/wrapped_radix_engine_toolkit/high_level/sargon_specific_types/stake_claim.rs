use crate::prelude::*;
use sargon::StakeClaim as InternalStakeClaim;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct StakeClaim {
    pub validator_address: ValidatorAddress,
    pub resource_address: NonFungibleResourceAddress,
    pub ids: Vec<NonFungibleLocalId>,
    /// The summed claim amount across ids
    pub amount: Decimal192,
}

impl From<InternalStakeClaim> for StakeClaim {
    fn from(value: InternalStakeClaim) -> Self {
        Self {
            validator_address: value.validator_address.into(),
            resource_address: value.resource_address.into(),
            ids: value.ids.into_vec(),
            amount: value.amount.into(),
        }
    }
}

impl Into<InternalStakeClaim> for StakeClaim {
    fn into(self) -> InternalStakeClaim {
        InternalStakeClaim {
            validator_address: self.validator_address.into(),
            resource_address: self.resource_address.into(),
            ids: self.ids.into_internal_vec(),
            amount: self.amount.into(),
        }
    }
}

#[uniffi::export]
pub fn new_stake_claim_sample() -> StakeClaim {
    InternalStakeClaim::sample().into()
}

#[uniffi::export]
pub fn new_stake_claim_sample_other() -> StakeClaim {
    InternalStakeClaim::sample_other().into()
}
