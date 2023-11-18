use serde::{Deserialize, Serialize};

use crate::v100::address::{
    non_fungible_global_id::NonFungibleGlobalId, resource_address::ResourceAddress,
};

/// The addresses that can be added as exception to the `DepositRule`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum DepositorAddress {
    ResourceAddress(ResourceAddress),
    NonFungibleGlobalID(NonFungibleGlobalId),
}
