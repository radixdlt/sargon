use profile_gateway::prelude::Gateway;

use crate::prelude::*;

/// An HTTP client for consuming the Radix ⛩️ Gateway API ([docs]).
///
/// The implementing FFI clients can then consume the Radix Gateway API to e.g.
/// fetch the XRD balance of an account address or submit a signed transaction.
///
/// [docs]: https://radix-babylon-gateway-api.redoc.ly/
pub struct GatewayClient {
    /// The HTTP client that actually executes the network requests.
    pub http_client: HttpClient,

    /// The gateway this GatewayClient talks to, which is a (URL, NetworkID) tuple
    /// essentially.
    pub gateway: Gateway,
}

impl GatewayClient {
    /// Constructs a new `GatewayClient` with a NetworkingDriver for a specified
    /// `Gateway`.
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

    /// Constructs a new `GatewayClient` with a `HttpClient` for a specified network
    pub fn with_http_client(
        http_client: HttpClient,
        network_id: NetworkID,
    ) -> Self {
        Self {
            http_client,
            gateway: Gateway::from(network_id),
        }
    }

    /// Constructs a new `GatewayClient` with a NetworkingDriver for a specified
    /// network, by looking up an Radix DLT provided Gateway on that network.
    ///
    /// # Panics
    /// Panics if Radix DLT does not provide a Gateway for the specified
    /// `network_id` - e.g. will panic if you specify `NetworkID::Simulator` (duh).
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
    use std::time::Duration;

    const MAX: Duration = Duration::from_millis(10);

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn test_submit_notarized_transaction_mock_duplicate() {
        let mock_networking_driver =
            MockNetworkingDriver::with_response(TransactionSubmitResponse {
                duplicate: true,
            });
        let sut =
            SUT::new(Arc::new(mock_networking_driver), NetworkID::Stokenet);
        let req =
            sut.submit_notarized_transaction(NotarizedTransaction::sample());
        let result = timeout(MAX, req).await.unwrap();

        assert_eq!(
            result,
            Err(
                CommonError::GatewaySubmitDuplicateTX {
                    intent_hash: r#"txid_rdx198k527d5wt4ms5tvrdcu8089v4hptp7ztv388k539uzzvmw25ltsj7u4zz"#.to_owned() }
                )
            );
    }

    #[actix_rt::test]
    async fn execute_network_request_invalid_url() {
        let mock_networking_driver =
            MockNetworkingDriver::new(200, BagOfBytes::new());
        let base = "http://example.com";
        let sut = SUT::with_gateway(
            Arc::new(mock_networking_driver),
            Gateway::declare(base, NetworkID::Stokenet),
        );
        let bad_path = "https://exa%23mple.org";
        let bad_value = format!("{}/{}", base, bad_path);
        let req = sut.post_empty::<i8, i8, _>(bad_path, res_id);
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(
            result,
            Err(CommonError::NetworkRequestInvalidUrl { bad_value })
        )
    }

    #[actix_rt::test]
    async fn execute_network_request_bad_status_code() {
        let mock_networking_driver = MockNetworkingDriver::new(
            404, // bad code
            BagOfBytes::new(),
        );
        let sut =
            SUT::new(Arc::new(mock_networking_driver), NetworkID::Stokenet);
        let req = sut.current_epoch();
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(
            result,
            Err(CommonError::NetworkResponseBadCode { code: 404 })
        )
    }

    #[actix_rt::test]
    async fn execute_network_request_invalid_json() {
        let mock_networking_driver = MockNetworkingDriver::new(
            200,
            BagOfBytes::sample_aced(), // wrong JSON
        );
        let sut =
            SUT::new(Arc::new(mock_networking_driver), NetworkID::Stokenet);
        let req = sut.current_epoch();
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(
            result,
            Err(CommonError::NetworkResponseJSONDeserialize {
                into_type: "TransactionConstructionResponse".to_owned()
            })
        )
    }

    #[actix_rt::test]
    async fn spy_headers() {
        let mock_networking_driver = MockNetworkingDriver::with_spy(
            200,
            BagOfBytes::new(),
            |request, _| {
                assert_eq!(
                    request
                        .headers
                        .keys()
                        .map(|v| v.to_string())
                        .collect::<BTreeSet<String>>(),
                    [
                        "RDX-Client-Version",
                        "RDX-Client-Name",
                        "accept",
                        "content-Type",
                        "user-agent"
                    ]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect::<BTreeSet<String>>()
                )
            },
        );
        let sut =
            SUT::new(Arc::new(mock_networking_driver), NetworkID::Stokenet);
        let req = sut.current_epoch();
        drop(timeout(MAX, req).await.unwrap());
    }
}
