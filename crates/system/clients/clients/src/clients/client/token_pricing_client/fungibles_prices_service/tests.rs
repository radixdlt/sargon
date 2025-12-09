use crate::prelude::*;
use crate::clients::client::token_pricing_client::fungibles_prices_service::cache::*;
use std::path::Path;

#[allow(clippy::upper_case_acronyms)]
type SUT = FungiblesPricesClient;

// Test helper functions

fn make_http_client_with_responses(
    responses: Vec<Vec<TokenPrice>>,
) -> Arc<HttpClient> {
    Arc::new(HttpClient::new(Arc::new(
        MockNetworkingDriver::with_responses(responses),
    )))
}

fn make_http_client_with_single_response(
    prices: Vec<TokenPrice>,
) -> Arc<HttpClient> {
    make_http_client_with_responses(vec![prices])
}

fn make_http_client_failing() -> Arc<HttpClient> {
    Arc::new(HttpClient::new(Arc::new(
        MockNetworkingDriver::new_always_failing(),
    )))
}

fn sample_token_price(
    address: &str,
    price: f32,
    currency: FiatCurrency,
) -> TokenPrice {
    TokenPrice {
        resource_address: ResourceAddress::from_str(address).unwrap(),
        price,
        currency,
    }
}

fn sample_token_prices_usd() -> Vec<TokenPrice> {
    vec![
        sample_token_price(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            0.5,
            FiatCurrency::USD,
        ),
        sample_token_price(
            "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
            1.25,
            FiatCurrency::USD,
        ),
    ]
}

fn sample_token_prices_mixed() -> Vec<TokenPrice> {
    vec![
        sample_token_price(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            0.5,
            FiatCurrency::USD,
        ),
        sample_token_price(
            "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
            5.0,
            FiatCurrency::SEK,
        ),
        sample_token_price(
            "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
            1.25,
            FiatCurrency::USD,
        ),
    ]
}

// Tests for get_prices_for_currency

#[actix_rt::test]
async fn test_get_prices_for_currency_filters_correctly() {
    // Arrange
    let prices = sample_token_prices_mixed();
    let http_client = make_http_client_with_single_response(prices);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert
    assert!(result.is_ok());
    let per_token_prices = result.unwrap();
    assert_eq!(per_token_prices.len(), 2); // Only USD prices

    // Check that we got the right prices
    let addr1 = ResourceAddress::from_str(
        "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
    )
    .unwrap();
    let addr3 = ResourceAddress::from_str(
        "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
    )
    .unwrap();

    assert!(per_token_prices.contains_key(&addr1));
    assert!(per_token_prices.contains_key(&addr3));
    assert_eq!(
        per_token_prices.get(&addr1).unwrap(),
        &Decimal192::from(0.5f32)
    );
    assert_eq!(
        per_token_prices.get(&addr3).unwrap(),
        &Decimal192::from(1.25f32)
    );
}

#[actix_rt::test]
async fn test_get_prices_for_currency_no_matches_returns_empty() {
    // Arrange
    let prices = sample_token_prices_usd();
    let http_client = make_http_client_with_single_response(prices);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::SEK).await;

    // Assert
    assert!(result.is_ok());
    let per_token_prices = result.unwrap();
    assert_eq!(per_token_prices.len(), 0); // No SEK prices
}

#[actix_rt::test]
async fn test_get_prices_for_currency_with_empty_response() {
    // Arrange
    let http_client = make_http_client_with_single_response(vec![]);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert
    assert!(result.is_ok());
    let per_token_prices = result.unwrap();
    assert_eq!(per_token_prices.len(), 0);
}

// Tests for remote fetching

#[actix_rt::test]
async fn test_fetch_remote_token_prices_success() {
    // Arrange
    let prices = sample_token_prices_usd();
    let http_client = make_http_client_with_single_response(prices.clone());
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.fetch_remote_token_prices().await;

    // Assert
    assert!(result.is_ok());
    let fetched_prices = result.unwrap();
    assert_eq!(fetched_prices.len(), 2);
    assert_eq!(fetched_prices[0].price, 0.5);
    assert_eq!(fetched_prices[0].currency, FiatCurrency::USD);
    assert_eq!(fetched_prices[1].price, 1.25);
}

#[actix_rt::test]
async fn test_fetch_remote_token_prices_network_failure() {
    // Arrange
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.fetch_remote_token_prices().await;

    // Assert
    assert!(result.is_err());
}

