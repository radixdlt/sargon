use std::path::{Path, PathBuf};

use crate::prelude::*;

#[derive(Debug)]
pub struct RustFileSystemDriver;

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
