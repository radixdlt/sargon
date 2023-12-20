use std::{borrow::BorrowMut, cell::Cell, collections::BTreeMap, ops::AddAssign};

use identified_vec::{IdentifiedVecOf, IsIdentifiableVecOfVia, IsIdentifiedVec, IsIdentifiedVecOf};
use serde::{Deserialize, Serialize};

use crate::{identified_vec_via::IdentifiedVecVia, v100::header::content_hint::ContentHint};

use super::network::network::Network;
use wallet_kit_common::network_id::NetworkID;

/// An ordered mapping of NetworkID -> `Profile.Network`, containing
/// all the users Accounts, Personas and AuthorizedDapps the user
/// has created and interacted with on this network.
pub type Networks = IdentifiedVecVia<Network>;

// Constructors
impl Networks {
    /// Instantiates a new network collection with the provided
    /// `network`.
    pub fn with_network(network: Network) -> Self {
        // let mut map = BTreeMap::new();
        // map.insert(network.id(), network);
        // Self(map)
        let vec = IdentifiedVecOf::<Network>::from_iter([network]);
        Self::from_identified_vec_of(vec)
    }
}

impl Networks {
    pub fn content_hint(&self) -> ContentHint {
        let number_of_accounts = self.iter().fold(0, |acc, x| acc + x.accounts().len());
        ContentHint::with_counters(number_of_accounts, 0, self.len())
    }
}

// Trait: Default
impl Default for Networks {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Networks {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::with_network(Network::placeholder())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_other() -> Self {
        Self::with_network(Network::placeholder_other())
    }
}

#[cfg(test)]
mod tests {
    use identified_vec::IsIdentifiedVec;
    use wallet_kit_common::network_id::NetworkID;

    use crate::v100::{
        entity::account::account::Account,
        header::content_hint::ContentHint,
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
    fn inequality() {
        assert_ne!(Networks::placeholder(), Networks::placeholder_other());
    }

    #[test]
    fn with_network() {
        let network = Network::new(
            NetworkID::Mainnet,
            Accounts::with_account(Account::placeholder_mainnet()),
        );
        assert_eq!(Networks::with_network(network).len(), 1);
    }

    #[test]
    fn content_hint() {
        assert_eq!(
            Networks::placeholder().content_hint(),
            ContentHint::with_counters(2, 0, 1)
        );
    }
}
