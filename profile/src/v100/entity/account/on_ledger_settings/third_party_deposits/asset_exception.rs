use serde::{Deserialize, Serialize};

use crate::v100::address::resource_address::ResourceAddress;

use super::deposit_address_exception_rule::DepositAddressExceptionRule;

/// The specific Asset exception rule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetException {
    pub address: ResourceAddress,
    pub exception_rule: DepositAddressExceptionRule,
}
