use crate::prelude::*;

/// A mapping from NFT global IDs to their calculated fiat values.
pub type NonFungibleTokenPricesTable = HashMap<NonFungibleGlobalId, Decimal192>;

/// Client for fetching and calculating NFT prices based on their liquidity receipts.
///
/// This client calculates the value of NFTs (specifically liquidity pool position NFTs) by:
/// 1. Fetching liquidity receipt data that describes the fungible tokens backing the NFT
/// 2. Fetching current prices for those fungible tokens
/// 3. Calculating the total value: sum of (token_amount × token_price) for each token
///
/// # Caching Strategy
///
/// Liquidity receipts are cached for 5 minutes to minimize API calls. The client uses a cache-first
/// strategy similar to `FungiblesPricesClient`.
///
/// # Example
///
/// ```ignore
/// let client = NonFungiblePricesClient::new(http_client, file_system);
///
/// let nft_ids = HashSet::from([nft_global_id1, nft_global_id2]);
/// let prices = client
///     .fetch_non_fungibles_prices(nft_ids, FiatCurrency::USD)
///     .await?;
///
/// for (nft_id, value) in prices {
///     println!("NFT {}: ${}", nft_id, value);
/// }
/// ```
pub struct NonFungiblePricesClient {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) file_system_client: Arc<FileSystemClient>,
    pub(crate) fungibles_prices_client: FungiblesPricesClient,
}

impl NonFungiblePricesClient {
    pub fn new(
        http_client: Arc<HttpClient>,
        file_system_client: Arc<FileSystemClient>,
    ) -> Self {
        Self {
            http_client: http_client.clone(),
            file_system_client: file_system_client.clone(),
            fungibles_prices_client: FungiblesPricesClient::new(
                http_client,
                file_system_client,
            ),
        }
    }
}

impl NonFungiblePricesClient {
    /// Fetches and calculates the fiat value of NFTs based on their liquidity receipts.
    ///
    /// This method:
    /// 1. Fetches liquidity receipt data for the requested NFTs (with caching)
    /// 2. Fetches current fungible token prices in the requested currency
    /// 3. Calculates each NFT's value as: Σ(token_amount × token_price) for all tokens in the receipt
    ///
    /// # Arguments
    ///
    /// * `addresses` - Set of NFT global IDs to calculate prices for
    /// * `currency` - The fiat currency to calculate values in (e.g., USD, SEK)
    ///
    /// # Returns
    ///
    /// A `HashMap` mapping each NFT's `NonFungibleGlobalId` to its calculated `Decimal192` value
    /// in the requested currency. NFTs with no fungible tokens or unavailable price data will
    /// have a value of zero.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Network requests fail
    /// * Response data cannot be deserialized
    /// * Cache operations fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let nft_ids = HashSet::from([
    ///     NonFungibleGlobalId::from_str("resource_...:1")?,
    ///     NonFungibleGlobalId::from_str("resource_...:2")?,
    /// ]);
    ///
    /// let prices = client
    ///     .fetch_nft_fiat_values(nft_ids, FiatCurrency::USD)
    ///     .await?;
    ///
    /// // NFT value is sum of all token values
    /// // E.g., if NFT contains 100 XRD ($1 each) + 50 CANDY ($2 each):
    /// // value = (100 * $1) + (50 * $2) = $200
    /// ```
    pub async fn fetch_nft_fiat_values(
        &self,
        state_version: u64,
        addresses: HashSet<NonFungibleGlobalId>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<NonFungibleTokenPricesTable> {
        let liquidity_receipts = self
            .fetch_liquidity_receipts(state_version, addresses, force_fetch)
            .await?;
        let fungible_prices = self
            .fungibles_prices_client
            .get_prices_for_currency(currency)
            .await?;

        let mut value_table = NonFungibleTokenPricesTable::new();

        for liquidity_receipt in liquidity_receipts {
            for liquidity_receipt_item in liquidity_receipt.items {
                let mut item_value: Decimal192 = Decimal192::zero();
                for resource in liquidity_receipt_item.resources {
                    if let Some(price) = fungible_prices.get(&resource.address)
                    {
                        // Calculate value by multiplying token price by amount
                        let resource_value = *price * resource.amount;
                        item_value = item_value + resource_value;
                    }
                }
                // Convert resource address to NFT resource address safely
                let nft_resource_address = NonFungibleResourceAddress::new(
                    liquidity_receipt.resource_manager_address,
                )?;
                let global_id = NonFungibleGlobalId::new(
                    nft_resource_address,
                    liquidity_receipt_item.local_id,
                );
                value_table.insert(global_id, item_value);
            }
        }
        Ok(value_table)
    }

    async fn fetch_liquidity_receipts(
        &self,
        state_version: u64,
        addresses: HashSet<NonFungibleGlobalId>,
        force_fetch: bool,
    ) -> Result<Vec<NonFungibleLiquidityReceipt>> {
        if !force_fetch {
            let cached_snapshot =
                self.load_cached_snapshot().await.ok().flatten();

            if let Some(cached_receipts) = cached_snapshot {
                // Collect all NFT IDs from all cached receipts
                let all_cached_ids: HashSet<_> = cached_receipts
                    .iter()
                    .flat_map(|receipt| receipt.all_non_fungible_ids())
                    .collect();

                // Check if cache contains all requested NFT IDs
                let all_non_fungibles_in_cache =
                    all_cached_ids.is_superset(&addresses);
                if all_non_fungibles_in_cache {
                    return Ok(cached_receipts);
                }
            };
        }

        let remote_receipts = self
            .fetch_remote_liquidity_receipts(state_version, addresses)
            .await?;
        let new_cache_snapshot = LiquidityReceiptsSnapshot::new(
            Timestamp::now_utc(),
            remote_receipts.clone(),
        );

        _ = self.cache_snapshot(new_cache_snapshot).await;
        Ok(remote_receipts)
    }
}

/// Represents liquidity receipt data for a collection of NFTs from the same resource.
///
/// A liquidity receipt describes the fungible tokens that back one or more liquidity
/// position NFTs. Each NFT in the collection is represented by a `LiquidityReceiptItem`.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NonFungibleLiquidityReceipt {
    /// The resource address of the NFT collection
    pub resource_manager_address: ResourceAddress,
    /// Individual liquidity receipt items, one per NFT in the response
    pub items: Vec<LiquidityReceiptItem>,
}

impl NonFungibleLiquidityReceipt {
    pub fn all_non_fungible_ids(&self) -> HashSet<NonFungibleGlobalId> {
        self.items
            .iter()
            .map(|item| {
                NonFungibleGlobalId::new(
                    self.resource_manager_address.try_into().unwrap(),
                    item.local_id.clone(),
                )
            })
            .collect()
    }
}

/// Represents an individual NFT's liquidity receipt data.
///
/// Contains the NFT's local ID and the list of fungible tokens (with amounts) that
/// back this specific NFT.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityReceiptItem {
    /// The local ID of the NFT within its collection
    pub local_id: NonFungibleLocalId,
    /// The fungible token resources and their amounts backing this NFT
    pub resources: Vec<LiquidityReceiptItemResource>,
}

/// Represents a fungible token resource and amount within a liquidity receipt.
///
/// Used to describe how much of each token backs an NFT position.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LiquidityReceiptItemResource {
    /// The address of the fungible token resource
    pub address: ResourceAddress,
    /// The amount of this token backing the NFT
    pub amount: Decimal192,
}
