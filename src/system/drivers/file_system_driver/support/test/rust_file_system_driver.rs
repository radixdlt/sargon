use std::path::{Path, PathBuf};

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
        Ok(path)
    } else {
        Ok(path)
    }
}

#[async_trait::async_trait]
impl FileSystemDriver for RustFileSystemDriver {
    async fn load_from_file(&self, path: String) -> Result<Option<BagOfBytes>> {
        let path = path_from_str(path, true)?;
        match fs::read(path) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                _ => Err(CommonError::Unknown),
            },
            Ok(contents) => Ok(Some(BagOfBytes::from(contents))),
        }
    }

    async fn save_to_file(&self, path: String, data: BagOfBytes) -> Result<()> {
        let path = path_from_str(path, false)?;
        fs::write(path, data.as_ref()).map_err(|_| CommonError::Unknown)?;
        Ok(())
    }

    async fn delete_file(&self, path: String) -> Result<()> {
        let path = path_from_str(path, false)?;
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(CommonError::Unknown),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RustFileSystemDriver;

    #[actix_rt::test]
    async fn test_create_load_delete() {
        let sut = SUT::new();

        let dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tmp");
        assert!(std::fs::create_dir_all(dir_path.clone()).is_ok());

        let file_name = format!("delete-me--generated-by-test-{}.txt", id());
        let file_path_buf = dir_path.join(file_name);
        let file = String::from(file_path_buf.to_string_lossy());

        let data = BagOfBytes::from("this file is completely safe to delete. it was generated by RustFileSystemDriver test.".as_bytes());
        sut.save_to_file(file.clone(), data.clone()).await.unwrap();
        let loaded = sut.load_from_file(file.clone()).await.unwrap().unwrap();
        assert_eq!(loaded, data);
        assert!(sut.delete_file(file.clone()).await.is_ok());
    }
}
