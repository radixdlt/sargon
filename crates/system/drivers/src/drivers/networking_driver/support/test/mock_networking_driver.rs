use serde::{Deserialize, Serialize};

use crate::prelude::*;
use std::sync::Mutex;

/// A mocked network antenna, useful for testing.
pub struct MockNetworkingDriver {
    hard_coded_responses: Mutex<Vec<MockNetworkingDriverResponse>>,
    // `MockNetworkingDriver` will try to use a hard_coded_response first, and if it doesn't have one, it will use the lazy_responder.
    lazy_responder: Option<MockNetworkingDriverLazyResponder>,
    spy: fn(NetworkRequest, u64) -> (),
    count: Mutex<u64>,
}

#[derive(Debug, Clone)]
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
        lazy_responder: impl Into<Option<MockNetworkingDriverLazyResponder>>,
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
            lazy_responder: lazy_responder.into(),
        }
    }

    pub fn with_spy(
        status: u16,
        body: impl Into<BagOfBytes>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        Self::_new(status, vec![body.into()], spy, None)
    }

    pub fn with_spy_multiple_bodies(
        status: u16,
        bodies: Vec<BagOfBytes>,
        spy: fn(NetworkRequest, u64) -> (),
    ) -> Self {
        Self::_new(status, bodies, spy, None)
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
            lazy_responder: None,
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

        Self::_new(200, bodies, |_, _| {}, None)
    }

    pub fn with_lazy_responses(
        provide_lazy: impl Fn(NetworkRequest, u64) -> NetworkResponse
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self::_new(
            200,
            vec![],
            |_, _| {},
            MockNetworkingDriverLazyResponder::new_with_responses(provide_lazy),
        )
    }

    /// N.B. the inverse of Serialize/Deserialize!
    pub fn with_lazy_response<Req, Resp>(
        provide_lazy: impl Fn(Req, u64) -> Resp + Send + Sync + 'static,
    ) -> Self
    where
        Resp: Serialize,
        Req: for<'a> Deserialize<'a>,
    {
        Self::_new(
            200,
            vec![],
            |_, _| {},
            MockNetworkingDriverLazyResponder::new_with_response(provide_lazy),
        )
    }
}

pub struct MockNetworkingDriverLazyResponder {
    provide_response_for: Arc<
        dyn Fn(NetworkRequest, u64) -> NetworkResponse + Send + Sync + 'static,
    >,
}

impl MockNetworkingDriverLazyResponder {
    fn new_with_responses(
        provide: impl Fn(NetworkRequest, u64) -> NetworkResponse
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            provide_response_for: Arc::new(provide),
        }
    }
    /// N.B. the inverse of Serialize/Deserialize!
    fn new_with_response<Req, Resp>(
        provide: impl Fn(Req, u64) -> Resp + Send + Sync + 'static,
    ) -> Self
    where
        Resp: Serialize,
        Req: for<'a> Deserialize<'a>,
    {
        Self::new_with_responses(move |req, count| {
            println!("🔮MOCK DRIVER got req: {:?}", req);
            let s = String::from_utf8_lossy(&req.body);
            println!("🔮MOCK DRIVER RAW json: {:?}", s);
            let req: Req = serde_json::from_slice(&req.body).unwrap();
            let resp: Resp = provide(req, count);
            let resp_body = serde_json::to_vec(&resp).unwrap();
            NetworkResponse::new(200, resp_body)
        })
    }

    fn provide_response_for(
        &self,
        request: NetworkRequest,
        count: u64,
    ) -> NetworkResponse {
        (self.provide_response_for)(request, count)
    }
}

#[async_trait::async_trait]
impl NetworkingDriver for MockNetworkingDriver {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        let mut count = self.count.lock().unwrap();
        (self.spy)(request.clone(), *count);
        let mut responses = self.hard_coded_responses.lock().unwrap();
        let res = if responses.is_empty() {
            if let Some(lazy_responder) = self.lazy_responder.as_ref() {
                Ok(lazy_responder.provide_response_for(request.clone(), *count))
            } else {
                Err(CommonError::Unknown)
            }
        } else {
            let response = responses.remove(0);
            Ok(NetworkResponse {
                status_code: response.status,
                body: response.body,
            })
        };
        *count += 1;
        res
    }
}
