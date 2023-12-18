use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::network::network::Network;
use wallet_kit_common::network_id::NetworkID;

/// An ordered mapping of NetworkID -> `Profile.Network`, containing
/// all the users Accounts, Personas and AuthorizedDapps the user
/// has created and interacted with on this network.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Networks(BTreeMap<NetworkID, Network>);

// Constructors
impl Networks {
    /// Instantiates a new empty networks collection.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Instantiates a new network collection with the provided
    /// `network`.
    pub fn with_network(network: Network) -> Self {
        let mut map = BTreeMap::new();
        map.insert(network.id(), network);
        Self(map)
    }
}

// Getters
impl Networks {
    /// Returns the number of networks user has accounts on.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// Trait: Default
impl Default for Networks {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::network_id::NetworkID;

    use crate::v100::{
        entity::account::account::Account,
        networks::{
            network::{accounts::Accounts, network::Network},
            networks::Networks,
        },
    };

    #[test]
    fn default_is_empty() {
        assert_eq!(Networks::default().len(), 0)
    }

    #[test]
    fn with_network() {
        let network = Network::new(
            NetworkID::Mainnet,
            Accounts::with_account(Account::placeholder_mainnet()),
        );
        assert_eq!(Networks::with_network(network).len(), 1);
    }
}
