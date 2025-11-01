use crate::prelude::*;
use std::path::Path;

#[allow(clippy::upper_case_acronyms)]
type SUT = NonFungiblePricesClient;

// Test helper functions

fn sample_resource_addresses() -> (ResourceAddress, ResourceAddress) {
    (
        ResourceAddress::sample_mainnet_xrd(),
        ResourceAddress::sample_mainnet_candy(),
    )
}

fn sample_nft_resource_addresses() -> (ResourceAddress, ResourceAddress) {
    (
        ResourceAddress::sample_mainnet_nft_gc_membership(),
        ResourceAddress::sample_mainnet_nft_other(),
    )
}

fn sample_nft_global_id(
    resource: ResourceAddress,
    local_id: NonFungibleLocalId,
) -> NonFungibleGlobalId {
    NonFungibleGlobalId::new_unchecked(resource, local_id)
}

fn sample_liquidity_receipt(
    resource_address: ResourceAddress,
    items: Vec<LiquidityReceiptItem>,
) -> NonFungibleLiquidityReceipt {
    NonFungibleLiquidityReceipt {
        resource_manager_address: resource_address,
        items,
    }
}

fn sample_liquidity_receipt_item(
    local_id: NonFungibleLocalId,
    resources: Vec<LiquidityReceiptItemResource>,
) -> LiquidityReceiptItem {
    LiquidityReceiptItem { local_id, resources }
}

fn sample_liquidity_receipt_item_resource(
    resource: ResourceAddress,
    amount: Decimal192,
) -> LiquidityReceiptItemResource {
    LiquidityReceiptItemResource { resource, amount }
}

fn make_client_with_responses(
    liquidity_receipts: Vec<NonFungibleLiquidityReceipt>,
    token_prices: Vec<TokenPrice>,
) -> SUT {
    // Mock HTTP client for both NFT and fungible endpoints
    let captured_requests = Arc::new(std::sync::Mutex::new(Vec::<NetworkRequest>::new()));
    let captured_requests_clone = captured_requests.clone();

    let liquidity_receipts_clone = liquidity_receipts.clone();
    let token_prices_clone = token_prices.clone();

    let driver = Arc::new(MockNetworkingDriver::with_lazy_responses(
        move |request, _| {
            captured_requests_clone
                .lock()
                .unwrap()
                .push(request.clone());

            // Check which endpoint is being called
            if request.url.as_str().contains("liquidity-reciept") {
                let body = serde_json::to_vec(&liquidity_receipts_clone).unwrap();
                NetworkResponse::new(200, body)
            } else if request.url.as_str().contains("token-price-service") {
                let body = serde_json::to_vec(&token_prices_clone).unwrap();
                NetworkResponse::new(200, body)
            } else {
                NetworkResponse::new(404, vec![])
            }
        },
    ));

    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());

    SUT::new(http_client, file_system)
}

fn make_client_failing() -> SUT {
    let driver = Arc::new(MockNetworkingDriver::new_always_failing());
    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());
    SUT::new(http_client, file_system)
}

fn sample_token_price(
    address: ResourceAddress,
    price: f32,
    currency: FiatCurrency,
) -> TokenPrice {
    TokenPrice {
        resource_address: address,
        price,
        currency,
    }
}

