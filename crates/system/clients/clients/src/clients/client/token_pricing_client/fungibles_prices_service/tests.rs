use crate::clients::client::token_pricing_client::fungibles_prices_service::cache::*;
use crate::prelude::*;
use std::path::Path;

#[allow(clippy::upper_case_acronyms)]
type SUT = FungiblesPricesClient;

fn make_http_client_with_responses(
    responses: Vec<ScopedTokenPricesResponse>,
) -> Arc<HttpClient> {
    Arc::new(HttpClient::new(Arc::new(
        MockNetworkingDriver::with_responses(responses),
    )))
}

fn make_http_client_with_single_response(
    response: ScopedTokenPricesResponse,
) -> Arc<HttpClient> {
    make_http_client_with_responses(vec![response])
}

fn make_http_client_failing() -> Arc<HttpClient> {
    Arc::new(HttpClient::new(Arc::new(
        MockNetworkingDriver::new_always_failing(),
    )))
}

fn addr(s: &str) -> ResourceAddress {
    ResourceAddress::from_str(s).unwrap()
}

fn token(resource_address: &str, usd_price: f64) -> ScopedTokenPrice {
    ScopedTokenPrice {
        resource_address: addr(resource_address),
        usd_price,
    }
}

fn lsu(resource_address: &str, usd_price: f64) -> ScopedLsuPrice {
    ScopedLsuPrice {
        resource_address: addr(resource_address),
        usd_price,
    }
}

fn sample_scoped_response() -> ScopedTokenPricesResponse {
    ScopedTokenPricesResponse {
        tokens: vec![
            token(
                "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
                0.5,
            ),
            token(
                "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
                1.25,
            ),
        ],
        lsus: vec![lsu(
            "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
            2.0,
        )],
    }
}

