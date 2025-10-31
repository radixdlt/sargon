use crate::prelude::*;
use std::path::Path;

/// File name for the cached liquidity receipts snapshot
const FILE_PATH: &str = "liquidity_receipts_snapshot.json";

/// Cache time-to-live in seconds (5 minutes)
///
/// Liquidity receipt data is considered fresh for 5 minutes before requiring a refresh
/// from the remote API. This balances data freshness with API rate limiting.
pub const CACHE_TTL_SECONDS: i64 = 5 * 60;

impl NonFungiblePricesClient {
    pub async fn load_cached_snapshot(
        &self,
    ) -> Result<Option<Vec<NonFungibleLiquidityReceipt>>> {
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(FILE_PATH))
            .await?;

        if let Some(bytes) = bytes {
            let snapshot: LiquidityReceiptsSnapshot = bytes.deserialize()?;
            let now = Timestamp::now_utc();

            // Cache is valid if:
            // 1. It was fetched in the past (not from the future)
            // 2. It's not older than the TTL
            if snapshot.fetched_at <= now {
                let age = now.duration_since(snapshot.fetched_at);
                if age.whole_seconds() <= CACHE_TTL_SECONDS {
                    return Ok(Some(snapshot.receipts));
                }
            }
        };

        return Ok(None)
    }

    pub async fn cache_snapshot(
        &self,
        snapshot: LiquidityReceiptsSnapshot,
    ) -> Result<()> {
        let serialized = snapshot.serialize_to_bytes()?;

        self.file_system_client
            .save_to_file(Path::new(FILE_PATH), serialized, true)
            .await
    }
}


/// A cached snapshot of liquidity receipts with timestamp for TTL validation.
///
/// This structure is serialized and stored in the file system to provide caching
/// for liquidity receipt data. The cache is considered valid for 5 minutes.
#[derive(Deserialize, Serialize)]
pub struct LiquidityReceiptsSnapshot {
    /// When this snapshot was created (UTC timestamp)
    pub fetched_at: Timestamp,
    /// The cached liquidity receipts
    pub receipts: Vec<NonFungibleLiquidityReceipt>,
}

impl LiquidityReceiptsSnapshot {
    pub fn new(
        fetched_at: Timestamp,
        receipts: Vec<NonFungibleLiquidityReceipt>,
    ) -> Self {
        Self {
            fetched_at,
            receipts,
        }
    }
}

impl LiquidityReceiptsSnapshot {
    pub fn contains_all(&self, nft_ids: &HashSet<NonFungibleGlobalId>) -> bool {
        let all_ids_in_cache: HashSet<NonFungibleGlobalId> = self
            .receipts
            .iter()
            .flat_map(|receipt| receipt.all_non_fungible_ids())
            .collect();

        all_ids_in_cache.is_superset(nft_ids)
    }
}
