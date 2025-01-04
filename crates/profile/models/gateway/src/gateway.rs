use sargon_core_misc::parse_url;

use crate::prelude::*;

/// A gateway to some Radix Network, which is a high level REST API which clients (wallets) can
/// consume in order to query asset balances and submit transactions.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.url.to_string())]
#[debug("{}: {}", self.network.display_description, self.url.to_string())]
pub struct Gateway {
    /// The Radix network the API is a Gateway to.
    pub network: NetworkDefinition,

    /// The URL to the gateways API endpoint
    pub url: Url,
}

impl Identifiable for Gateway {
    type ID = Url;

    fn id(&self) -> Self::ID {
        self.url.clone()
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::mainnet()
    }
}

impl From<NetworkID> for Gateway {
    fn from(value: NetworkID) -> Self {
        match value {
            NetworkID::Mainnet => Self::mainnet(),
            NetworkID::Stokenet => Self::stokenet(),
            NetworkID::Nebunet => Self::nebunet(),
            NetworkID::Kisharnet => Self::kisharnet(),
            NetworkID::Ansharnet => Self::ansharnet(),
            NetworkID::Enkinet => Self::enkinet(),
            NetworkID::Hammunet => Self::hammunet(),
            NetworkID::Mardunet => Self::mardunet(),
            NetworkID::Adapanet => panic!("No network exists for {}", value),
            NetworkID::Zabanet => panic!("No network exists for {}", value),
            NetworkID::Nergalnet => panic!("No network exists for {}", value),
            NetworkID::Simulator => panic!("No network exists for {}", value),
        }
    }
}

impl Gateway {
    pub fn new(url: String, id: NetworkID) -> Result<Self> {
        let url = parse_url(url)?;
        let network = NetworkDefinition::lookup_by_id(id)?;
        Ok(Self { url, network })
    }
}

impl Gateway {
    pub fn declare(url: &str, id: NetworkID) -> Self {
        Self::new(url.to_string(), id).expect("Valid").clone()
    }
}

impl HasSampleValues for Gateway {
    fn sample() -> Self {
        Gateway::mainnet()
    }

    fn sample_other() -> Self {
        Gateway::stokenet()
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
        Self::declare(
            "https://enkinet-gateway.radixdlt.com/",
            NetworkID::Enkinet,
        )
    }

    pub fn mardunet() -> Self {
        Self::declare(
            "https://mardunet-gateway.radixdlt.com/",
            NetworkID::Mardunet,
        )
    }
}

impl Gateway {
    pub fn wellknown() -> Gateways {
        Gateways::from_iter([Self::mainnet(), Self::stokenet()])
    }

    pub fn is_wellknown(&self) -> bool {
        Self::wellknown().contains_by_id(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Gateway;

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = SUT::mainnet();
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
        let sut = SUT::stokenet();
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
            format!("{}", SUT::mainnet()),
            "https://mainnet.radixdlt.com/"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::mainnet()),
            "Mainnet: https://mainnet.radixdlt.com/"
        );
    }

    #[test]
    fn identifiable() {
        assert_eq!(SUT::mainnet().id(), SUT::mainnet().url);
    }

    #[test]
    fn mainnet_is_default() {
        assert_eq!(SUT::default(), SUT::mainnet());
    }

    #[test]
    fn mainnet_is_wellknown() {
        assert!(SUT::mainnet().is_wellknown());
    }

    #[test]
    fn stokenet_is_wellknown() {
        assert!(SUT::stokenet().is_wellknown());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Gateway>::from_iter([
                SUT::mainnet(),
                SUT::stokenet(),
                SUT::rcnet(),
                SUT::nebunet(),
                SUT::kisharnet(),
                SUT::ansharnet(),
                SUT::hammunet(),
                SUT::enkinet(),
                SUT::mardunet(),
                // Twice
                SUT::mainnet(),
                SUT::stokenet(),
                SUT::rcnet(),
                SUT::nebunet(),
                SUT::kisharnet(),
                SUT::ansharnet(),
                SUT::hammunet(),
                SUT::enkinet(),
                SUT::mardunet(),
            ])
            .len(),
            9
        );
    }

    #[test]
    fn from_network_id() {
        let ids = HashSet::<NetworkID>::from_iter(NetworkID::all());
        let unsupported = HashSet::<NetworkID>::from_iter([
            NetworkID::Adapanet,
            NetworkID::Zabanet,
            NetworkID::Nergalnet,
            NetworkID::Simulator,
        ]);
        ids.difference(&unsupported).for_each(|n| {
            let sut = SUT::from(*n);
            assert_eq!(sut.network.id, *n);
        })
    }

    #[test]
    #[should_panic(expected = "No network exists for adapanet")]
    fn from_network_id_unsupported_adapanet() {
        _ = SUT::from(NetworkID::Adapanet);
    }

    #[test]
    #[should_panic(expected = "No network exists for zabanet")]
    fn from_network_id_unsupported_zabanet() {
        _ = SUT::from(NetworkID::Zabanet);
    }

    #[test]
    #[should_panic(expected = "No network exists for nergalnet")]
    fn from_network_id_unsupported_nergalnet() {
        _ = SUT::from(NetworkID::Nergalnet);
    }

    #[test]
    #[should_panic(expected = "No network exists for simulator")]
    fn from_network_id_unsupported_simulator() {
        _ = SUT::from(NetworkID::Simulator);
    }
}
