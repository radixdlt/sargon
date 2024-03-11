use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedPoolRedemption as RetTrackedPoolRedemption;

/// A pool redemptions observed in the transaction
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TrackedPoolRedemption {
    pub pool_address: PoolAddress,

    /* Input */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,

    /* Output */
    pub redeemed_resources: HashMap<ResourceAddress, Decimal192>,
}

impl TrackedPoolRedemption {
    pub fn new(
        pool_address: impl Into<PoolAddress>,
        pool_units_resource_address: impl Into<ResourceAddress>,
        pool_units_amount: impl Into<Decimal192>,
        redeemed_resources: impl Into<HashMap<ResourceAddress, Decimal192>>,
    ) -> Self {
        Self {
            pool_address: pool_address.into(),
            pool_units_resource_address: pool_units_resource_address.into(),
            pool_units_amount: pool_units_amount.into(),
            redeemed_resources: redeemed_resources.into(),
        }
    }
}

impl From<(RetTrackedPoolRedemption, NetworkID)> for TrackedPoolRedemption {
    fn from(value: (RetTrackedPoolRedemption, NetworkID)) -> Self {
        let (ret, n) = value;
        Self::new(
            (ret.pool_address, n),
            (ret.pool_units_resource_address, n),
            ret.pool_units_amount,
            to_hashmap_network_aware_key(ret.redeemed_resources, n),
        )
    }
}
