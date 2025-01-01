use crate::prelude::*;

///
/// Internal
///
impl GatewayClient {
    /// Dispatches an HTTP `POST` request by JSON serializing the specified
    /// `request` and setting it as the `body` for the network request.
    pub(crate) async fn post<T, U, V, F>(
        &self,
        path: impl AsRef<str>,
        request: T,
        map: F,
    ) -> Result<V, CommonError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        // Append relative path to base url
        let path = path.as_ref();
        let url = self.gateway.url.join(path).map_err(|e| {
            let bad_value = format!("{}{}", self.gateway.url, path);
            error!(
                "Failed to parse URL, error: {:?}, from string: {}",
                e, &bad_value
            );
            CommonError::NetworkRequestInvalidUrl { bad_value }
        })?;

        let request = NetworkRequest::new_post(url)
            .with_gateway_api_headers()
            .with_serializing_body(request)?;

        self.http_client
            .execute_request_with_map(request, map)
            .await
    }

    /// Dispatches an HTTP `POST` request without any `body`.
    pub(crate) async fn post_empty<U, V, F>(
        &self,
        path: impl AsRef<str>,
        map: F,
    ) -> Result<V, CommonError>
    where
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        #[derive(Serialize)]
        struct EmptyBodyPostRequest {}
        self.post(path, EmptyBodyPostRequest {}, map).await
    }
}

/// An identity mapping function for Result
pub(crate) const fn res_id<T>(x: T) -> Result<T, CommonError> {
    std::convert::identity::<Result<T, CommonError>>(Ok(x))
}

pub trait NetworkRequestWithGatewayAndApiHeaders {
    fn with_gateway_api_headers(self) -> NetworkRequest;
}

impl NetworkRequestWithGatewayAndApiHeaders for NetworkRequest {
    fn with_gateway_api_headers(self) -> NetworkRequest {
        let headers = HashMap::<String, String>::from_iter([
            ("content-Type".to_owned(), "application/json".to_owned()),
            ("accept".to_owned(), "application/json".to_owned()),
            ("user-agent".to_owned(), "Sargon".to_owned()), // https://stackoverflow.com/a/77866494/1311272
            ("RDX-Client-Name".to_owned(), "Sargon".to_owned()),
            ("RDX-Client-Version".to_owned(), "1.5.1".to_owned()),
        ]);

        self.with_headers(headers)
    }
}
