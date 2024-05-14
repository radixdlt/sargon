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
    #[allow(clippy::too_many_arguments)]
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

#[cfg(test)]
impl Drivers {
    pub fn test() -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_networking(networking: Arc<dyn NetworkingDriver>) -> Arc<Self> {
        Drivers::new(
            networking,
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_secure_storage(
        secure_storage: Arc<dyn SecureStorageDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            secure_storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_entropy_provider(
        entropy_provider: Arc<dyn EntropyProviderDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            entropy_provider,
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_host_info(host_info: Arc<dyn HostInfoDriver>) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            host_info,
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_logging(logging: Arc<dyn LoggingDriver>) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            logging,
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_event_bus(event_bus: Arc<dyn EventBusDriver>) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            event_bus,
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_file_system(
        file_system: Arc<dyn FileSystemDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            file_system,
            EphemeralUnsafeStorage::new(),
        )
    }

    pub fn with_unsafe_storage(
        unsafe_storage: Arc<dyn UnsafeStorageDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            unsafe_storage,
        )
    }
}
