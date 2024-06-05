use crate::prelude::*;

const SUFFIX_WELL_KNOWN_FILE: &str = ".well-known/dapp.json";

pub struct WellKnownClient {
    pub http_client: HttpClient,
}

impl WellKnownClient {
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    pub fn new_with_network_antenna(
        network_antenna: Arc<dyn NetworkAntenna>,
    ) -> Self {
        Self::new(HttpClient::new(network_antenna))
    }
}

impl NetworkRequest {
    fn get_well_known_request(url: Url) -> Self {
        let well_known_url = url.join(SUFFIX_WELL_KNOWN_FILE).unwrap();
        NetworkRequest::new_get(well_known_url)
    }
}

impl WellKnownClient {
    pub async fn get_well_known_file(
        &self,
        url: Url,
    ) -> Result<DappDefinitions> {
        let network_request = NetworkRequest::get_well_known_request(url);
        self.http_client
            .execute_request_with_decoding(network_request)
            .await
    }
}

#[cfg(test)]
impl WellKnownClient {
    fn new_always_failing() -> Self {
        Self::new_with_network_antenna(Arc::new(
            MockAntenna::new_always_failing(),
        ))
    }

    fn new_succeeding_http_client(response: &DappDefinitions) -> Self {
        Self::new_with_network_antenna(Arc::new(MockAntenna::with_response(
            response,
        )))
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
    fn test_get_well_known_request_error() {
        let url = Url::parse(TEST_ORIGIN).unwrap();
        let request = NetworkRequest::get_well_known_request(url.clone());
        assert_eq!(request.url, url.join(SUFFIX_WELL_KNOWN_FILE).unwrap());
        assert_eq!(request.method, NetworkMethod::Get);
    }

    #[actix_rt::test]
    async fn test_get_well_known_file_correct_request_made() {
        let mock_antenna_with_spy =
            MockAntenna::with_spy(200, vec![], |request| {
                let request = NetworkRequest::get_well_known_request(
                    Url::from_str(TEST_ORIGIN).unwrap(),
                );

                let expected_request = NetworkRequest {
                    url: Url::from_str(TEST_ORIGIN)
                        .unwrap()
                        .join(SUFFIX_WELL_KNOWN_FILE)
                        .unwrap(),
                    method: NetworkMethod::Get,
                    body: BagOfBytes::new(),
                    headers: HashMap::new(),
                };

                pretty_assertions::assert_eq!(
                    request.url,
                    expected_request.url
                );
                pretty_assertions::assert_eq!(
                    request.method,
                    expected_request.method
                );
                pretty_assertions::assert_eq!(
                    request.body,
                    expected_request.body
                );
            });

        let client = WellKnownClient::new_with_network_antenna(Arc::new(
            mock_antenna_with_spy,
        ));

        let req =
            client.get_well_known_file(Url::from_str(TEST_ORIGIN).unwrap());
        let _ = timeout(MAX, req).await.unwrap();
    }

    #[actix_rt::test]
    async fn test_get_well_known_file_failure_response() {
        let client = WellKnownClient::new_always_failing();

        let req =
            client.get_well_known_file(Url::from_str(TEST_ORIGIN).unwrap());
        let result = timeout(MAX, req).await.unwrap();
        assert!(result.is_err());
    }

    // #[actix_rt::test]
    // async fn test_1() {
    //     let (sut, json) = fixture_and_json::<DappDefinitions>(include_str!(concat!(
    //         env!("FIXTURES_MODELS"),
    //         "well_known.json"
    //     )))
    //     .unwrap();
    //     let js = serde_json::to_value(&sut).unwrap();
    //     let mock_antenna = MockAntenna::new(200, json.into());
    //     let client = WellKnownClient::new_with_network_antenna(Arc::new(
    //         mock_antenna
    //     ));

    //     let req =
    //         client.get_well_known_file(Url::from_str(TEST_ORIGIN).unwrap());
    //     let result = timeout(MAX, req).await.unwrap().unwrap();
    //     assert_eq!(result, sut);
    // }
}
