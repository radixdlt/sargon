use crate::prelude::*;

#[derive(Debug)]
pub struct UnsafeStorageClient {
    driver: Arc<dyn UnsafeStorageDriver>,
}

impl UnsafeStorageClient {
    pub(crate) fn new(driver: Arc<dyn UnsafeStorageDriver>) -> Self {
        Self { driver }
    }
}
