use crate::prelude::*;
use sargon::TrackedPoolRedemption as InternalTrackedPoolRedemption;

/// A pool redemptions observed in the transaction
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TrackedPoolRedemption {
    pub pool_address: PoolAddress,

    /* Input */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,

    /* Output */
    pub redeemed_resources: HashMap<ResourceAddress, Decimal192>,
}
