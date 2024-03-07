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

impl From<(RetTrackedPoolContribution, NetworkID)> for TrackedPoolContribution {
    fn from(value: (RetTrackedPoolContribution, NetworkID)) -> Self {
        let (ret, network_id) = value;
        Self {
            pool_address: (ret.pool_address, network_id).into(),
            contributed_resources: ret.contributed_resources.into_iter()
            .map(|(k, v)| {
                (
                    Into::<ResourceAddress>::into((k, network_id)),
                    Into::<Decimal192>::into(v)
                )
            })
            .collect::<HashMap<_, _>>(),
            pool_units_resource_address: (ret.pool_units_resource_address, network_id).into(),
            pool_units_amount: ret.pool_units_amount.into(),
        }
    }
}