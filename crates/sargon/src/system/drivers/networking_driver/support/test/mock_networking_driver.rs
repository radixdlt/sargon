#![cfg(test)]

use crate::prelude::*;
use hex::ToHex;
use std::sync::Mutex;

/// A mocked network antenna, useful for testing.
#[derive(Debug)]
pub struct MockNetworkingDriver {
    hard_coded_status: u16,
    hard_coded_bodies: Mutex<Vec<BagOfBytes>>,
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
            hard_coded_bodies: Mutex::new(vec![body.into()]),
            spy,
        }
    }

    pub fn with_spy_multiple_bodies(
        status: u16,
        bodies: Vec<BagOfBytes>,
        spy: fn(NetworkRequest) -> (),
    ) -> Self {
        Self {
            hard_coded_status: status,
            hard_coded_bodies: Mutex::new(bodies),
            spy,
        }
    }

    pub fn new(status: u16, body: impl Into<BagOfBytes>) -> Self {
        Self::with_spy(status, body, |_| {})
    }

    pub fn new_with_bodies(status: u16, bodies: Vec<BagOfBytes>) -> Self {
        Self::with_spy_multiple_bodies(status, bodies, |_| {})
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

    pub fn with_responses<T>(responses: Vec<T>) -> Self
    where
        T: Serialize,
    {
        let bodies = responses
            .into_iter()
            .map(|r| serde_json::to_vec(&r).unwrap())
            .map(BagOfBytes::from)
            .collect();

        Self {
            hard_coded_status: 200,
            hard_coded_bodies: Mutex::new(bodies),
            spy: |_| {},
        }
    }
}

#[async_trait::async_trait]
impl NetworkingDriver for MockNetworkingDriver {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        (self.spy)(request);
        let mut bodies = self.hard_coded_bodies.lock().unwrap();
        if bodies.is_empty() {
            Err(CommonError::Unknown)
        } else {
            Ok(NetworkResponse {
                status_code: self.hard_coded_status,
                body: bodies.remove(0),
            })
        }
    }
}
