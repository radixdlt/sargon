use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct Drivers {
    pub networking: Arc<dyn NetworkingDriver>,
    pub secure_storage: Arc<dyn SecureStorageDriver>,
    pub entropy_provider: Arc<dyn EntropyProviderDriver>,
    pub host_info: Arc<dyn HostInfoDriver>,
    pub logging_driver: Arc<dyn LoggingDriver>,
}

#[uniffi::export]
impl Drivers {
    #[uniffi::constructor]
    pub fn new(
        networking: Arc<dyn NetworkingDriver>,
        secure_storage: Arc<dyn SecureStorageDriver>,
        entropy_provider: Arc<dyn EntropyProviderDriver>,
        host_info: Arc<dyn HostInfoDriver>,
        logging_driver: Arc<dyn LoggingDriver>,
    ) -> Arc<Self> {
        Arc::new(Self {
            networking,
            secure_storage,
            entropy_provider,
            host_info,
            logging_driver,
        })
    }
}
