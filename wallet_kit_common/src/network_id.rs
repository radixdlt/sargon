use std::fmt::Display;

use enum_iterator::Sequence;
use radix_engine_common::network::NetworkDefinition;
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Sequence,
)]
#[repr(u8)]
pub enum NetworkID {
    /// Mainnet (0x01 / 0d01)
    ///
    /// The Radix public network.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L79
    Mainnet = 0x01,

    /// Stokenet (0x02 / 0d02)
    ///
    /// The public testnet for Radix.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L71
    Stokenet = 0x02,
}

impl Default for NetworkID {
    fn default() -> Self {
        Self::Mainnet
    }
}

impl NetworkID {
    /// The raw representation of this network id, an `u8`.
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    /// The name of the network.
    pub fn description(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Stokenet => "Stokenet".to_string(),
        }
    }
}

impl TryFrom<u8> for NetworkID {
    type Error = crate::error::Error;

    /// Tries to instantiate a NetworkID from its raw representation `u8`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(Self::Error::UnknownNetworkID(value))
    }
}

impl Display for NetworkID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.description())
    }
}

impl NetworkID {
    /// Looks up a `NetworkDefinition` in a lookup table.
    pub fn network_definition(&self) -> NetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json::{assert_json_value_eq_after_roundtrip, assert_json_value_fails};
    use serde_json::json;

    use super::NetworkID;

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&NetworkID::Mainnet, json!(1));
        assert_json_value_fails::<NetworkID>(json!("1"));
    }

    #[test]
    fn from_repr() {
        assert_eq!(NetworkID::Mainnet, NetworkID::from_repr(0x01).unwrap());
        assert_eq!(NetworkID::Stokenet, NetworkID::from_repr(0x02).unwrap());
    }

    #[test]
    fn discriminant() {
        assert_eq!(NetworkID::Mainnet.discriminant(), 0x01);
        assert_eq!(NetworkID::Stokenet.discriminant(), 0x02);
    }
}
