#![cfg(test)]

use crate::prelude::*;

/// A mocked network antenna, useful for testing.
#[derive(Debug)]
pub struct MockNetworkingDriver {
    hard_coded_status: u16,
    hard_coded_body: BagOfBytes,
    spy: fn(NetworkRequest) -> (),
}

#[allow(unused)]
impl MockNetworkingDriver {
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

    pub fn new_always_failing() -> Self {
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
impl NetworkingDriver for MockNetworkingDriver {
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
