use crate::prelude::*;
use sargon::TrackedPoolContribution as InternalTrackedPoolContribution;

/// A contribution to a pool observed in the transaction
#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct TrackedPoolContribution {
    pub pool_address: PoolAddress,

    /* Input */
    pub contributed_resources: HashMap<ResourceAddress, Decimal>,

    /* Output */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,
}