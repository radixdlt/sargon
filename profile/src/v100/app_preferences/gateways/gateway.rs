use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};
use url::Url;

use super::radix_network::RadixNetwork;

/// A client the user have connected P2P with, typically a
/// WebRTC connections with a DApp, but might be Android or iPhone
/// client as well.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Gateway {
    network: RadixNetwork,
    /// The URL to the gateways API endpoint
    url: Url,
}

impl Identifiable for Gateway {
    type ID = Url;

    fn id(&self) -> Self::ID {
        self.url.clone()
    }
}
