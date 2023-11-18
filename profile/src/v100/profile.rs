use serde::{Deserialize, Serialize};

use super::{header::header::Header, networks::networks::Networks};

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    pub header: Header,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    pub networks: Networks,
}
