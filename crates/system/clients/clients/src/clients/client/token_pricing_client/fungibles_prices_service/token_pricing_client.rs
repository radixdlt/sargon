use crate::prelude::*;

/// A mapping from resource addresses to their prices in a specific fiat currency.
pub type PerTokenPrices = HashMap<ResourceAddress, Decimal192>;

/// Client for fetching and caching fungible token prices.
///
/// This client provides access to real-time token pricing data from the Radix token price service.
/// It implements a cache-first strategy with a 5-minute TTL to minimize network requests and
/// improve performance.
///
/// # Caching Strategy
///
/// 1. When prices are requested, the client first checks the local file system cache
/// 2. If valid cached prices exist (less than 5 minutes old), they are returned immediately
/// 3. If cache is expired or missing, prices are fetched from the remote API
/// 4. Successful remote fetches are automatically cached for future requests
///
/// # Example
///
/// ```ignore
/// let http_client = Arc::new(HttpClient::new(driver));
/// let file_system = Arc::new(FileSystemClient::new(fs_driver));
/// let client = FungiblesPricesClient::new(http_client, file_system);
///
/// // Fetch USD prices for all tokens
/// let prices = client.get_prices_for_currency(FiatCurrency::USD).await?;
/// ```
pub struct FungiblesPricesClient {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) file_system_client: Arc<FileSystemClient>,
}

impl FungiblesPricesClient {
    pub fn new(
        http_client: Arc<HttpClient>,
        file_system_client: Arc<FileSystemClient>,
    ) -> Self {
        Self {
            http_client,
            file_system_client,
        }
    }
}

impl FungiblesPricesClient {
    /// Fetches token prices for a specific fiat currency.
    ///
    /// This method returns a mapping of resource addresses to their prices in the requested
    /// fiat currency. It uses a cache-first strategy with automatic fallback to remote fetching.
    ///
    /// # Arguments
    ///
    /// * `currency` - The fiat currency to get prices in (e.g., USD, SEK)
    ///
    /// # Returns
    ///
    /// A `HashMap` mapping `ResourceAddress` to `Decimal192` price values in the requested currency.
    /// Only tokens with prices in the requested currency are included in the result.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The cache is invalid and remote fetch fails
    /// * Network request fails
    /// * Response cannot be deserialized
    ///
    /// # Example
    ///
    /// ```ignore
    /// let prices = client.get_prices_for_currency(FiatCurrency::USD).await?;
    /// for (resource_addr, price) in prices {
    ///     println!("{}: ${}", resource_addr, price);
    /// }
    /// ```
    pub async fn get_prices_for_currency(
        &self,
        currency: FiatCurrency,
    ) -> Result<PerTokenPrices> {
        let prices = self.get_token_prices().await?;
        let mut per_token_prices: HashMap<ResourceAddress, Decimal192> =
            HashMap::new();

        for token_price in prices {
            if token_price.currency == currency {
                per_token_prices.insert(
                    token_price.resource_address,
                    token_price.price.into(),
                );
            }
        }

        Ok(per_token_prices)
    }

    async fn get_token_prices(&self) -> Result<Vec<TokenPrice>> {
        let cached_prices = self.load_cached_prices().await.ok().flatten();

        if let Some(cached_prices) = cached_prices {
            return Ok(cached_prices);
        }

        let remote_prices = self.fetch_remote_token_prices().await;

        if let Ok(remote_prices) = remote_prices.clone() {
            _ = self.store_prices(remote_prices).await;
        }

        remote_prices
    }
}
