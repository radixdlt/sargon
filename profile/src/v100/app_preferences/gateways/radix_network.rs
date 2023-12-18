use serde::{Deserialize, Serialize};
use wallet_kit_common::network_id::NetworkID::{self, *};

/// A version of the Radix Network, for a NetworkID with an identifier (name) and display description (display name)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RadixNetwork {
    /// A String identifier (always lowercase) with the name of the Network that MUST match what Gateway returns.
    #[serde(rename = "name")]
    logical_name: String,

    /// The canonical identifier of this network.
    id: NetworkID,

    /// A name of the network intended for display purposes only.
    display_description: String,
}

impl RadixNetwork {
    fn declare(id: NetworkID, display: &str) -> Self {
        Self {
            logical_name: id.network_definition().logical_name,
            id,
            display_description: display.to_string(),
        }
    }
}

impl RadixNetwork {
    /// The Radix mainnet, the "real" Network on which all launched Dapps and
    /// assets with any real value resides.
    pub fn mainnet() -> Self {
        Self::declare(Mainnet, "Mainnet")
    }

    /// The primary public testnet of the Radix ecosystem, used by Dapp Developers
    /// and RDX Works alike to test new features.
    pub fn stokenet() -> Self {
        Self::declare(Stokenet, "Stokenet")
    }

    /// A Betanet.
    pub fn nebunet() -> Self {
        Self::declare(Nebunet, "Betanet")
    }

    /// Was a Release Candidate for Babylon launch.
    pub fn kisharnet() -> Self {
        Self::declare(Kisharnet, "RCnet")
    }

    /// Was the second Release Candidate for Babylon launch.
    pub fn ansharnet() -> Self {
        Self::declare(Ansharnet, "RCnet-V2 test network")
    }

    /// Was the third Release Candidate for Babylon launch.
    pub fn zabanet() -> Self {
        Self::declare(Zabanet, "RCnet-3 test network")
    }

    /// A testnet.
    pub fn hammunet() -> Self {
        Self::declare(Hammunet, "Hammunet (Test Network)")
    }

    /// A testnet.
    pub fn enkinet() -> Self {
        Self::declare(Enkinet, "Enkinet (Test Network)")
    }

    /// A testnet.
    pub fn nergalnet() -> Self {
        Self::declare(NetworkID::Nergalnet, "Nergalnet (Test Network)")
    }

    /// A testnet.
    pub fn mardunet() -> Self {
        Self::declare(NetworkID::Mardunet, "Mardunet (Test Network)")
    }
}

impl RadixNetwork {
    pub fn placeholder() -> Self {
        Self::mainnet()
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::RadixNetwork;

    #[test]
    fn json_roundtrip() {
        let sut = RadixNetwork::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "mainnet",
                "id": 1,
                "displayDescription": "Mainnet"
            }
            "#,
        )
    }
}
