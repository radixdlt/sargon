use crate::prelude::*;
use std::sync::Mutex;

/// A mocked network antenna, useful for testing.
#[derive(Debug)]
pub struct MockNetworkingDriver {
    hard_coded_responses: Mutex<Vec<MockNetworkingDriverResponse>>,
    spy: fn(NetworkRequest, u64) -> (),
    count: Mutex<u64>,
}

#[derive(Debug)]
pub struct MockNetworkingDriverResponse {
    status: u16,
    body: BagOfBytes,
}

impl MockNetworkingDriverResponse {
    pub fn new_failing() -> Self {
        Self {
            status: 500,
            body: BagOfBytes::new(),
        }
    }

    pub fn new_success<T>(body: T) -> Self
    where
        T: Serialize,
    {
        let vec = serde_json::to_vec(&body).unwrap();
        let body = BagOfBytes::from(vec);
        Self { status: 200, body }
    }
}

#[allow(unused)]
impl MockNetworkingDriver {
    fn _new(
        hard_coded_status: u16,
        hard_coded_bodies: Vec<BagOfBytes>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        let responses = hard_coded_bodies
            .into_iter()
            .map(|body| MockNetworkingDriverResponse {
                status: hard_coded_status,
                body,
            })
            .collect();
        Self {
            hard_coded_responses: Mutex::new(responses),
            spy,
            count: Mutex::new(0),
        }
    }

    pub fn with_spy(
        status: u16,
        body: impl Into<BagOfBytes>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        Self::_new(status, vec![body.into()], spy)
    }

    pub fn with_spy_multiple_bodies(
        status: u16,
        bodies: Vec<BagOfBytes>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        Self::_new(status, bodies, spy)
    }

    pub fn new(status: u16, body: impl Into<BagOfBytes>) -> Self {
        Self::with_spy(status, body, |_, _| {})
    }

    pub fn new_with_bodies(status: u16, bodies: Vec<BagOfBytes>) -> Self {
        Self::with_spy_multiple_bodies(status, bodies, |_, _| {})
    }

    pub fn new_with_responses(
        responses: Vec<MockNetworkingDriverResponse>,
    ) -> Self {
        Self::new_with_responses_and_spy(responses, |_, _| {})
    }

    pub fn new_with_responses_and_spy(
        responses: Vec<MockNetworkingDriverResponse>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        Self {
            hard_coded_responses: Mutex::new(responses),
            spy,
            count: Mutex::new(0),
        }
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

        Self::_new(200, bodies, |_, _| {})
    }
}

#[async_trait::async_trait]
impl NetworkingDriver for MockNetworkingDriver {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        let mut count = self.count.lock().unwrap();
        (self.spy)(request, *count);
        *count += 1;
        let mut responses = self.hard_coded_responses.lock().unwrap();
        if responses.is_empty() {
            Err(CommonError::Unknown)
        } else {
            let response = responses.remove(0);
            Ok(NetworkResponse {
                status_code: response.status,
                body: response.body,
            })
        }
    }
}
