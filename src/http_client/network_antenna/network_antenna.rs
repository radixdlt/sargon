use crate::prelude::*;

/// Trait for executing network requests, to be  implemented on the FFI side
/// (iOS/Android), so that Sargon can with some HTTP client perform make network
/// requests.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkAntenna: Send + Sync {
    /// Method invoked by Sargon Rust side, telling an implementing type to
    /// execute a `NetworkRequest` and pass the Result back to Sargon Rust side.
    ///
    /// Either: `Err` or `Ok(NetworkResponse)`.
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, CommonError>;
}

/// A mocked network antenna, useful for testing.
#[derive(Debug)]
pub struct MockAntenna {
    hard_coded_status: u16,
    hard_coded_body: BagOfBytes,
    spy: fn(NetworkRequest) -> (),
}

#[allow(unused)]
impl MockAntenna {
    pub fn with_spy(
        status: u16,
        body: impl Into<BagOfBytes>,
        spy: fn(NetworkRequest) -> (),
    ) -> Self {
        Self {
            hard_coded_status: status,
            hard_coded_body: body.into(),
            spy,
        }
    }

    pub fn new(status: u16, body: impl Into<BagOfBytes>) -> Self {
        Self::with_spy(status, body, |_| {})
    }

    pub fn new_always_failling() -> Self {
        Self::new(500, BagOfBytes::new())
    }

    pub fn with_response<T>(response: T) -> Self
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(&response).unwrap();
        Self::new(200, body)
    }
}

#[async_trait::async_trait]
impl NetworkAntenna for MockAntenna {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        (self.spy)(request);
        Ok(NetworkResponse {
            status_code: self.hard_coded_status,
            body: self.hard_coded_body.clone(),
        })
    }
}
