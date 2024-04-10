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
        self.dispatch_network_request(path, NetworkMethod::Post, request, map)
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

///
/// Private
///
impl GatewayClient {
    fn model_from_response<U>(
        &self,
        response: NetworkResponse,
    ) -> Result<U, CommonError>
    where
        U: for<'a> Deserialize<'a>,
    {
        if let 200..=299 = response.status_code {
            // all good
        } else {
            return Err(CommonError::NetworkResponseBadCode);
        }

        if response.body.is_empty() {
            return Err(CommonError::NetworkResponseEmptyBody);
        }

        serde_json::from_slice::<U>(&response.body).map_err(|_| {
            CommonError::NetworkResponseJSONDeserialize {
                into_type: std::any::type_name::<U>().to_owned(),
            }
        })
    }

    async fn dispatch_network_request<T, U, V, F>(
        &self,
        path: impl AsRef<str>,
        method: NetworkMethod,
        request: T,
        map: F,
    ) -> Result<V, CommonError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, CommonError>,
    {
        // JSON serialize request into body bytes
        let body = BagOfBytes::from(serde_json::to_vec(&request).unwrap());

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

        let headers = HashMap::<String, String>::from_iter([
            ("content-Type".to_owned(), "application/json".to_owned()),
            ("accept".to_owned(), "application/json".to_owned()),
            ("user-agent".to_owned(), "Sargon".to_owned()), // https://stackoverflow.com/a/77866494/1311272
            ("RDX-Client-Name".to_owned(), "Sargon".to_owned()),
            ("RDX-Client-Version".to_owned(), "1.5.1".to_owned()),
        ]);

        let request = NetworkRequest {
            url,
            body,
            method,
            headers,
        };

        // Let Swift side make network request and await response
        let response = self
            .network_antenna
            .execute_network_request(request)
            .await?;

        // Read out HTTP body from response and JSON parse it into U
        let model = self.model_from_response(response)?;

        // Map U -> V
        map(model)
    }
}
