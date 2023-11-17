use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{network::Network, network_id::NetworkID};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Networks(BTreeMap<NetworkID, Network>);

impl Networks {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn with_network(network: Network) -> Self {
        let mut map = BTreeMap::new();
        map.insert(network.id, network);
        Self(map)
    }
}

impl Default for Networks {
    fn default() -> Self {
        Self::new()
    }
}
