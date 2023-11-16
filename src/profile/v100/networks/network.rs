use serde::{Deserialize, Serialize};

use super::network_id::NetworkID;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Network {
    /// Network ID
    pub id: NetworkID,
}
