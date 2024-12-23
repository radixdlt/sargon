use crate::prelude::*;
use serde_json::Value;

/// A `HttpClient` needs a "network antenna" to be able to execute the
/// network requests - which is a trait that clients implement on the FFI side (iOS/Android).
#[derive(Debug)]
pub struct HttpClient {
    /// An object implementing the `NetworkingDriver` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub driver: Arc<dyn NetworkingDriver>,
}

impl HttpClient {
    pub fn new(driver: Arc<dyn NetworkingDriver>) -> Self {
        Self { driver }
    }
}

impl HttpClient {
    pub async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<BagOfBytes, CommonError> {
        let response = self.driver.execute_network_request(request).await?;

        // Check for valid status code
        if !(200..=299).contains(&response.status_code) {
            return Err(CommonError::NetworkResponseBadCode {
                code: response.status_code,
            });
        }

        Ok(response.body)
    }
}

impl HttpClient {
    fn model_from_response<U>(
        &self,
        bytes: BagOfBytes,
    ) -> Result<U, CommonError>
    where
        U: for<'a> Deserialize<'a>,
    {
        serde_json::from_slice::<U>(&bytes).map_err(|_| {
            CommonError::NetworkResponseJSONDeserialize {
                into_type: type_name::<U>(),
            }
        })
    }

    pub async fn execute_request_with_decoding<U>(
        &self,
        request: NetworkRequest,
    ) -> Result<U>
    where
        U: for<'a> Deserialize<'a>,
    {
        let response = self.execute_network_request(request).await?;
        self.model_from_response(response)
    }

    pub async fn execute_request_with_map<U, V, F>(
        &self,
        request: NetworkRequest,
        map: F,
    ) -> Result<V, CommonError>
    where
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        let model = self.execute_request_with_decoding(request).await?;
        map(model)
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
        let mock_networking_driver = MockNetworkingDriver::new(200, BagOfBytes::new());
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
        let mock_networking_driver =
            MockNetworkingDriver::with_spy(200, (), |request, _| {
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
            });
        let sut =
            SUT::new(Arc::new(mock_networking_driver), NetworkID::Stokenet);
        let req = sut.current_epoch();
        drop(timeout(MAX, req).await.unwrap());
    }
}
