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
    pub async fn fetch_nft_fiat_values(
        &self,
        nft_ids: HashSet<NonFungibleGlobalId>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<HashMap<NonFungibleGlobalId, Decimal192>> {
        let gateway_client = self.gateway_client()?;
        let state_version = gateway_client
            .gateway_status()
            .await?
            .ledger_state
            .state_version;

        self.nft_prices_client
            .fetch_nft_fiat_values(
                state_version,
                nft_ids,
                currency,
                force_fetch,
            )
            .await
    }
}
