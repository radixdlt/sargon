use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::FileSystemDriver as InternalFileSystemDriver;
use sargon::Result as InternalResult;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait FileSystemDriver: Send + Sync + std::fmt::Debug {
    async fn load_from_file(&self, path: String) -> Result<Option<BagOfBytes>>;

    async fn save_to_file(&self, path: String, data: BagOfBytes) -> Result<()>;

    async fn delete_file(&self, path: String) -> Result<()>;
}

#[derive(Debug)]
pub struct FileSystemDriverAdapter {
    pub wrapped: Arc<dyn FileSystemDriver>,
}

#[async_trait::async_trait]
impl InternalFileSystemDriver for FileSystemDriverAdapter {
    async fn load_from_file(
        &self,
        path: String,
    ) -> InternalResult<Option<InternalBagOfBytes>> {
        self.wrapped
            .load_from_file(path)
            .await
            .into_internal_result()
    }

    async fn save_to_file(
        &self,
        path: String,
        data: InternalBagOfBytes,
    ) -> InternalResult<()> {
        self.wrapped
            .save_to_file(path, data.into())
            .await
            .into_internal_result()
    }

    async fn delete_file(&self, path: String) -> InternalResult<()> {
        self.wrapped.delete_file(path).await.into_internal_result()
    }
}