// Tests for fetch_non_fungibles_prices

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_calculates_value_correctly() {
    // Arrange
    let (xrd, candy) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    // NFT contains 100 XRD and 50 CANDY
    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![
                sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100)),
                sample_liquidity_receipt_item_resource(candy.clone(), Decimal192::from(50)),
            ],
        )],
    );

    // XRD = $1.0, CANDY = $2.0
    let token_prices = vec![
        sample_token_price(xrd, 1.0, FiatCurrency::USD),
        sample_token_price(candy, 2.0, FiatCurrency::USD),
    ];

    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_global_id.clone()]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);

    // Value should be: (100 * $1.0) + (50 * $2.0) = $100 + $100 = $200
    let value = prices.get(&nft_global_id).unwrap();
    assert_eq!(*value, Decimal192::from(100.0) + Decimal192::from(100.0));
}

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_multiple_nfts() {
    // Arrange
    let (xrd, candy) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_1_local_id = NonFungibleLocalId::integer(1);
    let nft_2_local_id = NonFungibleLocalId::integer(2);
    let nft_1_global_id = sample_nft_global_id(nft_resource.clone(), nft_1_local_id.clone());
    let nft_2_global_id = sample_nft_global_id(nft_resource.clone(), nft_2_local_id.clone());

    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![
            sample_liquidity_receipt_item(
                nft_1_local_id.clone(),
                vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100))],
            ),
            sample_liquidity_receipt_item(
                nft_2_local_id.clone(),
                vec![sample_liquidity_receipt_item_resource(candy.clone(), Decimal192::from(50))],
            ),
        ],
    );

    let token_prices = vec![
        sample_token_price(xrd, 1.5, FiatCurrency::USD),
        sample_token_price(candy, 0.5, FiatCurrency::USD),
    ];

    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_1_global_id.clone(), nft_2_global_id.clone()]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 2);

    // NFT1 has 100 XRD at $1.5 = $150
    assert_eq!(*prices.get(&nft_1_global_id).unwrap(), Decimal192::from(150.0));
    // NFT2 has 50 CANDY at $0.5 = $25
    assert_eq!(*prices.get(&nft_2_global_id).unwrap(), Decimal192::from(25.0));
}

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_filters_by_currency() {
    // Arrange
    let (xrd, _) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100))],
        )],
    );

    // XRD has both USD and SEK prices
    let token_prices = vec![
        sample_token_price(xrd.clone(), 1.0, FiatCurrency::USD),
        sample_token_price(xrd, 10.0, FiatCurrency::SEK),
    ];

    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_global_id.clone()]);

    // Act - request SEK prices
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::SEK)
        .await;

    // Assert - should use SEK price: 100 XRD at 10.0 SEK = 1000 SEK
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);
    assert_eq!(*prices.get(&nft_global_id).unwrap(), Decimal192::from(1000.0));
}

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_missing_token_price() {
    // Arrange
    let (xrd, candy) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    // NFT contains both XRD and CANDY
    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![
                sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100)),
                sample_liquidity_receipt_item_resource(candy, Decimal192::from(50)),
            ],
        )],
    );

    // Only XRD price is available, CANDY price is missing
    let token_prices = vec![sample_token_price(xrd, 1.0, FiatCurrency::USD)];

    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_global_id.clone()]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert - should only count XRD (100 * $1.0 = $100), ignore CANDY
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);
    assert_eq!(*prices.get(&nft_global_id).unwrap(), Decimal192::from(100.0));
}

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_empty_receipt() {
    // Arrange
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    // NFT has no resources
    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(nft_local_id.clone(), vec![])],
    );

    let token_prices = vec![];
    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_global_id.clone()]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert - value should be zero
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);
    assert_eq!(*prices.get(&nft_global_id).unwrap(), Decimal192::zero());
}

#[actix_rt::test]
async fn test_fetch_non_fungibles_prices_network_failure() {
    // Arrange
    let sut = make_client_failing();
    let (nft_resource, _) = sample_nft_resource_addresses();
    let nft_global_id = sample_nft_global_id(nft_resource, NonFungibleLocalId::integer(1));
    let addresses = HashSet::from([nft_global_id]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result.is_err());
}

// Tests for cache functionality

#[actix_rt::test]
async fn test_load_cached_snapshot_when_no_cache() {
    // Arrange
    let sut = make_client_failing();

    // Act
    let result = sut.load_cached_snapshot().await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[actix_rt::test]
async fn test_cache_and_load_snapshot_roundtrip() {
    // Arrange
    let (xrd, _) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);

    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd, Decimal192::from(100))],
        )],
    );

    let snapshot = LiquidityReceiptsSnapshot::new(
        Timestamp::now_utc(),
        vec![liquidity_receipt.clone()],
    );

    let sut = make_client_failing();

    // Act - store snapshot
    let store_result = sut.cache_snapshot(snapshot).await;
    assert!(store_result.is_ok());

    // Act - load snapshot
    let load_result = sut.load_cached_snapshot().await;

    // Assert
    assert!(load_result.is_ok());
    let loaded = load_result.unwrap();
    assert!(loaded.is_some());
    let loaded_receipts = loaded.unwrap();
    assert_eq!(loaded_receipts.len(), 1);
    assert_eq!(loaded_receipts[0], liquidity_receipt);
}

