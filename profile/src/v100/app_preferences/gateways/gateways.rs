use identified_vec::IdentifiedVecOf;
use serde::{Deserialize, Serialize};

use super::gateway::Gateway;

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Gateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    current: Gateway,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    other: IdentifiedVecOf<Gateway>,
}
