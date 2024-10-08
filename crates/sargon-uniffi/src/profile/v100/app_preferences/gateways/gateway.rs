use crate::prelude::*;
use sargon::Gateway as InternalGateway;

/// A gateway to some Radix Network, which is a high level REST API which clients (wallets) can
/// consume in order to query asset balances and submit transactions.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    uniffi::Record,
)]
pub struct Gateway {
    /// The Radix network the API is a Gateway to.
    pub network: NetworkDefinition,

    /// The URL to the gateways API endpoint
    pub url: Url,
}

impl From<InternalGateway> for Gateway {
    fn from(value: InternalGateway) -> Self {
        Self {
            network: value.network.into(),
            url: value.url.into(),
        }
    }
}

impl Into<InternalGateway> for Gateway {
    fn into(self) -> InternalGateway {
        InternalGateway {
            network: self.network.into(),
            url: self.url.into(),
        }
    }
}

#[uniffi::export]
pub fn new_gateway_sample() -> Gateway {
    InternalGateway::sample().into()
}

#[uniffi::export]
pub fn new_gateway_sample_other() -> Gateway {
    InternalGateway::sample_other().into()
}

#[uniffi::export]
pub fn new_gateway_for_network_id(network_id: NetworkID) -> Gateway {
    InternalGateway::from(network_id.into()).into()
}

#[uniffi::export]
pub fn gateway_mainnet() -> Gateway {
    InternalGateway::mainnet().into()
}

#[uniffi::export]
pub fn gateway_stokenet() -> Gateway {
    InternalGateway::stokenet().into()
}

#[uniffi::export]
pub fn new_gateway_with_url_on_network(
    url: String,
    network_id: NetworkID,
) -> Result<Gateway> {
    InternalGateway::new(url, network_id.into()).map_result()
}

#[uniffi::export]
pub fn gateway_wellknown_gateways() -> Gateways {
    InternalGateway::wellknown().into()
}

#[uniffi::export]
pub fn gateway_is_wellknown(gateway: &Gateway) -> bool {
    gateway.into_internal().is_wellknown()
}

#[uniffi::export]
pub fn gateway_to_string(gateway: &Gateway) -> String {
    gateway.into_internal().to_string()
}

#[uniffi::export]
pub fn gateway_id(gateway: &Gateway) -> Url {
    gateway.into_internal().id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Gateway;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_gateway_sample(),
                new_gateway_sample_other(),
                // duplicates should get removed
                new_gateway_sample(),
                new_gateway_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        let sut = SUT::sample();
        assert_eq!(gateway_to_string(&sut), sut.to_string());
    }

    #[test]
    fn test_id() {
        let sut = SUT::sample();
        assert_eq!(gateway_id(&sut), sut.id());
    }

    #[test]
    fn test_gateway_mainnet() {
        assert_eq!(gateway_mainnet(), SUT::mainnet());
    }

    #[test]
    fn test_gateway_stokenet() {
        assert_eq!(gateway_stokenet(), SUT::stokenet());
    }

    #[test]
    fn test_new_gateway_with_url_on_network() {
        assert_eq!(
            new_gateway_with_url_on_network(
                "https://mainnet.radixdlt.com/".to_owned(),
                NetworkID::Mainnet
            )
            .unwrap(),
            SUT::mainnet()
        );
    }

    #[test]
    fn test_gateway_wellknown_gateways() {
        assert_eq!(gateway_wellknown_gateways(), SUT::wellknown())
    }

    #[test]
    fn test_gateway_is_wellknown() {
        assert!(gateway_is_wellknown(&SUT::sample()))
    }

    #[test]
    fn test_new_gateway_for_network_id() {
        assert_eq!(
            new_gateway_for_network_id(NetworkID::Mainnet),
            SUT::mainnet()
        )
    }
}
