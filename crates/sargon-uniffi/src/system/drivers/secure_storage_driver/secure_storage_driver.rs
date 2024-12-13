use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::Result as InternalResult;
use sargon::SecureStorageDriver as InternalSecureStorageDriver;
use sargon::SecureStorageKey as InternalSecureStorageKey;

#[uniffi::export(with_foreign)]
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

    async fn contains_data_for_key(
        &self,
        key: SecureStorageKey,
    ) -> Result<bool>;
}

#[derive(Debug)]
pub struct SecureStorageDriverAdapter {
    pub wrapped: Arc<dyn SecureStorageDriver>,
}

#[async_trait::async_trait]
impl InternalSecureStorageDriver for SecureStorageDriverAdapter {
    async fn load_data(
        &self,
        key: InternalSecureStorageKey,
    ) -> InternalResult<Option<InternalBagOfBytes>> {
        self.wrapped
            .load_data(key.into())
            .await
            .into_internal_result()
    }

    async fn save_data(
        &self,
        key: InternalSecureStorageKey,
        data: InternalBagOfBytes,
    ) -> InternalResult<()> {
        self.wrapped
            .save_data(key.into(), data.into())
            .await
            .into_internal_result()
    }

    async fn delete_data_for_key(
        &self,
        key: InternalSecureStorageKey,
    ) -> InternalResult<()> {
        self.wrapped
            .delete_data_for_key(key.into())
            .await
            .into_internal_result()
    }

    async fn contains_data_for_key(
        &self,
        key: InternalSecureStorageKey,
    ) -> InternalResult<bool> {
        self.wrapped
            .contains_data_for_key(key.into())
            .await
            .into_internal_result()
    }
}