#[actix_rt::test]
async fn test_cache_expired_returns_none() {
    // Arrange
    let (nft_resource, _) = sample_nft_resource_addresses();
    let liquidity_receipt = sample_liquidity_receipt(nft_resource, vec![]);

    // Create expired snapshot (from year 2020)
    let expired_snapshot = LiquidityReceiptsSnapshot::new(
        Timestamp::parse("2020-01-01T00:00:00Z").unwrap(),
        vec![liquidity_receipt],
    );

    let sut = make_client_failing();

    // Manually store expired snapshot
    let serialized = expired_snapshot.serialize_to_bytes().unwrap();
    sut.file_system_client
        .save_to_file(
            Path::new("liquidity_receipts_snapshot.json"),
            serialized,
            true,
        )
        .await
        .unwrap();

    // Act
    let result = sut.load_cached_snapshot().await;

    // Assert - expired cache should return None
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// Tests for cache behavior (via public API)

#[actix_rt::test]
async fn test_uses_cache_when_available() {
    // Arrange
    let (xrd, _) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    let cached_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(999))],
        )],
    );

    // Client will fail if it tries to fetch liquidity receipts from network
    // But we need to provide token prices
    let token_prices = vec![sample_token_price(xrd, 1.0, FiatCurrency::USD)];

    let driver = Arc::new(MockNetworkingDriver::with_lazy_responses(
        move |request, _| {
            // Only succeed for token price requests, fail for liquidity receipts
            if request.url.as_str().contains("token-price-service") {
                let body = serde_json::to_vec(&token_prices).unwrap();
                NetworkResponse::new(200, body)
            } else {
                NetworkResponse::new(500, vec![])
            }
        },
    ));

    let http_client = Arc::new(HttpClient::new(driver));
    let file_system = Arc::new(FileSystemClient::in_memory());
    let sut = SUT::new(http_client, file_system);

    // Pre-populate cache
    let snapshot = LiquidityReceiptsSnapshot::new(
        Timestamp::now_utc(),
        vec![cached_receipt.clone()],
    );
    sut.cache_snapshot(snapshot).await.unwrap();

    // Act - this should use cached liquidity receipts and NOT call liquidity network (which would fail)
    let addresses = HashSet::from([nft_global_id.clone()]);
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert - succeeds because cache is used
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 1);
}

#[actix_rt::test]
async fn test_fetches_remote_on_cache_miss() {
    // Arrange
    let (xrd, _) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_local_id = NonFungibleLocalId::integer(1);
    let nft_global_id = sample_nft_global_id(nft_resource.clone(), nft_local_id.clone());

    let remote_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            nft_local_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100))],
        )],
    );

    let token_prices = vec![sample_token_price(xrd, 1.0, FiatCurrency::USD)];
    let sut = make_client_with_responses(vec![remote_receipt.clone()], token_prices);

    // Act - no cache, should fetch from remote
    let addresses = HashSet::from([nft_global_id]);
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result.is_ok());

    // Verify cache was populated
    let cached = sut.load_cached_snapshot().await.unwrap();
    assert!(cached.is_some());
    let cached_receipts = cached.unwrap();
    assert_eq!(cached_receipts.len(), 1);
    assert_eq!(cached_receipts[0], remote_receipt);
}

