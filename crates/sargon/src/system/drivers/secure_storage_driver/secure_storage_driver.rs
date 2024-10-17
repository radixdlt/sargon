use crate::prelude::*;

#[async_trait::async_trait]
pub trait SecureStorageDriver: Send + Sync + std::fmt::Debug {
    async fn load_data(
        &self,
        key: SecureStorageKey,
    ) -> Result<Option<BagOfBytes>>;

    async fn save_data(
        &self,
        key: SecureStorageKey,
        data: BagOfBytes,
    ) -> Result<()>;

    async fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()>;
}
