use std::fmt::Display;

use enum_iterator::Sequence;
use radix_engine_common::network::NetworkDefinition;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(
    Serialize,
    Deserialize,
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
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    pub fn description(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Stokenet => "Stokenet".to_string(),
        }
    }
}

impl TryFrom<u8> for NetworkID {
    type Error = crate::error::Error;

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
    pub fn network_definition(&self) -> NetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
        }
    }
}
