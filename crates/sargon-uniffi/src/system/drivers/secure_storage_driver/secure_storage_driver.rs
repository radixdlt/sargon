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
        map_result_to_internal_optional(
            self.wrapped.load_data(key.into()).await,
        )
    }

    async fn save_data(
        &self,
        key: InternalSecureStorageKey,
        data: InternalBagOfBytes,
    ) -> InternalResult<()> {
        map_result_to_internal(
            self.wrapped.save_data(key.into(), data.into()).await,
        )
    }

    async fn delete_data_for_key(
        &self,
        key: InternalSecureStorageKey,
    ) -> InternalResult<()> {
        map_result_to_internal(
            self.wrapped.delete_data_for_key(key.into()).await,
        )
    }
}
