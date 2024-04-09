use crate::prelude::*;

/// An HTTP client for consuming the Radix ⛩️ Gateway API ([docs]).
///
/// A `GatewayClient` needs a "network antenna" to be able to execute the
/// network requests - which is a trait that clients implement on the FFI side
/// (iOS/Android) an "installs" when initiating an instance of the `GatewayClient`.
///
/// The implementing FFI clients can then consume the Radix Gateway API to e.g.
/// fetch the XRD balance of an account address or submit a signed transaction.
///
/// [docs]: https://radix-babylon-gateway-api.redoc.ly/
#[derive(uniffi::Object)]
pub struct GatewayClient {
    /// An object implementing the `NetworkAntenna` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub network_antenna: Arc<dyn NetworkAntenna>,

    /// The gateway this GatewayClient talks to, which is a (URL, NetworkID) tuple
    /// essentially.
    pub gateway: Gateway,
}

#[uniffi::export]
impl GatewayClient {
    /// Constructs a new `GatewayClient` with a NetworkAntenna for a specified
    /// `Gateway`.
    #[uniffi::constructor]
    pub fn with_gateway(
        network_antenna: Arc<dyn NetworkAntenna>,
        gateway: Gateway,
    ) -> Self {
        Self {
            network_antenna,
            gateway,
        }
    }

    /// Constructs a new `GatewayClient` with a NetworkAntenna for a specified
    /// network, by looking up an Radix DLT provided Gateway on that network.
    ///
    /// # Panics
    /// Panics if Radix DLT does not provide a Gateway for the specified
    /// `network_id` - e.g. will panic if you specify `NetworkID::Simulator` (duh).
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        network_id: NetworkID,
    ) -> Self {
        Self::with_gateway(network_antenna, Gateway::from(network_id))
    }
}

/// A mocked network antenna, useful for testing.
#[derive(Debug)]
struct MockAntenna {
    hard_coded_status: u16,
    hard_coded_body: BagOfBytes,
}

#[async_trait::async_trait]
impl NetworkAntenna for MockAntenna {
    async fn execute_network_request(
        &self,
        _request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        Ok(NetworkResponse {
            status_code: self.hard_coded_status,
            body: self.hard_coded_body.clone(),
        })
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
    async fn execute_network_request_invalid_url() {
        let mock_antenna = MockAntenna {
            hard_coded_status: 200,
            hard_coded_body: BagOfBytes::new(),
        };
        let base = "http://example.com";
        let sut = SUT::with_gateway(
            Arc::new(mock_antenna),
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
        let mock_antenna = MockAntenna {
            hard_coded_status: 404, // bad status code
            hard_coded_body: BagOfBytes::new(),
        };
        let sut = SUT::new(Arc::new(mock_antenna), NetworkID::Stokenet);
        let req = sut.current_epoch();
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(result, Err(CommonError::NetworkResponseBadCode))
    }

    #[actix_rt::test]
    async fn execute_network_request_empty_body() {
        let mock_antenna = MockAntenna {
            hard_coded_status: 200,
            hard_coded_body: BagOfBytes::new(), // empty body
        };
        let sut = SUT::new(Arc::new(mock_antenna), NetworkID::Stokenet);
        let req = sut.current_epoch();
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(result, Err(CommonError::NetworkResponseEmptyBody))
    }

    #[actix_rt::test]
    async fn execute_network_request_invalid_json() {
        let mock_antenna = MockAntenna {
            hard_coded_status: 200,
            hard_coded_body: "deadbeef".parse().unwrap(), // wrong JSON
        };
        let sut = SUT::new(Arc::new(mock_antenna), NetworkID::Stokenet);
        let req = sut.current_epoch();
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(result, Err(CommonError::NetworkResponseJSONDeserialize { into_type: "sargon::gateway_api::models::types::response::transaction::construction::transaction_construction_response::TransactionConstructionResponse".to_owned() }))
    }
}
