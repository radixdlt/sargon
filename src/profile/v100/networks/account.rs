use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::{account_address::AccountAddress, network_id::NetworkID};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    pub network_id: NetworkID,
    pub display_name: String,
    pub address: AccountAddress,
}
