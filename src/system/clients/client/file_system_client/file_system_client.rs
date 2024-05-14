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
        .ok_or(CommonError::Unknown)
        .map(|s| s.to_owned())
}

#[allow(dead_code)]
impl FileSystemClient {
    async fn load_from_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Option<BagOfBytes>> {
        let path = path_to_string(path.as_ref())?;
        self.driver.load_from_file(path).await
    }

    async fn save_to_file(
        &self,
        path: impl AsRef<Path>,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        let path = path_to_string(path.as_ref())?;
        self.driver
            .save_to_file(path, BagOfBytes::from(data.as_ref()))
            .await
    }

    async fn delete_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_string(path.as_ref())?;
        self.driver.delete_file(path).await
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
        let dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tmp");
        file_in_dir(dir_path)
    }

    fn contents() -> BagOfBytes {
        BagOfBytes::from("this file is completely safe to delete. it was generated by FileSystemClient test.".as_bytes())
    }

    impl FileSystemClient {
        pub(crate) fn test() -> Self {
            Self::new(RustFileSystemDriver::new())
        }
    }

    #[actix_rt::test]
    async fn test_create_load_delete() {
        let sut = SUT::test();
        let file = file_in_tmp();

        let data = contents();
        sut.save_to_file(file.clone(), data.clone()).await.unwrap();
        let loaded = sut.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, data);
        assert!(sut.delete_file(file.clone()).await.is_ok());
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
        assert_eq!(res, Err(CommonError::Unknown));
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
        let file = file_in_dir(Path::new("/"));
        let res = sut.save_to_file(file, contents()).await;
        assert_eq!(res, Err(CommonError::Unknown));
    }

    #[actix_rt::test]
    async fn test_delete_dir_does_not_work() {
        let sut = SUT::test();
        let res = sut
            .delete_file(String::from(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("target/")
                    .to_string_lossy(),
            ))
            .await;
        assert_eq!(res, Err(CommonError::Unknown));
    }
}