fn request_usd_with_unsorted_duplicates() -> FungiblePricesRequest {
    FungiblePricesRequest::new(
        FiatCurrency::USD,
        vec![
            addr(
                "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
            ),
            addr(
                "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            ),
            addr(
                "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
            ),
        ],
        vec![
            addr(
                "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
            ),
            addr(
                "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
            ),
        ],
    )
}

fn token_price_services_two_endpoints() -> TokenPriceServices {
    TokenPriceServices::from_iter([
        TokenPriceService::new(
            Url::parse("https://token-prices-primary.example").unwrap(),
        ),
        TokenPriceService::new(
            Url::parse("https://token-prices-secondary.example").unwrap(),
        ),
    ])
}

fn cache_file_path_for(request: &FungiblePricesRequest) -> String {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut hasher = DefaultHasher::new();
    request.hash(&mut hasher);
    let hash = hasher.finish();

    format!("scoped_token_prices_{}.json", hash)
}

#[test]
fn test_request_normalizes_and_deduplicates() {
    let request = request_usd_with_unsorted_duplicates();

    assert_eq!(request.tokens.len(), 2);
    assert_eq!(request.lsus.len(), 1);

    let token_strings: Vec<String> =
        request.tokens.iter().map(|a| a.to_string()).collect();
    let lsu_strings: Vec<String> =
        request.lsus.iter().map(|a| a.to_string()).collect();

    assert!(token_strings.windows(2).all(|w| w[0] <= w[1]));
    assert!(lsu_strings.windows(2).all(|w| w[0] <= w[1]));
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_success() {
    let response = sample_scoped_response();
    let http_client = make_http_client_with_single_response(response);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let result = sut.fetch_remote_token_prices(&request).await;

    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 3);
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_uses_requested_currency() {
    let captured_requests =
        Arc::new(std::sync::Mutex::new(Vec::<NetworkRequest>::new()));
    let captured_requests_clone = captured_requests.clone();

    let response = sample_scoped_response();

    let driver = Arc::new(MockNetworkingDriver::with_lazy_responses(
        move |request, _| {
            captured_requests_clone
                .lock()
                .unwrap()
                .push(request.clone());
            let body = serde_json::to_vec(&response).unwrap();
            NetworkResponse::new(200, body)
        },
    ));

    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = FungiblePricesRequest::new(
        FiatCurrency::SEK,
        vec![addr(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
        )],
        vec![],
    );

    let _ = sut.fetch_remote_token_prices(&request).await.unwrap();

    let captured = captured_requests.lock().unwrap();
    assert_eq!(captured.len(), 1);
    let body: Vec<u8> = captured[0].body.to_vec();
    let decoded: FungiblePricesRequest = serde_json::from_slice(&body).unwrap();
    assert_eq!(decoded.currency, FiatCurrency::SEK);
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_fails_over_to_next_service() {
    let captured_requests =
        Arc::new(std::sync::Mutex::new(Vec::<NetworkRequest>::new()));
    let captured_requests_clone = captured_requests.clone();
    let response = sample_scoped_response();

    let driver = Arc::new(MockNetworkingDriver::with_lazy_responses(
        move |request, _| {
            captured_requests_clone
                .lock()
                .unwrap()
                .push(request.clone());

            if request.url.host_str() == Some("token-prices-primary.example") {
                NetworkResponse::new(500, vec![])
            } else {
                let body = serde_json::to_vec(&response).unwrap();
                NetworkResponse::new(200, body)
            }
        },
    ));

    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let result = sut
        .fetch_remote_token_prices_using_token_price_services(
            &request,
            token_price_services_two_endpoints(),
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);

    let captured = captured_requests.lock().unwrap();
    assert_eq!(captured.len(), 2);
    assert_eq!(
        captured[0].url.as_str(),
        "https://token-prices-primary.example/price/tokens"
    );
    assert_eq!(
        captured[1].url.as_str(),
        "https://token-prices-secondary.example/price/tokens"
    );
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_all_services_fail() {
    let driver = Arc::new(MockNetworkingDriver::new_always_failing());
    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let result = sut
        .fetch_remote_token_prices_using_token_price_services(
            &request,
            token_price_services_two_endpoints(),
        )
        .await;

    assert!(result.is_err());
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_empty_services_returns_error() {
    let response = sample_scoped_response();
    let http_client = make_http_client_with_single_response(response);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let result = sut
        .fetch_remote_token_prices_using_token_price_services(
            &request,
            TokenPriceServices::new(),
        )
        .await;

    assert_eq!(result, Err(CommonError::ExpectedNonEmptyCollection));
}

#[actix_rt::test]
async fn test_get_prices_for_request_fetches_and_caches() {
    let response = sample_scoped_response();
    let http_client = make_http_client_with_single_response(response);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let result = sut
        .get_prices_for_request(request.clone(), false)
        .await
        .unwrap();

    assert_eq!(result.len(), 3);
    let cached = sut.load_cached_prices(&request).await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_get_prices_for_request_uses_cache() {
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let mut cached_prices = PerTokenPrices::new();
    cached_prices.insert(
        addr("resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0"),
        Decimal192::from(99.0f32),
    );

    sut.store_prices(&request, &cached_prices).await.unwrap();

    let result = sut.get_prices_for_request(request, false).await.unwrap();
    assert_eq!(result, cached_prices);
}

#[actix_rt::test]
async fn test_force_fetch_bypasses_cache() {
    let response = ScopedTokenPricesResponse {
        tokens: vec![token(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            1.0,
        )],
        lsus: vec![],
    };
    let http_client = make_http_client_with_single_response(response);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request = request_usd_with_unsorted_duplicates();
    let mut cached_prices = PerTokenPrices::new();
    cached_prices.insert(
        addr("resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0"),
        Decimal192::from(99.0f32),
    );
    sut.store_prices(&request, &cached_prices).await.unwrap();

    let result = sut.get_prices_for_request(request, true).await.unwrap();

    assert_eq!(
        result
            .get(&addr(
                "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            ))
            .unwrap(),
        &Decimal192::from(1.0f32)
    );
}

#[actix_rt::test]
async fn test_cache_is_request_scoped() {
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let request_a = request_usd_with_unsorted_duplicates();
    let request_b = FungiblePricesRequest::new(
        FiatCurrency::USD,
        vec![addr(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
        )],
        vec![],
    );

    let mut prices = PerTokenPrices::new();
    prices.insert(
        addr("resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0"),
        Decimal192::from(5.0f32),
    );

    sut.store_prices(&request_a, &prices).await.unwrap();

    let cached_a = sut.load_cached_prices(&request_a).await.unwrap();
    let cached_b = sut.load_cached_prices(&request_b).await.unwrap();

    assert!(cached_a.is_some());
    assert!(cached_b.is_none());
}

#[actix_rt::test]
async fn test_expired_cache_triggers_remote_fetch() {
    let request = request_usd_with_unsorted_duplicates();
    let response = ScopedTokenPricesResponse {
        tokens: vec![token(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            1.0,
        )],
        lsus: vec![],
    };

    let http_client = make_http_client_with_single_response(response);
    let file_system = Arc::new(FileSystemClient::in_memory());

    let expired_snapshot = ScopedTokenPricesSnapshot {
        fetched_at: Timestamp::parse("2020-01-01T00:00:00Z").unwrap(),
        prices: {
            let mut map = PerTokenPrices::new();
            map.insert(
                addr(
                    "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
                ),
                Decimal192::from(99.0f32),
            );
            map
        },
    };

    let serialized = expired_snapshot.serialize_to_bytes().unwrap();
    let path = cache_file_path_for(&request);
    file_system
        .save_to_file(Path::new(&path), serialized, true)
        .await
        .unwrap();

    let sut = SUT::new(http_client, file_system);

    let result = sut.get_prices_for_request(request, false).await.unwrap();

    assert_eq!(
        result
            .get(&addr(
                "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            ))
            .unwrap(),
        &Decimal192::from(1.0f32)
    );
}
