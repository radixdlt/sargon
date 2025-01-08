use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::prelude::*;

#[derive(Debug)]
pub struct RustFileSystemDriver;

impl RustFileSystemDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustFileSystemDriver)
    }
}

#[allow(dead_code)]
pub(crate) fn path_from_str(str: String, require: bool) -> Result<PathBuf> {
    let path = PathBuf::from_str(&str).map_err(|_| CommonError::Unknown)?;
    if require {
        path.try_exists().map_err(|_| CommonError::Unknown)?;
    }
    Ok(path)
}

impl RustFileSystemDriver {
    pub(crate) fn tmp_dir() -> PathBuf {
        fs::canonicalize(Path::new(env!("TMP_DIR"))).unwrap()
    }
}

#[async_trait::async_trait]
impl FileSystemDriver for RustFileSystemDriver {
    async fn writable_app_dir_path(&self) -> Result<String> {
        Ok(Self::tmp_dir().to_string_lossy().to_string())
    }

    async fn load_from_file(&self, path: String) -> Result<Option<BagOfBytes>> {
        let path_buf = path_from_str(path.clone(), true)?;
        match fs::read(path_buf) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                _ => Err(CommonError::FailedToLoadFile { path }),
            },
            Ok(contents) => Ok(Some(BagOfBytes::from(contents))),
        }
    }

    async fn save_to_file(
        &self,
        path: String,
        data: BagOfBytes,
        is_allowed_to_overwrite: bool,
    ) -> Result<()> {
        let path_buf = path_from_str(path.clone(), false)?;
        if let Ok(true) = fs::exists(path_buf.clone())
            && !is_allowed_to_overwrite
        {
            return Err(CommonError::FileAlreadyExists { path });
        }
        fs::write(path_buf, data.as_ref())
            .map_err(|_| CommonError::FailedToSaveFile { path })?;
        Ok(())
    }

    async fn delete_file(&self, path: String) -> Result<()> {
        let path_buf = path_from_str(path.clone(), false)?;
        match fs::remove_file(path_buf) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(CommonError::FailedToDeleteFile { path }),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RustFileSystemDriver;

    fn file_in_dir(dir_path: impl AsRef<Path>) -> String {
        let dir_path = dir_path.as_ref();
        assert!(std::fs::create_dir_all(dir_path).is_ok());

        let file_name =
            format!("delete-me--generated-by-test-{}.txt", Uuid::new_v4());
        let file_path_buf = dir_path.join(file_name);
        let file = String::from(file_path_buf.to_string_lossy());
        file
    }

    fn file_in_tmp() -> String {
        let dir_path = SUT::tmp_dir();
        file_in_dir(dir_path)
    }

    fn contents() -> BagOfBytes {
        BagOfBytes::from("this file is completely safe to delete. it was generated by RustFileSystemDriver test.".as_bytes())
    }

    #[actix_rt::test]
    async fn test_create_load_delete() {
        let sut = SUT::new();
        let file = file_in_tmp();

        let data = contents();
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
        let file = file_in_tmp();

        let data = contents();
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
    async fn test_load_fail() {
        let sut = SUT::new();
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
        let sut = SUT::new();
        let res = sut.delete_file("does not exist".to_owned()).await;
        assert_eq!(res, Ok(()));
    }

    #[actix_rt::test]
    async fn test_save_to_root_is_err() {
        let sut = SUT::new();
        let path = file_in_dir(Path::new("/"));
        let res = sut.save_to_file(path.clone(), contents(), true).await;
        assert_eq!(res, Err(CommonError::FailedToSaveFile { path }));
    }

    #[actix_rt::test]
    async fn test_delete_dir_does_not_work() {
        let sut = SUT::new();
        let path =
            String::from(Path::new(env!("TARGET_DIR")).to_string_lossy());
        let res = sut.delete_file(path.clone()).await;
        assert_eq!(res, Err(CommonError::FailedToDeleteFile { path }));
    }
}