#[actix_rt::test]
async fn test_cache_miss_when_nft_not_in_cache() {
    // Arrange
    let (xrd, _) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let cached_nft_id = NonFungibleLocalId::integer(1);
    let requested_nft_id = NonFungibleLocalId::integer(2);

    let requested_global_id = sample_nft_global_id(nft_resource.clone(), requested_nft_id.clone());

    // Cache only has NFT #1
    let cached_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            cached_nft_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100))],
        )],
    );

    // Remote has NFT #2
    let remote_receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![sample_liquidity_receipt_item(
            requested_nft_id.clone(),
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(200))],
        )],
    );

    let token_prices = vec![sample_token_price(xrd, 1.0, FiatCurrency::USD)];
    let sut = make_client_with_responses(vec![remote_receipt.clone()], token_prices);

    // Populate cache with NFT #1
    let snapshot = LiquidityReceiptsSnapshot::new(
        Timestamp::now_utc(),
        vec![cached_receipt],
    );
    sut.cache_snapshot(snapshot).await.unwrap();

    // Act - request NFT #2 which is not in cache, should fetch from remote
    let addresses = HashSet::from([requested_global_id.clone()]);
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert - should fetch from remote successfully
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert!(prices.contains_key(&requested_global_id));
}

// Tests for NonFungibleLiquidityReceipt helper methods

#[test]
fn test_all_non_fungible_ids() {
    // Arrange
    let (nft_resource, _) = sample_nft_resource_addresses();
    let local_1 = NonFungibleLocalId::integer(1);
    let local_2 = NonFungibleLocalId::integer(2);

    let receipt = sample_liquidity_receipt(
        nft_resource.clone(),
        vec![
            sample_liquidity_receipt_item(local_1.clone(), vec![]),
            sample_liquidity_receipt_item(local_2.clone(), vec![]),
        ],
    );

    // Act
    let ids = receipt.all_non_fungible_ids();

    // Assert
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&sample_nft_global_id(nft_resource.clone(), local_1)));
    assert!(ids.contains(&sample_nft_global_id(nft_resource, local_2)));
}

// Tests for LiquidityReceiptsSnapshot helper methods

#[test]
fn test_snapshot_contains_all() {
    // Arrange
    let (nft_resource, _) = sample_nft_resource_addresses();
    let local_1 = NonFungibleLocalId::integer(1);
    let local_2 = NonFungibleLocalId::integer(2);
    let local_3 = NonFungibleLocalId::integer(3);

    let global_1 = sample_nft_global_id(nft_resource.clone(), local_1.clone());
    let global_2 = sample_nft_global_id(nft_resource.clone(), local_2.clone());
    let global_3 = sample_nft_global_id(nft_resource.clone(), local_3);

    let receipt = sample_liquidity_receipt(
        nft_resource,
        vec![
            sample_liquidity_receipt_item(local_1, vec![]),
            sample_liquidity_receipt_item(local_2, vec![]),
        ],
    );

    let snapshot = LiquidityReceiptsSnapshot::new(
        Timestamp::now_utc(),
        vec![receipt],
    );

    // Act & Assert - contains subset
    assert!(snapshot.contains_all(&HashSet::from([global_1.clone()])));
    assert!(snapshot.contains_all(&HashSet::from([global_2.clone()])));
    assert!(snapshot.contains_all(&HashSet::from([global_1.clone(), global_2.clone()])));

    // Act & Assert - does not contain superset
    assert!(!snapshot.contains_all(&HashSet::from([global_1, global_2, global_3])));
}

#[test]
fn test_snapshot_contains_all_empty_set() {
    // Arrange
    let snapshot = LiquidityReceiptsSnapshot::new(Timestamp::now_utc(), vec![]);

    // Act & Assert
    assert!(snapshot.contains_all(&HashSet::new()));
}

// Integration tests

