use crate::prelude::*;
use sargon::Gateway as InternalGateway;

/// A gateway to some Radix Network, which is a high level REST API which clients (wallets) can
/// consume in order to query asset balances and submit transactions.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
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
    InternalGateway::wellknown().into_vec()
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

