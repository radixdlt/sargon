use crate::prelude::*;

/// A validator stake observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrackedValidatorStake {
    pub validator_address: ValidatorAddress,

    /* Input */
    pub xrd_amount: Decimal,

    /* Output */
    pub liquid_stake_unit_address: ResourceAddress,
    pub liquid_stake_unit_amount: Decimal,
}

impl TrackedValidatorStake {
    pub fn new(
        validator_address: impl Into<ValidatorAddress>,
        xrd_amount: impl Into<Decimal>,
        liquid_stake_unit_address: impl Into<ResourceAddress>,
        liquid_stake_unit_amount: impl Into<Decimal192>,
    ) -> Self {
        Self {
            validator_address: validator_address.into(),
            xrd_amount: xrd_amount.into(),
            liquid_stake_unit_address: liquid_stake_unit_address.into(),
            liquid_stake_unit_amount: liquid_stake_unit_amount.into(),
        }
    }
}

impl From<(RetTrackedValidatorStake, NetworkID)> for TrackedValidatorStake {
    fn from(value: (RetTrackedValidatorStake, NetworkID)) -> Self {
        let (ret, n) = value;
        Self::new(
            (ret.validator_address, n),
            ret.xrd_amount,
            (ret.liquid_stake_unit_address, n),
            ret.liquid_stake_unit_amount,
        )
    }
}
