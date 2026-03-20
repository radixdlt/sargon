use crate::prelude::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
};

/// Prefix for request-scoped token price cache files.
const SCOPED_TOKEN_PRICES_PATH_PREFIX: &str = "scoped_token_prices";

/// Cache time-to-live in seconds (5 minutes).
pub const CACHE_TTL_SECONDS: i64 = 5 * 60;

impl FungiblesPricesClient {
    pub async fn load_cached_prices(
        &self,
        request: &FungiblePricesRequest,
    ) -> Result<Option<PerTokenPrices>> {
        let path = Self::cache_file_path(request);
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(&path))
            .await?;

        if let Some(bytes) = bytes {
            let snapshot: ScopedTokenPricesSnapshot = bytes.deserialize()?;
            let now = Timestamp::now_utc();

            let age = now.duration_since(snapshot.fetched_at);
            if age.whole_seconds() <= CACHE_TTL_SECONDS {
                return Ok(Some(snapshot.prices));
            }
        };

        Ok(None)
    }

    pub async fn store_prices(
        &self,
        request: &FungiblePricesRequest,
        prices: &PerTokenPrices,
    ) -> Result<()> {
        let snapshot = ScopedTokenPricesSnapshot::new(
            Timestamp::now_utc(),
            prices.clone(),
        );

        let serialized = snapshot.serialize_to_bytes()?;
        let path = Self::cache_file_path(request);

        self.file_system_client
            .save_to_file(Path::new(&path), serialized, true)
            .await
    }

    fn cache_file_path(request: &FungiblePricesRequest) -> String {
        let mut hasher = DefaultHasher::new();
        request.hash(&mut hasher);
        let hash = hasher.finish();

        format!("{}_{}.json", SCOPED_TOKEN_PRICES_PATH_PREFIX, hash)
    }
}

/// A cached snapshot of scoped prices with timestamp for TTL validation.
#[derive(Deserialize, Serialize)]
pub struct ScopedTokenPricesSnapshot {
    pub fetched_at: Timestamp,
    pub prices: PerTokenPrices,
}

impl ScopedTokenPricesSnapshot {
    fn new(fetched_at: Timestamp, prices: PerTokenPrices) -> Self {
        Self { fetched_at, prices }
    }
}
