use crate::prelude::*;

#[derive(Debug)]
pub struct UnsafeStorageClient {
    #[allow(dead_code)]
    driver: Arc<dyn UnsafeStorageDriver>,
}

impl UnsafeStorageClient {
    pub fn new(driver: Arc<dyn UnsafeStorageDriver>) -> Self {
        Self { driver }
    }
}
