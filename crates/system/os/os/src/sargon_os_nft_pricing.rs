use crate::prelude::*;

impl SargonOS {
    /// Fetches and calculates the fiat value of NFTs (liquidity position NFTs) based on
    /// their underlying token composition.
    ///
    /// This method:
    /// 1. Fetches liquidity receipt data describing the fungible tokens backing each NFT
    /// 2. Fetches current market prices for those fungible tokens
    /// 3. Calculates total value as: Σ(token_amount × token_price) for each token
    ///
    /// The calculation uses a cache-first strategy with 5-minute TTL for both liquidity
    /// receipts and token prices to minimize API calls.
    ///
    /// # Arguments
    ///
    /// * `nft_ids` - Set of NFT global IDs to calculate prices for
    /// * `currency` - The fiat currency to calculate values in (USD, SEK, etc.)
    ///
    /// # Returns
    ///
    /// A `HashMap` mapping each NFT's `NonFungibleGlobalId` to its calculated `Decimal192`
    /// value in the requested currency. NFTs with no backing tokens or unavailable price
    /// data will have a value of zero.
    ///
    /// # Example
    ///
    /// If an NFT contains:
    /// - 100 XRD @ $1.00 each
    /// - 50 CANDY @ $2.00 each
    ///
    /// Then: NFT Value = (100 × $1.00) + (50 × $2.00) = $200
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Network requests fail
    /// * Response data cannot be deserialized
    /// * Cache operations fail
    pub async fn fetch_nft_prices(
        &self,
        nft_ids: HashSet<NonFungibleGlobalId>,
        currency: FiatCurrency,
    ) -> Result<HashMap<NonFungibleGlobalId, Decimal192>> {
        info!(
            "Fetching NFT prices for {} NFTs in {:?}",
            nft_ids.len(),
            currency
        );

        let result = self
            .nft_prices_client
            .fetch_non_fungibles_prices(nft_ids, currency)
            .await;

        match &result {
            Ok(prices) => {
                info!("Successfully fetched prices for {} NFTs", prices.len());
            }
            Err(e) => {
                error!("Failed to fetch NFT prices: {:?}", e);
            }
        }

        result
    }
}
