use crate::prelude::*;

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::sync::Arc;

impl From<NetworkMethod> for reqwest::Method {
    fn from(value: NetworkMethod) -> Self {
        match value {
            NetworkMethod::Post => reqwest::Method::POST,
            NetworkMethod::Get => reqwest::Method::GET,
            NetworkMethod::Head => reqwest::Method::HEAD,
        }
    }
}

/// A **Rust** networking driver using `reqwest`.
#[derive(Debug)]
pub struct RustNetworkingDriver {
    client: reqwest::Client,
}

impl RustNetworkingDriver {
    pub fn new() -> Arc<Self> {
        let reqwest_client = reqwest::Client::new();
        Arc::new(Self {
            client: reqwest_client,
        })
    }
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
            .request(reqwest::Method::from(request.method), request.url)
            .body(request.body.to_vec())
            .headers(headers)
            .build()
            .unwrap();

        let response = self.client.execute(request).await.map_err(|_| {
            CommonError::Unknown {
                error_message: "Failed to execute request".to_string(),
            }
        })?;

        let status_code = response.status().as_u16();
        let body_bytes =
            response.bytes().await.map_err(|_| CommonError::Unknown {
                error_message: "Failed reading response bytes".to_string(),
            })?;
        let body = BagOfBytes::from(body_bytes.to_vec());

        Ok(NetworkResponse { status_code, body })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_map_method() {
        let test = |m: NetworkMethod, exp: reqwest::Method| {
            assert_eq!(reqwest::Method::from(m), exp);
        };
        test(NetworkMethod::Post, reqwest::Method::POST);
        test(NetworkMethod::Get, reqwest::Method::GET);
        test(NetworkMethod::Head, reqwest::Method::HEAD);
    }
}
