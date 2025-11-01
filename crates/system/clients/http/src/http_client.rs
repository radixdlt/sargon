use crate::prelude::*;

/// A `HttpClient` needs a "network antenna" to be able to execute the
/// network requests - which is a trait that clients implement on the FFI side (iOS/Android).
#[derive(Clone)]
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
