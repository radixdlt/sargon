use crate::prelude::*;

#[async_trait::async_trait]
pub trait FileSystemDriver: Send + Sync + std::fmt::Debug {
    async fn writable_app_dir_path(&self) -> Result<String>;
    async fn load_from_file(&self, path: String) -> Result<Option<BagOfBytes>>;

    async fn save_to_file(&self, path: String, data: BagOfBytes) -> Result<()>;

    async fn delete_file(&self, path: String) -> Result<()>;
}
