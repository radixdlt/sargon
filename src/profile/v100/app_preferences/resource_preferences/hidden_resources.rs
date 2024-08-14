use crate::prelude::*;

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Hash, uniffi::Record,
)]
pub struct HiddenResources {
    pub fungible: Vec<ResourceAddress>,
    pub non_fungible: Vec<NonFungibleGlobalId>,
    pub pool_unit: Vec<PoolAddress>,
}
