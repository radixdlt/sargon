use crate::prelude::*;
use std::sync::RwLock;

#[derive(Debug, uniffi::Object)]
pub struct ProfileHolder {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile: RwLock<Profile>,
}

impl ProfileHolder {
    pub async fn with_driver(
        app_secure_storage_driver: Arc<dyn AppSecureStorageDriver>,
    ) -> Arc<Self> {
        todo!()
    }
}
