mod poly_manifest_builder;
mod profile_view;
mod transaction_intent_builder;
mod xrd_balances_fetcher;

pub use poly_manifest_builder::*;
pub(crate) use profile_view::*;
pub use transaction_intent_builder::*;
pub use xrd_balances_fetcher::*;

use crate::prelude::*;

pub(crate) fn gateway_client_for_network(
    networking_driver: Arc<dyn NetworkingDriver>,
    gateways: &SavedGateways,
    network_id: NetworkID,
) -> GatewayClient {
    let gateway = gateways
        .all()
        .into_iter()
        .find(|gateway| gateway.network.id == network_id)
        .unwrap_or_else(|| Gateway::from(network_id));

    GatewayClient::with_gateway(networking_driver, gateway)
}
