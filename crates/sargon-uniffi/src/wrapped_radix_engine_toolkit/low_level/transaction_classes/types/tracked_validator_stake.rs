use crate::prelude::*;
use sargon::TrackedValidatorStake as InternalTrackedValidatorStake;

/// A validator stake observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TrackedValidatorStake {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub xrd_amount: Decimal,

    /* Output */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}

impl From<InternalTrackedValidatorStake> for TrackedValidatorStake {
    fn from(value: InternalTrackedValidatorStake) -> Self {
        Self {
            validator_address: value.validator_address.into(),
            xrd_amount: value.xrd_amount.into(),
            liquid_stake_unit_address: value.liquid_stake_unit_address.into(),
            liquid_stake_unit_amount: value.liquid_stake_unit_amount.into(),
        }
    }
}

impl Into<InternalTrackedValidatorStake> for TrackedValidatorStake {
    fn into(self) -> InternalTrackedValidatorStake {
        InternalTrackedValidatorStake {
            validator_address: self.validator_address.into(),
            xrd_amount: self.xrd_amount.into(),
            liquid_stake_unit_address: self.liquid_stake_unit_address.into(),
            liquid_stake_unit_amount: self.liquid_stake_unit_amount.into(),
        }
    }
}