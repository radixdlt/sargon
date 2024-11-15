use crate::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct FileSystemClient {
    #[allow(dead_code)]
    driver: Arc<dyn FileSystemDriver>,
}

impl FileSystemClient {
    pub(crate) fn new(driver: Arc<dyn FileSystemDriver>) -> Self {
        Self { driver }
    }
}

#[allow(dead_code)]
pub(crate) fn path_to_string(path: impl AsRef<Path>) -> Result<String> {
    path.as_ref()
        .to_str()
        .ok_or(CommonError::InvalidPath {
            bad_value: format!("{:?}", path.as_ref()),
        })
        .map(|s| s.to_owned())
}

#[allow(dead_code)]
impl FileSystemClient {
    pub async fn writable_app_dir_path(&self) -> Result<String> {
        self.driver.writable_app_dir_path().await
    }

    pub async fn load_from_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Option<BagOfBytes>> {
        let path = path_to_string(path.as_ref())?;
        self.driver
            .load_from_file(path)
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    pub async fn save_to_file(
        &self,
        path: impl AsRef<Path>,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        let path = path_to_string(path.as_ref())?;
        let data = BagOfBytes::from(data.as_ref());
        self.driver
            .save_to_file(path, data)
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    async fn delete_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_string(path.as_ref())?;
        self.driver
            .delete_file(path)
            // tarpaulin will incorrectly flag next line is missed
            .await
    }
}

#[cfg(test)]
impl FileSystemClient {
    pub fn test() -> Self {
        Self::new(RustFileSystemDriver::new())
    }
    pub fn in_memory() -> Self {
        Self::new(InMemoryFileSystemDriver::new())
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FileSystemClient;

    fn file_in_dir(dir_path: impl AsRef<Path>) -> PathBuf {
        let dir_path = dir_path.as_ref();
        assert!(std::fs::create_dir_all(dir_path).is_ok());

        let file_name = format!("delete-this--generated-by-test-{}.txt", id());
        dir_path.join(file_name)
    }

    fn file_in_tmp() -> PathBuf {
        let dir_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp");
        file_in_dir(dir_path)
    }

    fn make_contents(suffix: impl AsRef<str>) -> BagOfBytes {
        let suffix = suffix.as_ref();
        let str = format!("this file is completely safe to delete. it was generated by FileSystemClient test. Suffix: {}", suffix);
        BagOfBytes::from(str.as_bytes())
    }
    fn contents() -> BagOfBytes {
        make_contents("first")
    }
    fn other_contents() -> BagOfBytes {
        make_contents("second")
    }

    #[actix_rt::test]
    async fn test_create_load_delete() {
        let sut = SUT::test();
        let file = file_in_tmp();

        let data = contents();
        sut.save_to_file(file.clone(), data.clone()).await.unwrap();
        let loaded = sut.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, data);

        // Assert can be updated
        let new_data = other_contents();
        sut.save_to_file(file.clone(), new_data.clone())
            .await
            .unwrap();
        let loaded = sut.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, new_data);

        assert!(sut.delete_file(file.clone()).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_create_load_delete_in_memory_shared() {
        let client1 = Arc::new(SUT::in_memory());
        let file = file_in_tmp();

        let data = contents();
        client1
            .save_to_file(file.clone(), data.clone())
            .await
            .unwrap();
        let loaded =
            client1.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, data);

        // Assert can be updated
        let client2 = client1.clone();
        let new_data = other_contents();
        client2
            .save_to_file(file.clone(), new_data.clone())
            .await
            .unwrap();

        let loaded_from_client2 =
            client2.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded_from_client2, new_data);

        let loaded_from_client1 =
            client1.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded_from_client1, new_data);
    }

    #[actix_rt::test]
    async fn test_load_non_existing_is_ok() {
        let sut = SUT::test();
        let res = sut.load_from_file("non-existing".to_owned()).await;
        assert_eq!(res, Ok(None));
    }

    #[actix_rt::test]
    async fn test_load_fail() {
        let sut = SUT::test();
        let res = sut.load_from_file("/".to_owned()).await;
        assert_eq!(
            res,
            Err(CommonError::FailedToLoadFile {
                path: "/".to_owned()
            })
        );
    }

    #[actix_rt::test]
    async fn test_delete_non_existing_is_ok() {
        let sut = SUT::test();
        let res = sut.delete_file("does not exist".to_owned()).await;
        assert_eq!(res, Ok(()));
    }

    #[actix_rt::test]
    async fn test_save_to_root_is_err() {
        let sut = SUT::test();
        let path = file_in_dir(Path::new("/"));
        let res = sut.save_to_file(path.clone(), contents()).await;
        assert_eq!(
            res,
            Err(CommonError::FailedToSaveFile {
                path: path.to_str().unwrap().to_owned()
            })
        );
    }

    #[actix_rt::test]
    async fn test_delete_dir_does_not_work() {
        let path = String::from(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/")
                .to_string_lossy(),
        );
        let sut = SUT::test();
        let res = sut.delete_file(path.clone()).await;
        assert_eq!(res, Err(CommonError::FailedToDeleteFile { path }));
    }
}
