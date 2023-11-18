use std::collections::BTreeSet;

use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};

use crate::v100::entity::account::account::Account;

use super::network_id::NetworkID;

/// A NonEmpty ordered set of Accounts on a specific network.
pub type Accounts = NonEmpty<BTreeSet<Account>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub id: NetworkID,

    /// A non empty ordered set of Accounts on this network.
    pub accounts: Accounts,
}
