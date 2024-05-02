use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct Drivers {
    pub networking_driver: Arc<dyn NetworkingDriver>,
    pub secure_storage_driver: Arc<dyn SecureStorageDriver>,
}
