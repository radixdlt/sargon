use crate::prelude::*;

const SUFFIX_WELL_KNOWN_FILE: &str = ".well-known/radix.json";

/// The Well Known Client .
/// It will be used to fetch the well-known file for a given origin.
pub struct WellKnownClient {
    /// The HTTP client to be used to make the requests.
    pub http_client: HttpClient,
}

impl WellKnownClient {
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    pub fn new_with_networking_driver(
        networking_driver: Arc<dyn NetworkingDriver>,
    ) -> Self {
        Self::new(HttpClient::new(networking_driver))
    }
}

impl NetworkRequest {
    fn get_well_known(url: Url) -> Self {
        let well_known_url = url.join(SUFFIX_WELL_KNOWN_FILE).unwrap();
        NetworkRequest::new_get(well_known_url)
    }
}

impl WellKnownClient {
    /// Fetches the well-known file, internally it appends the suffix to the origin URL.
    pub async fn get_well_known_file(
        &self,
        origin: Url,
    ) -> Result<DappWellKnownData> {
        todo!()
        // let network_request = NetworkRequest::get_well_known(origin);
        // self.http_client
        //     .execute_request_with_decoding(network_request)
        //     .await
    }
}

#[cfg(test)]
impl WellKnownClient {
    fn new_always_failing() -> Self {
        Self::new_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::time::Duration;
    const MAX: Duration = Duration::from_millis(10);

    const TEST_ORIGIN: &str = "https://d2xmq49o1iddud.cloudfront.net";

    #[test]
    fn test_suffix_is_correct() {
        assert_eq!(SUFFIX_WELL_KNOWN_FILE, ".well-known/radix.json")
    }

    #[test]
    fn test_get_well_known_request_error() {
        // ARRANGE
        let url = Url::parse(TEST_ORIGIN).unwrap();
        // ACT
        let request = NetworkRequest::get_well_known(url.clone());
        // ASSERT
        assert_eq!(request.url, url.join(SUFFIX_WELL_KNOWN_FILE).unwrap());
        assert_eq!(request.method, NetworkMethod::Get);
    }

    #[actix_rt::test]
    async fn test_get_well_known_file_correct_request_made() {
        // ARRANGE
        let mock_antenna_with_spy =
            MockNetworkingDriver::with_spy(200, vec![], |_, _| {
                let request = NetworkRequest::get_well_known(
                    Url::from_str(TEST_ORIGIN).unwrap(),
                );

                let expected_request = NetworkRequest::new_get(
                    Url::from_str(TEST_ORIGIN)
                        .unwrap()
                        .join(SUFFIX_WELL_KNOWN_FILE)
                        .unwrap(),
                );

                // ASSERT
                pretty_assertions::assert_eq!(request, expected_request);
            });

        let client = WellKnownClient::new_with_networking_driver(Arc::new(
            mock_antenna_with_spy,
        ));
        // ACT
        let req =
            client.get_well_known_file(Url::from_str(TEST_ORIGIN).unwrap());
        let _ = timeout(MAX, req).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_get_well_known_file_failure_response() {
        // ARRANGE
        let client = WellKnownClient::new_always_failing();
        // ACT
        let req =
            client.get_well_known_file(Url::from_str(TEST_ORIGIN).unwrap());
        let result = timeout(MAX, req).await.unwrap();
        // ASSERT
        assert!(result.is_err());
    }
}
