use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};

use super::{
    account::{Account, IdentifiedArrayOf},
    network_id::NetworkID,
};

pub type Accounts = NonEmpty<IdentifiedArrayOf<Account>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub id: NetworkID,

    pub accounts: Accounts,
}
