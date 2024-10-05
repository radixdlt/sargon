use crate::prelude::*;

#[async_trait::async_trait]
pub trait UnsafeStorageDriver: Send + Sync + std::fmt::Debug {
    async fn load_data(
        &self,
        key: UnsafeStorageKey,
    ) -> Result<Option<BagOfBytes>>;

    async fn save_data(
        &self,
        key: UnsafeStorageKey,
        data: BagOfBytes,
    ) -> Result<()>;

    async fn delete_data_for_key(&self, key: UnsafeStorageKey) -> Result<()>;
}
