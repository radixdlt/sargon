use crate::prelude::*;

#[derive(Debug)]
pub struct InMemoryFileSystemDriver {
    in_memory_data: RwLock<IndexMap<String, BagOfBytes>>,
}

impl InMemoryFileSystemDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(InMemoryFileSystemDriver {
            in_memory_data: RwLock::new(IndexMap::new()),
        })
    }
}

#[async_trait::async_trait]
impl FileSystemDriver for InMemoryFileSystemDriver {
    async fn writable_app_dir_path(&self) -> Result<String> {
        Ok(RustFileSystemDriver::tmp_dir()
            .to_string_lossy()
            .to_string())
    }

    async fn load_from_file(&self, path: String) -> Result<Option<BagOfBytes>> {
        let data = self.in_memory_data.read().unwrap();
        Ok(data.get(&path).cloned())
    }

    async fn save_to_file(
        &self,
        path: String,
        data: BagOfBytes,
        is_allowed_to_overwrite: bool,
    ) -> Result<()> {
        let mut binding = self.in_memory_data.write().unwrap();
        if !is_allowed_to_overwrite && binding.contains_key(&path) {
            return Err(CommonError::FileAlreadyExists { path });
        }
        binding.insert(path, data);
        Ok(())
    }

    async fn delete_file(&self, path: String) -> Result<()> {
        let mut data = self.in_memory_data.write().unwrap();
        data.shift_remove(&path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InMemoryFileSystemDriver;

    #[actix_rt::test]
    async fn test_create_load_delete() {
        let sut = SUT::new();
        let file = "dummy".to_owned();

        let data = BagOfBytes::sample();
        sut.save_to_file(file.clone(), data.clone(), true)
            .await
            .unwrap();
        let loaded = sut.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, data);
        assert!(sut.delete_file(file.clone()).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_save_skip_overwrite_fails_for_existing_file() {
        let sut = SUT::new();
        let file = "dummy".to_owned();

        let data = BagOfBytes::sample();
        sut.save_to_file(file.clone(), data.clone(), true)
            .await
            .unwrap();

        let res = sut.save_to_file(file.clone(), data, false).await;
        assert!(matches!(res, Err(CommonError::FileAlreadyExists { .. })));
    }

    #[actix_rt::test]
    async fn test_load_non_existing_is_ok() {
        let sut = SUT::new();
        let res = sut.load_from_file("non-existing".to_owned()).await;
        assert_eq!(res, Ok(None));
    }

    #[actix_rt::test]
    async fn test_delete_non_existing_is_ok() {
        let sut = SUT::new();
        let res = sut.delete_file("does not exist".to_owned()).await;
        assert_eq!(res, Ok(()));
    }
}
