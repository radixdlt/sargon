use crate::prelude::*;

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

impl From<NetworkID> for u8 {
    fn from(value: NetworkID) -> Self {
        value.discriminant()
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

    /// Returns collection of all by Sargon known network ids.
    pub fn all() -> Vec<Self> {
        all::<Self>().collect()
    }
}

impl TryFrom<u8> for NetworkID {
    type Error = CommonError;

    /// Tries to instantiate a NetworkID from its raw representation `u8`.
    fn try_from(value: u8) -> Result<Self> {
        Self::from_repr(value)
            .ok_or(Self::Error::UnknownNetworkID { bad_value: value })
    }
}

impl std::fmt::Display for NetworkID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.logical_name())
    }
}

impl NetworkID {
    /// Looks up a `ScryptoNetworkDefinition` in lookup table,
    /// this is used internally for radix_common::address::AddressBech32Decoder,
    /// and to read out the canonical name (logical name) for a network.
    pub(crate) fn network_definition(&self) -> ScryptoNetworkDefinition {
        use NetworkID::*;
        match self {
            Mainnet => ScryptoNetworkDefinition::mainnet(),
            Stokenet => ScryptoNetworkDefinition::stokenet(),
            Adapanet => ScryptoNetworkDefinition::adapanet(),
            Nebunet => ScryptoNetworkDefinition::nebunet(),
            Kisharnet => ScryptoNetworkDefinition::kisharnet(),
            Ansharnet => ScryptoNetworkDefinition::ansharnet(),
            Zabanet => ScryptoNetworkDefinition::zabanet(),
            Enkinet => ScryptoNetworkDefinition {
                id: Enkinet.discriminant(),
                logical_name: String::from("enkinet"),
                hrp_suffix: String::from("tdx_21_"),
            },
            Hammunet => ScryptoNetworkDefinition {
                id: Hammunet.discriminant(),
                logical_name: String::from("hammunet"),
                hrp_suffix: String::from("tdx_22_"),
            },
            Nergalnet => ScryptoNetworkDefinition {
                id: Nebunet.discriminant(),
                logical_name: String::from("nergalnet"),
                hrp_suffix: String::from("tdx_24_"),
            },
            Mardunet => ScryptoNetworkDefinition {
                id: Mardunet.discriminant(),
                logical_name: String::from("mardunet"),
                hrp_suffix: String::from("tdx_24_"),
            },
            NetworkID::Simulator => ScryptoNetworkDefinition::simulator(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkID;

    #[test]
    fn mainnet_is_default() {
        assert_eq!(SUT::default(), SUT::Mainnet);
    }

    #[test]
    fn mainnet_logical_name_is_lowercase_mainnet() {
        assert_eq!(SUT::Mainnet.logical_name(), "mainnet");
    }

    #[test]
    fn mainnet_fmt() {
        assert_eq!(format!("{}", SUT::Mainnet), "mainnet");
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&SUT::Mainnet, json!(1));
        assert_json_value_fails::<SUT>(json!("1"));
    }

    #[test]
    fn into_u8() {
        assert_eq!(u8::from(SUT::Mainnet), 1);
    }

    #[test]
    fn from_repr() {
        assert_eq!(SUT::Mainnet, SUT::from_repr(0x01).unwrap());
        assert_eq!(SUT::Stokenet, SUT::from_repr(0x02).unwrap());
    }

    #[test]
    fn discriminant_mainnet() {
        assert_eq!(SUT::Mainnet.discriminant(), 0x01);
    }

    #[test]
    fn discriminant_stokenet() {
        assert_eq!(SUT::Stokenet.discriminant(), 0x02);
    }

    #[test]
    fn discriminant_simulator() {
        assert_eq!(SUT::Simulator.discriminant(), 242);
    }

    #[test]
    fn discriminant_adapanet() {
        assert_eq!(SUT::Adapanet.discriminant(), 0x0a);
    }

    #[test]
    fn discriminant_enkinet() {
        assert_eq!(SUT::Enkinet.discriminant(), 0x21);
    }

    #[test]
    fn discriminant_hammunet() {
        assert_eq!(SUT::Hammunet.discriminant(), 0x22);
    }

    #[test]
    fn discriminant_nebunet() {
        assert_eq!(SUT::Nebunet.discriminant(), 0x0b);
    }

    #[test]
    fn discriminant_kisharnet() {
        assert_eq!(SUT::Kisharnet.discriminant(), 0x0c);
    }

    #[test]
    fn discriminant_ansharnet() {
        assert_eq!(SUT::Ansharnet.discriminant(), 0x0d);
    }

    #[test]
    fn discriminant_zabanet() {
        assert_eq!(SUT::Zabanet.discriminant(), 0x0e);
    }

    #[test]
    fn no_mixup() {
        let ids = SUT::all();
        assert_eq!(
            BTreeSet::from_iter(ids.iter().map(|id| id.logical_name())).len(),
            ids.len()
        );
    }
    #[test]
    fn lookup_network_definition() {
        assert_eq!(
            SUT::Mainnet.network_definition().id,
            SUT::Mainnet.discriminant()
        )
    }

    #[test]
    fn lookup_network_definition_enkinet() {
        assert_eq!(
            SUT::Enkinet.network_definition().id,
            SUT::Enkinet.discriminant()
        )
    }

    #[test]
    fn logical_name() {
        assert_eq!(SUT::Mainnet.logical_name(), "mainnet");
        assert_eq!(SUT::Stokenet.logical_name(), "stokenet");
    }
}
