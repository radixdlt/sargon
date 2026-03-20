use crate::prelude::*;

impl SargonOS {
    /// Fetches scoped fiat prices for fungible tokens and LSUs.
    ///
    /// The request is scoped by the provided sets of resource addresses and
    /// cached for 5 minutes.
    pub async fn fetch_fungible_fiat_values(
        &self,
        tokens: HashSet<ResourceAddress>,
        lsus: HashSet<ResourceAddress>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<HashMap<ResourceAddress, Decimal192>> {
        self.nft_prices_client
            .fetch_fungible_fiat_values(tokens, lsus, currency, force_fetch)
            .await
    }
}
