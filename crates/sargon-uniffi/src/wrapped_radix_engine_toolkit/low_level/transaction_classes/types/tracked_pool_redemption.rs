use crate::prelude::*;
use sargon::TrackedPoolRedemption as InternalTrackedPoolRedemption;

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

impl From<InternalTrackedPoolRedemption> for TrackedPoolRedemption {
    fn from(value: InternalTrackedPoolRedemption) -> Self {
        Self {
            pool_address: value.pool_address.into(),
            pool_units_resource_address: value.pool_units_resource_address.into(),
            pool_units_amount: value.pool_units_amount.into(),
            redeemed_resources: value
                .redeemed_resources
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl Into<InternalTrackedPoolRedemption> for TrackedPoolRedemption {
    fn into(self) -> InternalTrackedPoolRedemption {
        InternalTrackedPoolRedemption {
            pool_address: self.pool_address.into(),
            pool_units_resource_address: self.pool_units_resource_address.into(),
            pool_units_amount: self.pool_units_amount.into(),
            redeemed_resources: self
                .redeemed_resources
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}