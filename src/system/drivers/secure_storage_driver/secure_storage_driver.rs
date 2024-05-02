use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait SecureStorageDriver: Send + Sync + std::fmt::Debug {
    async fn load_data(&self, key: SecureStorageKey)
        -> Result<Option<Vec<u8>>>;

    async fn save_data(
        &self,
        key: SecureStorageKey,
        data: Vec<u8>,
    ) -> Result<()>;

    async fn delete_data_for_key(&self, key: SecureStorageKey) -> Result<()>;
}
