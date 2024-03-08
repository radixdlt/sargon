use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedPoolContribution as RetTrackedPoolContribution;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TrackedPoolContribution {
    pub pool_address: PoolAddress,

    /* Input */
    pub contributed_resources: HashMap<ResourceAddress, Decimal>,

    /* Output */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,
}

impl TrackedPoolContribution {
    pub fn new(
        pool_address: impl Into<PoolAddress>,
        contributed_resources: impl Into<HashMap<ResourceAddress, Decimal>>,
        pool_units_resource_address: impl Into<ResourceAddress>,
        pool_units_amount: impl Into<Decimal192>,
    ) -> Self {
        Self {
            pool_address: pool_address.into(),
            contributed_resources: contributed_resources.into(),
            pool_units_resource_address: pool_units_resource_address.into(),
            pool_units_amount: pool_units_amount.into(),
        }
    }
}

impl From<(RetTrackedPoolContribution, NetworkID)> for TrackedPoolContribution {
    fn from(value: (RetTrackedPoolContribution, NetworkID)) -> Self {
        let (ret, network_id) = value;
        Self::new(
            (ret.pool_address, network_id),
            to_hashmap_network_aware_key(ret.contributed_resources, network_id),
            (ret.pool_units_resource_address, network_id),
            ret.pool_units_amount,
        )
    }
}