#[actix_rt::test]
async fn test_full_flow_with_cache() {
    // Arrange
    let (xrd, candy) = sample_resource_addresses();
    let (nft_resource, _) = sample_nft_resource_addresses();

    let nft_1_id = NonFungibleLocalId::integer(1);
    let nft_2_id = NonFungibleLocalId::integer(2);
    let nft_1_global = sample_nft_global_id(nft_resource.clone(), nft_1_id.clone());
    let nft_2_global = sample_nft_global_id(nft_resource.clone(), nft_2_id.clone());

    let liquidity_receipt = sample_liquidity_receipt(
        nft_resource,
        vec![
            sample_liquidity_receipt_item(
                nft_1_id,
                vec![
                    sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100)),
                    sample_liquidity_receipt_item_resource(candy.clone(), Decimal192::from(50)),
                ],
            ),
            sample_liquidity_receipt_item(
                nft_2_id,
                vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(200))],
            ),
        ],
    );

    let token_prices = vec![
        sample_token_price(xrd, 1.0, FiatCurrency::USD),
        sample_token_price(candy, 2.0, FiatCurrency::USD),
    ];

    let sut = make_client_with_responses(vec![liquidity_receipt], token_prices);
    let addresses = HashSet::from([nft_1_global.clone(), nft_2_global.clone()]);

    // Act - first call should fetch from remote and cache
    let result1 = sut
        .fetch_non_fungibles_prices(addresses.clone(), FiatCurrency::USD)
        .await;

    // Assert
    assert!(result1.is_ok());
    let prices1 = result1.unwrap();
    assert_eq!(prices1.len(), 2);
    // NFT1: (100 XRD * $1.0) + (50 CANDY * $2.0) = $100 + $100 = $200
    assert_eq!(*prices1.get(&nft_1_global).unwrap(), Decimal192::from(200.0));
    // NFT2: (200 XRD * $1.0) = $200
    assert_eq!(*prices1.get(&nft_2_global).unwrap(), Decimal192::from(200.0));

    // Verify cache was populated
    let cached = sut.load_cached_snapshot().await.unwrap();
    assert!(cached.is_some());

    // Act - second call should use cache (mock only has one response)
    let result2 = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result2.is_ok());
    let prices2 = result2.unwrap();
    assert_eq!(prices2, prices1);
}

#[actix_rt::test]
async fn test_multiple_resources_in_receipt() {
    // Arrange
    let (xrd, candy) = sample_resource_addresses();
    let (nft_resource_1, nft_resource_2) = sample_nft_resource_addresses();

    let nft_1_id = NonFungibleLocalId::integer(1);
    let nft_2_id = NonFungibleLocalId::integer(2);
    let nft_1_global = sample_nft_global_id(nft_resource_1.clone(), nft_1_id.clone());
    let nft_2_global = sample_nft_global_id(nft_resource_2.clone(), nft_2_id.clone());

    // Two different NFT resources
    let liquidity_receipt_1 = sample_liquidity_receipt(
        nft_resource_1,
        vec![sample_liquidity_receipt_item(
            nft_1_id,
            vec![sample_liquidity_receipt_item_resource(xrd.clone(), Decimal192::from(100))],
        )],
    );

    let liquidity_receipt_2 = sample_liquidity_receipt(
        nft_resource_2,
        vec![sample_liquidity_receipt_item(
            nft_2_id,
            vec![sample_liquidity_receipt_item_resource(candy.clone(), Decimal192::from(50))],
        )],
    );

    let token_prices = vec![
        sample_token_price(xrd, 1.0, FiatCurrency::USD),
        sample_token_price(candy, 2.0, FiatCurrency::USD),
    ];

    let sut = make_client_with_responses(
        vec![liquidity_receipt_1, liquidity_receipt_2],
        token_prices,
    );
    let addresses = HashSet::from([nft_1_global.clone(), nft_2_global.clone()]);

    // Act
    let result = sut
        .fetch_non_fungibles_prices(addresses, FiatCurrency::USD)
        .await;

    // Assert
    assert!(result.is_ok());
    let prices = result.unwrap();
    assert_eq!(prices.len(), 2);
    // NFT1: 100 XRD * $1.0 = $100
    assert_eq!(*prices.get(&nft_1_global).unwrap(), Decimal192::from(100.0));
    // NFT2: 50 CANDY * $2.0 = $100
    assert_eq!(*prices.get(&nft_2_global).unwrap(), Decimal192::from(100.0));
}
