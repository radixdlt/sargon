use crate::prelude::*;
use sargon::TrackedPoolContribution as InternalTrackedPoolContribution;

/// A contribution to a pool observed in the transaction
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TrackedPoolContribution {
    pub pool_address: PoolAddress,

    /* Input */
    pub contributed_resources: HashMap<ResourceAddress, Decimal>,

    /* Output */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,
}

impl From<InternalTrackedPoolContribution> for TrackedPoolContribution {
    fn from(value: InternalTrackedPoolContribution) -> Self {
        Self {
            pool_address: value.pool_address.into(),
            contributed_resources: value
                .contributed_resources
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            pool_units_resource_address: value
                .pool_units_resource_address
                .into(),
            pool_units_amount: value.pool_units_amount.into(),
        }
    }
}

impl Into<InternalTrackedPoolContribution> for TrackedPoolContribution {
    fn into(self) -> InternalTrackedPoolContribution {
        InternalTrackedPoolContribution {
            pool_address: self.pool_address.into(),
            contributed_resources: self
                .contributed_resources
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            pool_units_resource_address: self
                .pool_units_resource_address
                .into(),
            pool_units_amount: self.pool_units_amount.into(),
        }
    }
}