// Tests for cache loading

#[actix_rt::test]
async fn test_load_cached_prices_when_no_cache() {
    // Arrange
    let http_client = make_http_client_failing(); // Doesn't matter for this test
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.load_cached_prices().await;

    // Assert - should return Ok(None) when no cache exists
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[actix_rt::test]
async fn test_load_cached_prices_with_valid_cache() {
    // Arrange
    let http_client = make_http_client_failing(); // Won't be used
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client.clone(), file_system.clone());

    // Store prices first
    let prices = sample_token_prices_usd();
    let result = sut.store_prices(prices.clone()).await;
    assert!(result.is_ok());

    // Act - load cached prices
    let loaded_result = sut.load_cached_prices().await;

    // Assert
    assert!(loaded_result.is_ok());
    let loaded_prices = loaded_result.unwrap();
    assert!(loaded_prices.is_some());
    let loaded_prices = loaded_prices.unwrap();
    assert_eq!(loaded_prices.len(), 2);
    assert_eq!(loaded_prices[0].price, 0.5);
    assert_eq!(loaded_prices[1].price, 1.25);
}

#[actix_rt::test]
async fn test_load_cached_prices_with_expired_cache() {
    // Arrange
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());

    // Manually create an expired cache (fetched_at in the past)
    let expired_snapshot = AllTokenPricesSnapshot {
        fetched_at: Timestamp::parse("2020-01-01T00:00:00Z").unwrap(),
        prices: sample_token_prices_usd(),
    };

    let serialized = expired_snapshot.serialize_to_bytes().unwrap();
    file_system
        .save_to_file(Path::new("all_token_prices.json"), serialized, true)
        .await
        .unwrap();

    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.load_cached_prices().await;

    // Assert - expired cache should return None
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// Tests for cache storage

#[actix_rt::test]
async fn test_store_prices_success() {
    // Arrange
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system.clone());

    let prices = sample_token_prices_usd();

    // Act
    let result = sut.store_prices(prices.clone()).await;

    // Assert
    assert!(result.is_ok());

    // Verify the file was created
    let loaded = file_system
        .load_from_file(Path::new("all_token_prices.json"))
        .await
        .unwrap();
    assert!(loaded.is_some());
}

#[actix_rt::test]
async fn test_store_and_load_roundtrip() {
    // Arrange
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    let prices = sample_token_prices_mixed();

    // Act - store and then load
    sut.store_prices(prices.clone()).await.unwrap();
    let loaded = sut.load_cached_prices().await.unwrap();

    // Assert
    assert!(loaded.is_some());
    let loaded_prices = loaded.unwrap();
    assert_eq!(loaded_prices.len(), prices.len());

    // Verify each price
    for (expected, actual) in prices.iter().zip(loaded_prices.iter()) {
        assert_eq!(expected.resource_address, actual.resource_address);
        assert_eq!(expected.price, actual.price);
        assert_eq!(expected.currency, actual.currency);
    }
}

// Integration tests for get_token_prices (internal method testing the cache-first strategy)

#[actix_rt::test]
async fn test_get_token_prices_uses_cache_when_available() {
    // Arrange
    let http_client = make_http_client_failing(); // Will fail if called
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client.clone(), file_system.clone());

    // Pre-populate cache
    let cached_prices = sample_token_prices_usd();
    sut.store_prices(cached_prices.clone()).await.unwrap();

    // Act - this should use cache and NOT call HTTP (which would fail)
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - should succeed because it uses cache
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 2);
}

#[actix_rt::test]
async fn test_get_token_prices_fetches_remote_on_cache_miss() {
    // Arrange - no cache, but HTTP will succeed
    let remote_prices = sample_token_prices_usd();
    let http_client =
        make_http_client_with_single_response(remote_prices.clone());
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system.clone());

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - should succeed by fetching from remote
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 2);

    // Verify cache was populated
    let cached = sut.load_cached_prices().await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_get_token_prices_returns_error_when_cache_miss_and_remote_fails()
{
    // Arrange - no cache AND HTTP fails
    let http_client = make_http_client_failing();
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - should fail
    assert!(result.is_err());
}

