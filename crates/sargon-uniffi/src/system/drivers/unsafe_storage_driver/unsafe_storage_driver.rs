use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::Result as InternalResult;
use sargon::UnsafeStorageDriver as InternalUnsafeStorageDriver;
use sargon::UnsafeStorageKey as InternalUnsafeStorageKey;

#[uniffi::export(with_foreign)]
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

#[derive(Debug)]
pub struct UnsafeStorageDriverAdapter {
    pub wrapped: Arc<dyn UnsafeStorageDriver>,
}

#[async_trait::async_trait]
impl InternalUnsafeStorageDriver for UnsafeStorageDriverAdapter {
    async fn load_data(
        &self,
        key: InternalUnsafeStorageKey,
    ) -> InternalResult<Option<InternalBagOfBytes>> {
            self.wrapped.load_data(key.into()).await.into_internal_result()
    }

    async fn save_data(
        &self,
        key: InternalUnsafeStorageKey,
        data: InternalBagOfBytes,
    ) -> InternalResult<()> {
            self.wrapped.save_data(key.into(), data.into()).await.into_internal_result()
    }

    async fn delete_data_for_key(
        &self,
        key: InternalUnsafeStorageKey,
    ) -> InternalResult<()> {
            self.wrapped.delete_data_for_key(key.into()).await.into_internal_result()
    }
}
