use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::network::network::Network;
use wallet_kit_common::network_id::NetworkID;

/// An ordered mapping of NetworkID -> `Profile.Network`, containing
/// all the users Accounts, Personas and AuthorizedDapps the user
/// has created and interacted with on this network.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Networks(BTreeMap<NetworkID, Network>);

impl Networks {
    /// Instantiates a new empty networks collection.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Instantiates a new network collection with the provided
    /// `network`.
    pub fn with_network(network: Network) -> Self {
        let mut map = BTreeMap::new();
        map.insert(network.id, network);
        Self(map)
    }
}

impl Default for Networks {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}
