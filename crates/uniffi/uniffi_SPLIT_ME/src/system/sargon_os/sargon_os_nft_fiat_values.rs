use crate::prelude::*;
use sargon::SargonOS as InternalSargonOS;

#[uniffi::export]
impl SargonOS {
    /// Fetches and calculates the fiat value of NFTs (liquidity position NFTs).
    ///
    /// This method calculates NFT values by:
    /// 1. Fetching liquidity receipt data (cached for 5 minutes)
    /// 2. Fetching current token prices (cached for 5 minutes)
    /// 3. Computing: NFT Value = Σ(token_amount × token_price)
    ///
    /// # Example
    ///
    /// If an NFT contains 100 XRD ($1 each) + 50 CANDY ($2 each):
    /// NFT Value = (100 × $1) + (50 × $2) = $200
    ///
    /// # Arguments
    ///
    /// * `nft_ids` - Vector of NFT global IDs to price
    /// * `currency` - Fiat currency for valuation (USD, SEK, etc.)
    ///
    /// # Returns
    ///
    /// HashMap of NFT IDs to their calculated fiat values.
    /// NFTs with no backing tokens or missing prices will have value 0.
    pub async fn fetch_nft_fiat_values(
        &self,
        nft_ids: Vec<NonFungibleGlobalId>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<HashMap<NonFungibleGlobalId, Decimal192>> {
        let nft_ids_set: HashSet<_> =
            nft_ids.into_iter().map(|id| id.into_internal()).collect();

        let result = self
            .wrapped
            .fetch_nft_fiat_values(
                nft_ids_set,
                currency.into_internal(),
                force_fetch,
            )
            .await?;
        if result.is_empty() {
            panic!("Empty result from wrapped sargon")
        }
        Ok(result.into_hash_map())
    }
}
