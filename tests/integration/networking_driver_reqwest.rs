use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sargon::prelude::*;
use std::sync::Arc;

fn map_method(method: NetworkMethod) -> reqwest::Method {
    match method {
        NetworkMethod::Post => reqwest::Method::POST,
        NetworkMethod::Get => reqwest::Method::GET,
    }
}

#[derive(Debug)]
struct RustNetworkingDriver {
    client: reqwest::Client,
}

#[async_trait]
impl NetworkingDriver for RustNetworkingDriver {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse> {
        let mut headers = HeaderMap::new();
        for (key, value) in request.headers.iter() {
            headers.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(value.as_bytes()).unwrap(),
            );
        }
        let request = self
            .client
            .request(map_method(request.method), request.url)
            .body(request.body.to_vec())
            .headers(headers)
            .build()
            .unwrap();

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|_| CommonError::Unknown)?;

        let status_code = response.status().as_u16();
        let body_bytes =
            response.bytes().await.map_err(|_| CommonError::Unknown)?;
        let body = BagOfBytes::from(body_bytes.to_vec());

        Ok(NetworkResponse { status_code, body })
    }
}

pub(crate) fn new_gateway_client(network_id: NetworkID) -> GatewayClient {
    let reqwest_client = reqwest::Client::new();
    let network_antenna = RustNetworkingDriver {
        client: reqwest_client,
    };
    GatewayClient::new(Arc::new(network_antenna), network_id)
}
