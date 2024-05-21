use crate::prelude::*;

/// An HTTP client for consuming the Radix ⛩️ Gateway API ([docs]).
///
/// The implementing FFI clients can then consume the Radix Gateway API to e.g.
/// fetch the XRD balance of an account address or submit a signed transaction.
///
/// [docs]: https://radix-babylon-gateway-api.redoc.ly/
#[derive(uniffi::Object)]
pub struct GatewayClient {
    /// The HTTP client that actually executes the network requests.
    pub http_client: HttpClient,

    /// The gateway this GatewayClient talks to, which is a (URL, NetworkID) tuple
    /// essentially.
    pub gateway: Gateway,
}

#[uniffi::export]
impl GatewayClient {
    /// Constructs a new `GatewayClient` with a NetworkingDriver for a specified
    /// `Gateway`.
    #[uniffi::constructor]
    pub fn with_gateway(
        networking_driver: Arc<dyn NetworkingDriver>,
        gateway: Gateway,
    ) -> Self {
        Self {
            http_client: HttpClient {
                driver: networking_driver,
            },
            gateway,
        }
    }

    /// Constructs a new `GatewayClient` with a NetworkingDriver for a specified
    /// network, by looking up an Radix DLT provided Gateway on that network.
    ///
    /// # Panics
    /// Panics if Radix DLT does not provide a Gateway for the specified
    /// `network_id` - e.g. will panic if you specify `NetworkID::Simulator` (duh).
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
    ) -> Self {
        Self::with_gateway(networking_driver, Gateway::from(network_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use reqwest::Response;
    use std::time::Duration;

    const MAX: Duration = Duration::from_millis(10);

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn test_submit_notarized_transaction_mock_duplicate() {
        let mock_antenna =
            MockAntenna::with_response(TransactionSubmitResponse {
                duplicate: true,
            });
        let sut = SUT::new(Arc::new(mock_antenna), NetworkID::Stokenet);
        let req =
            sut.submit_notarized_transaction(NotarizedTransaction::sample());
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(result, Err(CommonError::GatewaySubmitDuplicateTX { intent_hash: "txid_rdx198k527d5wt4ms5tvrdcu8089v4hptp7ztv388k539uzzvmw25ltsj7u4zz".to_owned() }));
    }
}
