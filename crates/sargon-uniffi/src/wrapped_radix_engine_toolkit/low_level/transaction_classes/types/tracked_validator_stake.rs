use crate::prelude::*;
use sargon::TrackedValidatorStake as InternalTrackedValidatorStake;

/// A validator stake observed in the transaction
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TrackedValidatorStake {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub xrd_amount: Decimal,

    /* Output */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}
