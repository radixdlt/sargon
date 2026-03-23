use crate::prelude::*;

/// A mapping from resource addresses to their prices in a specific fiat currency.
pub type PerTokenPrices = HashMap<ResourceAddress, Decimal192>;

/// Request payload for scoped fungible and LSU price lookup.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FungiblePricesRequest {
    pub currency: FiatCurrency,
    pub tokens: Vec<ResourceAddress>,
    pub lsus: Vec<ResourceAddress>,
}

impl FungiblePricesRequest {
    pub fn new(
        currency: FiatCurrency,
        tokens: impl IntoIterator<Item = ResourceAddress>,
        lsus: impl IntoIterator<Item = ResourceAddress>,
    ) -> Self {
        Self {
            currency,
            tokens: Self::normalize_addresses(tokens),
            lsus: Self::normalize_addresses(lsus),
        }
    }

    fn normalize_addresses(
        addresses: impl IntoIterator<Item = ResourceAddress>,
    ) -> Vec<ResourceAddress> {
        let mut unique: Vec<_> =
            HashSet::<ResourceAddress>::from_iter(addresses)
                .into_iter()
                .collect();
        unique.sort_by_key(|address| address.to_string());
        unique
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScopedTokenPricesResponse {
    pub tokens: Vec<ScopedTokenPrice>,
    pub lsus: Vec<ScopedLsuPrice>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct ScopedTokenPrice {
    pub resource_address: ResourceAddress,
    pub usd_price: f64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct ScopedLsuPrice {
    pub resource_address: ResourceAddress,
    pub usd_price: f64,
}

impl ScopedTokenPricesResponse {
    pub fn into_prices_map(self) -> PerTokenPrices {
        let mut prices: PerTokenPrices = HashMap::new();

        for token in self.tokens {
            insert_price(&mut prices, token.resource_address, token.usd_price);
        }

        // In case a resource appears in both arrays, LSU value wins.
        for lsu in self.lsus {
            insert_price(&mut prices, lsu.resource_address, lsu.usd_price);
        }

        prices
    }
}

fn insert_price(
    prices: &mut PerTokenPrices,
    resource_address: ResourceAddress,
    value: f64,
) {
    if let Ok(decimal) = Decimal192::try_from(value) {
        prices.insert(resource_address, decimal);
    }
}

/// Client for fetching and caching scoped fungible token prices.
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
    /// Fetches token prices for a scoped request.
    ///
    /// Uses a 5-minute cache keyed by (currency, tokens, lsus).
    pub async fn get_prices_for_request(
        &self,
        request: FungiblePricesRequest,
        force_fetch: bool,
    ) -> Result<PerTokenPrices> {
        self.get_prices_for_request_using_token_price_services(
            request,
            TokenPriceServices::default(),
            force_fetch,
        )
        .await
    }

    /// Fetches token prices for a scoped request using an ordered list of
    /// token price services for automatic failover.
    pub async fn get_prices_for_request_using_token_price_services(
        &self,
        request: FungiblePricesRequest,
        token_price_services: TokenPriceServices,
        force_fetch: bool,
    ) -> Result<PerTokenPrices> {
        if !force_fetch {
            let cached_prices =
                self.load_cached_prices(&request).await.ok().flatten();
            if let Some(cached_prices) = cached_prices {
                return Ok(cached_prices);
            }
        }

        let remote_prices = if token_price_services.is_default() {
            self.fetch_remote_token_prices(&request).await?
        } else {
            self.fetch_remote_token_prices_using_token_price_services(
                &request,
                token_price_services,
            )
            .await?
        };
        _ = self.store_prices(&request, &remote_prices).await;
        Ok(remote_prices)
    }
}
