use crate::prelude::*;
use sargon::FileSystemDriver as InternalFileSystemDriver;
use sargon::BagOfBytes as InternalBagOfBytes;
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
    async fn load_from_file(&self, path: String) -> InternalResult<Option<InternalBagOfBytes>> {
        map_result_to_internal_optional(self.wrapped.load_from_file(path).await)
    }

    async fn save_to_file(&self, path: String, data: InternalBagOfBytes) -> InternalResult<()> {
        map_result_to_internal(self.wrapped.save_to_file(path, data.into()).await)
    }

    async fn delete_file(&self, path: String) -> InternalResult<()> {
        map_result_to_internal(self.wrapped.delete_file(path).await)
    }
}

fn map_result_to_internal<T, InternalT>(result: Result<T>) -> InternalResult<InternalT>
where
    T: Into<InternalT>,
{
    result.map_err(CommonError::into).map(T::into)
}

fn map_result_to_internal_optional<T, InternalT>(result: Result<Option<T>>) -> InternalResult<Option<InternalT>>
where
    T: Into<InternalT>,
{
    result.map_err(CommonError::into).map(|opt| opt.map(T::into))
}