use crate::prelude::*;

///
/// Internal
///
impl GatewayClient {
    /// Dispatches an HTTP `POST` request by JSON serializing the specified
    /// `request` and setting it as the `body` for the network request.
    pub async fn post<T, U, V, F>(
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
    pub async fn post_empty<U, V, F>(
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
pub const fn res_id<T>(x: T) -> Result<T, CommonError> {
    std::convert::identity::<Result<T, CommonError>>(Ok(x))
}