#[actix_rt::test]
async fn test_get_token_prices_prefers_cache_over_remote() {
    // Arrange - cache available with different prices than remote
    let cached_prices = vec![sample_token_price(
        "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
        99.99, // Different price in cache
        FiatCurrency::USD,
    )];

    let remote_prices = sample_token_prices_usd(); // Different prices
    let http_client = make_http_client_with_single_response(remote_prices);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Store cache first
    sut.store_prices(cached_prices.clone()).await.unwrap();

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - should get cached price (99.99), not remote price (0.5)
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);

    let addr = ResourceAddress::from_str(
        "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
    )
    .unwrap();
    assert_eq!(prices.get(&addr).unwrap(), &Decimal192::from(99.99f32));
}

#[actix_rt::test]
async fn test_expired_cache_triggers_remote_fetch() {
    // Arrange - expired cache should trigger remote fetch
    let file_system = Arc::new(FileSystemClient::in_memory());

    // Create expired cache
    let expired_snapshot = AllTokenPricesSnapshot {
        fetched_at: Timestamp::parse("2020-01-01T00:00:00Z").unwrap(),
        prices: vec![sample_token_price(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            99.99,
            FiatCurrency::USD,
        )],
    };
    let serialized = expired_snapshot.serialize_to_bytes().unwrap();
    file_system
        .save_to_file(Path::new("all_token_prices.json"), serialized, true)
        .await
        .unwrap();

    // Remote will return different prices
    let remote_prices = sample_token_prices_usd();
    let http_client = make_http_client_with_single_response(remote_prices);
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - should get remote prices (not expired cache)
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 2); // Remote has 2 prices, cache had 1

    // First price should be from remote (0.5), not cache (99.99)
    let addr = ResourceAddress::from_str(
        "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
    )
    .unwrap();
    assert_eq!(
        prices.get(&addr).unwrap(),
        &Decimal192::from(0.5f32) // Remote price
    );
}

// Tests for data consistency and edge cases

#[actix_rt::test]
async fn test_price_conversion_from_f32_to_decimal192() {
    // Arrange
    let prices = vec![
        sample_token_price(
            "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
            0.123456,
            FiatCurrency::USD,
        ),
        sample_token_price(
            "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
            999999.99,
            FiatCurrency::USD,
        ),
    ];

    let http_client = make_http_client_with_single_response(prices);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act
    let result = sut.get_prices_for_currency(FiatCurrency::USD).await;

    // Assert - verify conversion worked
    assert!(result.is_ok());
    let per_token_prices = result.unwrap();
    assert_eq!(per_token_prices.len(), 2);

    // Verify the Decimal192 conversion
    for value in per_token_prices.values() {
        // All values should be valid Decimal192
        assert!(value.clone() >= Decimal192::zero());
    }
}

#[actix_rt::test]
async fn test_multiple_calls_use_cache() {
    // Arrange
    let remote_prices = sample_token_prices_usd();
    let http_client = make_http_client_with_single_response(remote_prices);
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Act - first call fetches from remote and caches
    let result1 = sut.get_prices_for_currency(FiatCurrency::USD).await;
    assert!(result1.is_ok());

    // Second call should use cache (HTTP client only has one response, would fail on second call)
    let result2 = sut.get_prices_for_currency(FiatCurrency::USD).await;
    assert!(result2.is_ok());

    // Third call should also use cache
    let result3 = sut.get_prices_for_currency(FiatCurrency::SEK).await;
    assert!(result3.is_ok());
}

#[test]
fn test_token_price_decoding() {
    let raw_json = r#"
        [
  {
    "id": 2,
    "resource_address": "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
    "symbol": "$BOBBY",
    "name": "Bobby",
    "price": 0.04134813463690507,
    "currency": "USD"
  },
  {
    "id": 102,
    "resource_address": "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
    "symbol": "$MRD",
    "name": "Memerad",
    "price": 0.000004698356561816775,
    "currency": "USD"
  },
  {
    "id": 133,
    "resource_address": "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
    "symbol": "$WOWO",
    "name": "WOWO",
    "price": 0.00007746121946503883,
    "currency": "USD"
  }
  ]
        "#;

    let decoded: Vec<TokenPrice> = serde_json::from_str(raw_json).unwrap();
    let expected_tokens = vec![
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
                )
                .unwrap(),
                price: 0.04134813463690507,
                currency: FiatCurrency::USD,
            },
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
                )
                .unwrap(),
                price: 0.000004698356561816775,
                currency: FiatCurrency::USD,
            },
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
                )
                .unwrap(),
                price: 0.00007746121946503883,
                currency: FiatCurrency::USD,
            },
        ];

    pretty_assertions::assert_eq!(decoded, expected_tokens);
}
