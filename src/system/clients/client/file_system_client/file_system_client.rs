use crate::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct FileSystemClient {
    driver: Arc<dyn FileSystemDriver>,
}

impl FileSystemClient {
    pub(crate) fn new(driver: Arc<dyn FileSystemDriver>) -> Self {
        Self { driver }
    }
}

pub(crate) fn path_to_string(path: &Path) -> Result<String> {
    path.to_str()
        .ok_or(CommonError::Unknown)
        .map(|s| s.to_owned())
}

impl FileSystemClient {
    async fn load_from_file(&self, path: &Path) -> Result<Option<BagOfBytes>> {
        let path = path_to_string(path)?;
        self.driver.load_from_file(path).await
    }

    async fn save_to_file(
        &self,
        path: &Path,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        let path = path_to_string(path)?;
        self.driver
            .save_to_file(path, BagOfBytes::from(data.as_ref()))
            .await
    }

    async fn delete_file(&self, path: &Path) -> Result<()> {
        let path = path_to_string(path)?;
        self.driver.delete_file(path).await
    }
}
