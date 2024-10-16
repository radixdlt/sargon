use crate::prelude::*;
use sargon::ChangeGatewayOutcome as InternalChangeGatewayOutcome;
use sargon::Gateway as InternalGateway;
use sargon::Identifiable;

/// A gateway to some Radix Network, which is a high level REST API which clients (wallets) can
/// consume in order to query asset balances and submit transactions.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct Gateway {
    /// The Radix network the API is a Gateway to.
    pub network: NetworkDefinition,

    /// The URL to the gateways API endpoint
    pub url: Url,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum ChangeGatewayOutcome {
    /// If we did in fact change the gateway, and if the gateway was unknown
    /// or known before it was added, i.e. `is_new` will be true iff the gateway
    /// was unknown before changing to it.
    DidChange {
        /// If the Gateway we just switched to already was in the `other` list of
        /// saved gateways in AppPreferences, or if it was entirely new.
        is_new: bool,
    },

    /// We tried to change to the current gateway.
    NoChange,
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
    InternalGateway::from(network_id.into_internal()).into()
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
    InternalGateway::new(url, network_id.into()).into_result()
}

#[uniffi::export]
pub fn gateway_wellknown_gateways() -> Vec<Gateway> {
    InternalGateway::wellknown().into_type()
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
