use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedPoolRedemption as RetTrackedPoolRedemption;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TrackedPoolRedemption {
    pub pool_address: PoolAddress,

    /* Input */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,

    /* Output */
    pub redeemed_resources: HashMap<ResourceAddress, Decimal192>,
}

impl From<(RetTrackedPoolRedemption, NetworkID)> for TrackedPoolRedemption {
    fn from(value: (RetTrackedPoolRedemption, NetworkID)) -> Self {
        let (ret, n) = value;
        Self {
            pool_address: (ret.pool_address, n).into(),
            pool_units_resource_address: (ret.pool_units_resource_address, n)
                .into(),
            pool_units_amount: ret.pool_units_amount.into(),
            redeemed_resources: to_hashmap_network_aware_key(
                ret.redeemed_resources,
                n,
            ),
        }
    }
}
