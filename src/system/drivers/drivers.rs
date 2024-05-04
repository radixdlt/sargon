use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct Drivers {
    pub networking: Arc<dyn NetworkingDriver>,
    pub secure_storage: Arc<dyn SecureStorageDriver>,
    pub entropy_provider: Arc<dyn EntropyProviderDriver>,
    pub host_info: Arc<dyn HostInfoDriver>,
    pub logging: Arc<dyn LoggingDriver>,
    pub event_bus: Arc<dyn EventBusDriver>,
    pub file_system: Arc<dyn FileSystemDriver>,
    pub unsafe_storage: Arc<dyn UnsafeStorageDriver>,
}

#[uniffi::export]
impl Drivers {
    #[uniffi::constructor]
    pub fn new(
        networking: Arc<dyn NetworkingDriver>,
        secure_storage: Arc<dyn SecureStorageDriver>,
        entropy_provider: Arc<dyn EntropyProviderDriver>,
        host_info: Arc<dyn HostInfoDriver>,
        logging: Arc<dyn LoggingDriver>,
        event_bus: Arc<dyn EventBusDriver>,
        file_system: Arc<dyn FileSystemDriver>,
        unsafe_storage: Arc<dyn UnsafeStorageDriver>,
    ) -> Arc<Self> {
        Arc::new(Self {
            networking,
            secure_storage,
            entropy_provider,
            host_info,
            logging,
            event_bus,
            file_system,
            unsafe_storage,
        })
    }
}
