use crate::prelude::*;
use radix_engine_common::network::NetworkDefinition as NativeNetworkDefinition;

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
    enum_iterator::Sequence,
    uniffi::Enum,
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

    /// Enkinet (0x21 / 0d33)
    ///
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L94
    Enkinet = 0x21,

    /// Hammunet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L95
    /// Decimal value: 34
    Hammunet = 0x22,

    /// Nergalnet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L96
    /// Decimal value: 35
    Nergalnet = 0x23,

    /// Mardunet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L97
    /// Decimal value: 36
    Mardunet = 0x24,

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
    type Error = CommonError;

    /// Tries to instantiate a NetworkID from its raw representation `u8`.
    fn try_from(value: u8) -> Result<Self> {
        Self::from_repr(value).ok_or(Self::Error::UnknownNetworkID(value))
    }
}

impl std::fmt::Display for NetworkID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.logical_name())
    }
}

impl NetworkID {
    /// Looks up a `NativeNetworkDefinition` in lookup table,
    /// this is used internally for radix_engine_common::address::AddressBech32Decoder,
    /// and to read out the canonical name (logical name) for a network.
    pub(crate) fn network_definition(&self) -> NativeNetworkDefinition {
        use NetworkID::*;
        match self {
            Mainnet => NativeNetworkDefinition::mainnet(),
            Stokenet => NativeNetworkDefinition::stokenet(),
            Adapanet => NativeNetworkDefinition::adapanet(),
            Nebunet => NativeNetworkDefinition::nebunet(),
            Kisharnet => NativeNetworkDefinition::kisharnet(),
            Ansharnet => NativeNetworkDefinition::ansharnet(),
            Zabanet => NativeNetworkDefinition::zabanet(),
            Enkinet => NativeNetworkDefinition {
                id: Enkinet.discriminant(),
                logical_name: String::from("enkinet"),
                hrp_suffix: String::from("tdx_21_"),
            },
            Hammunet => NativeNetworkDefinition {
                id: Hammunet.discriminant(),
                logical_name: String::from("hammunet"),
                hrp_suffix: String::from("tdx_22_"),
            },
            Nergalnet => NativeNetworkDefinition {
                id: Nebunet.discriminant(),
                logical_name: String::from("nergalnet"),
                hrp_suffix: String::from("tdx_24_"),
            },
            Mardunet => NativeNetworkDefinition {
                id: Mardunet.discriminant(),
                logical_name: String::from("mardunet"),
                hrp_suffix: String::from("tdx_24_"),
            },
            NetworkID::Simulator => NativeNetworkDefinition::simulator(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use enum_iterator::all;

    #[test]
    fn mainnet_is_default() {
        assert_eq!(NetworkID::default(), NetworkID::Mainnet);
    }

    #[test]
    fn mainnet_logical_name_is_lowercase_mainnet() {
        assert_eq!(NetworkID::Mainnet.logical_name(), "mainnet");
    }

    #[test]
    fn mainnet_fmt() {
        assert_eq!(format!("{}", NetworkID::Mainnet), "mainnet");
    }

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
    fn discriminant_mainnet() {
        assert_eq!(NetworkID::Mainnet.discriminant(), 0x01);
    }

    #[test]
    fn discriminant_stokenet() {
        assert_eq!(NetworkID::Stokenet.discriminant(), 0x02);
    }

    #[test]
    fn discriminant_simulator() {
        assert_eq!(NetworkID::Simulator.discriminant(), 242);
    }

    #[test]
    fn discriminant_adapanet() {
        assert_eq!(NetworkID::Adapanet.discriminant(), 0x0a);
    }

    #[test]
    fn discriminant_enkinet() {
        assert_eq!(NetworkID::Enkinet.discriminant(), 0x21);
    }

    #[test]
    fn discriminant_hammunet() {
        assert_eq!(NetworkID::Hammunet.discriminant(), 0x22);
    }

    #[test]
    fn discriminant_nebunet() {
        assert_eq!(NetworkID::Nebunet.discriminant(), 0x0b);
    }

    #[test]
    fn discriminant_kisharnet() {
        assert_eq!(NetworkID::Kisharnet.discriminant(), 0x0c);
    }

    #[test]
    fn discriminant_ansharnet() {
        assert_eq!(NetworkID::Ansharnet.discriminant(), 0x0d);
    }

    #[test]
    fn discriminant_zabanet() {
        assert_eq!(NetworkID::Zabanet.discriminant(), 0x0e);
    }

    #[test]
    fn no_mixup() {
        let ids = all::<NetworkID>().collect::<Vec<NetworkID>>();
        assert_eq!(
            BTreeSet::from_iter(ids.iter().map(|id| id.logical_name())).len(),
            ids.len()
        );
    }
    /*
    #[test]
    fn lookup_network_definition() {
        assert_eq!(
            NetworkID::Mainnet.network_definition().id,
            NetworkID::Mainnet.discriminant()
        )
    }

    #[test]
    fn lookup_network_definition_enkinet() {
        assert_eq!(
            NetworkID::Enkinet.network_definition().id,
            NetworkID::Enkinet.discriminant()
        )
    }
    */

    #[test]
    fn logical_name() {
        assert_eq!(NetworkID::Mainnet.logical_name(), "mainnet");
        assert_eq!(NetworkID::Stokenet.logical_name(), "stokenet");
    }
}
