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

    /// Adapanet (0x0a / 0d10
    Adapanet = 0x0a,

    /// Nebunet (0x0b / 0d11 )
    ///
    /// The first Betanet of Babylon
    Nebunet = 0x0b,

    /// Kisharnet (0x0c / 0d12)
    ///
    /// The first release candidate of Babylon (RCnet v1)
    Kisharnet = 0x0c,

    /// Ansharnet (0x0d / 0d13)
    ///
    /// The second release candidate of Babylon (RCnet v2)
    Ansharnet = 0x0d,

    /// Zabanet (0x0e / 0d14)
    ///
    /// The third release candidate of Babylon (RCnet v3)
    Zabanet = 0x0e,

    /// Simulator (0xf2 / 0d242)
    Simulator = 242,
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

    /// Name, most not be changed, i.e. cannot capitalized, is used
    /// by app to validate against Gateway
    pub fn logical_name(&self) -> String {
        self.network_definition().logical_name
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
        writeln!(f, "{}", self.logical_name())
    }
}

impl NetworkID {
    /// Looks up a `NetworkDefinition` in a lookup table.
    pub fn network_definition(&self) -> NetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
            NetworkID::Adapanet => NetworkDefinition::adapanet(),
            NetworkID::Nebunet => NetworkDefinition::nebunet(),
            NetworkID::Kisharnet => NetworkDefinition::kisharnet(),
            NetworkID::Ansharnet => NetworkDefinition::ansharnet(),
            NetworkID::Zabanet => NetworkDefinition::zabanet(),
            NetworkID::Simulator => NetworkDefinition::simulator(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::json::{assert_json_value_eq_after_roundtrip, assert_json_value_fails};
    use enum_iterator::all;
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

    #[test]
    fn no_mixup() {
        let ids = all::<NetworkID>().collect::<Vec<NetworkID>>();
        assert_eq!(
            BTreeSet::from_iter(ids.iter().map(|id| id.logical_name())).len(),
            ids.len()
        );
    }

    #[test]
    fn lookup_network_definition() {
        assert_eq!(
            NetworkID::Mainnet.network_definition().id,
            NetworkID::Mainnet.discriminant()
        )
    }

    #[test]
    fn logical_name() {
        assert_eq!(NetworkID::Mainnet.logical_name(), "mainnet")
    }
}
