use crate::prelude::*;

#[derive(Debug)]
pub struct FactorInstancesCacheClient {
    file_system_client: Arc<FileSystemClient>,
}

impl FactorInstancesCacheClient {
    const CACHE_FILE: &'static str = "factor_instances_cache.json";
    pub fn new(file_system_client: Arc<FileSystemClient>) -> Self {
        Self { file_system_client }
    }

    async fn apa(&self) -> Result<()> {
        self.file_system_client
            .load_from_file(Self::CACHE_FILE)
            .await?;
        Ok(())
    }
}
