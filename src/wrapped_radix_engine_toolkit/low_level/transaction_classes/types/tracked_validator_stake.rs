use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedValidatorStake as RetTrackedValidatorStake;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TrackedValidatorStake {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub xrd_amount: Decimal,

    /* Output */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}

impl From<(RetTrackedValidatorStake, NetworkID)> for TrackedValidatorStake {
    fn from(value: (RetTrackedValidatorStake, NetworkID)) -> Self {
        let (ret, n) = value;
        Self {
            validator_address: (ret.validator_address, n).into(),
            xrd_amount: ret.xrd_amount.into(),
            liquid_stake_unit_address: (ret.liquid_stake_unit_address, n)
                .into(),
            liquid_stake_unit_amount: ret.liquid_stake_unit_amount.into(),
        }
    }
}
