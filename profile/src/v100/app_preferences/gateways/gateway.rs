use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use url::Url;
use wallet_kit_common::network_id::NetworkID;

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

impl Display for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.network.display_description,
            self.url.to_string(),
        )
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::mainnet()
    }
}

impl Gateway {
    fn declare(url: &str, id: NetworkID) -> Self {
        Self {
            url: Url::try_from(url).expect("Valid URL"),
            network: RadixNetwork::lookup_by_id(id).expect("Network for ID"),
        }
    }
}

impl Gateway {
    pub fn mainnet() -> Self {
        Self::declare("https://mainnet.radixdlt.com/", NetworkID::Mainnet)
    }

    pub fn stokenet() -> Self {
        Self::declare(
            "https://babylon-stokenet-gateway.radixdlt.com/",
            NetworkID::Stokenet,
        )
    }

    pub fn rcnet() -> Self {
        Self::declare("https://rcnet-v3.radixdlt.com/", NetworkID::Zabanet)
    }

    pub fn nebunet() -> Self {
        Self::declare("https://betanet.radixdlt.com/", NetworkID::Nebunet)
    }

    pub fn kisharnet() -> Self {
        Self::declare("https://rcnet.radixdlt.com/", NetworkID::Kisharnet)
    }

    pub fn ansharnet() -> Self {
        Self::declare(
            "https://ansharnet-gateway.radixdlt.com/",
            NetworkID::Ansharnet,
        )
    }

    pub fn hammunet() -> Self {
        Self::declare(
            "https://hammunet-gateway.radixdlt.com/",
            NetworkID::Hammunet,
        )
    }

    pub fn enkinet() -> Self {
        Self::declare("https://enkinet-gateway.radixdlt.com/", NetworkID::Enkinet)
    }

    pub fn mardunet() -> Self {
        Self::declare(
            "https://mardunet-gateway.radixdlt.com/",
            NetworkID::Mardunet,
        )
    }
}

impl Gateway {
    fn wellknown() -> Vec<Self> {
        vec![Self::mainnet(), Self::stokenet()]
    }

    pub fn is_wellknown(&self) -> bool {
        Self::wellknown().contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use identified_vec::Identifiable;
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::Gateway;

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Gateway::mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "network":
                {
                    "name": "mainnet",
                    "id": 1,
                    "displayDescription": "Mainnet"
                },
                "url": "https://mainnet.radixdlt.com/"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = Gateway::stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "network":
                {
                    "name": "stokenet",
                    "id": 2,
                    "displayDescription": "Stokenet"
                },
                "url": "https://babylon-stokenet-gateway.radixdlt.com/"
            }
            "#,
        )
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Gateway::mainnet()),
            "Mainnet: https://mainnet.radixdlt.com/"
        );
    }

    #[test]
    fn identifiable() {
        assert_eq!(Gateway::mainnet().id(), Gateway::mainnet().url);
    }

    #[test]
    fn mainnet_is_default() {
        assert_eq!(Gateway::default(), Gateway::mainnet());
    }

    #[test]
    fn mainnet_is_wellknown() {
        assert_eq!(Gateway::mainnet().is_wellknown(), true);
    }

    #[test]
    fn stokenet_is_wellknown() {
        assert_eq!(Gateway::stokenet().is_wellknown(), true);
    }
}
