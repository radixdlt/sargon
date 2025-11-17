use crate::prelude::*;
use std::path::Path;

/// File name for the cached token prices snapshot
const ALL_TOKEN_PRICES_PATH: &str = "all_token_prices.json";

/// Cache time-to-live in seconds (5 minutes)
///
/// Token prices are considered fresh for 5 minutes before requiring a refresh
/// from the remote API. This balances data freshness with API rate limiting.
pub const CACHE_TTL_SECONDS: i64 = 5 * 60;

impl FungiblesPricesClient {
    pub async fn load_cached_prices(&self) -> Result<Option<Vec<TokenPrice>>> {
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(ALL_TOKEN_PRICES_PATH))
            .await?;

        if let Some(bytes) = bytes {
            let snapshot: AllTokenPricesSnapshot = bytes.deserialize()?;
            let now = Timestamp::now_utc();

            let age = now.duration_since(snapshot.fetched_at);
            if age.whole_seconds() <= CACHE_TTL_SECONDS {
                return Ok(Some(snapshot.prices));
            }
        };

        Ok(None)
    }

    pub async fn store_prices(&self, prices: Vec<TokenPrice>) -> Result<()> {
        let snapshot =
            AllTokenPricesSnapshot::new(Timestamp::now_utc(), prices);

        let serialized = snapshot.serialize_to_bytes()?;

        self.file_system_client
            .save_to_file(Path::new(ALL_TOKEN_PRICES_PATH), serialized, true)
            .await
    }
}

/// A cached snapshot of all token prices with timestamp for TTL validation.
///
/// This structure is serialized and stored in the file system to provide caching
/// for token price data. The cache is considered valid for 5 minutes.
#[derive(Deserialize, Serialize)]
pub struct AllTokenPricesSnapshot {
    /// When this snapshot was created (UTC timestamp)
    pub fetched_at: Timestamp,
    /// The cached token prices across all currencies
    pub prices: Vec<TokenPrice>,
}

impl AllTokenPricesSnapshot {
    fn new(fetched_at: Timestamp, prices: Vec<TokenPrice>) -> Self {
        Self { fetched_at, prices }
    }
}
