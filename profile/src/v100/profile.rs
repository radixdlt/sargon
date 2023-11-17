use serde::{Deserialize, Serialize};

use super::{header::header::Header, networks::networks::Networks};

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub header: Header,
    pub networks: Networks,
}
