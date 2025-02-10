use crate::prelude::*;

pub struct Drivers {
    pub networking: Arc<dyn NetworkingDriver>,
    pub secure_storage: Arc<dyn SecureStorageDriver>,
    pub entropy_provider: Arc<dyn EntropyProviderDriver>,
    pub host_info: Arc<dyn HostInfoDriver>,
    pub logging: Arc<dyn LoggingDriver>,
    pub event_bus: Arc<dyn EventBusDriver>,
    pub file_system: Arc<dyn FileSystemDriver>,
    pub unsafe_storage: Arc<dyn UnsafeStorageDriver>,
    pub profile_state_change_driver: Arc<dyn ProfileStateChangeDriver>,
}

impl Drivers {
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
        profile_state_change_driver: Arc<dyn ProfileStateChangeDriver>,
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
            profile_state_change_driver,
        })
    }
}

impl Drivers {
    fn file_system() -> Arc<dyn FileSystemDriver> {
        #[cfg(test)]
        return InMemoryFileSystemDriver::new();

        #[cfg(not(test))]
        return RustFileSystemDriver::new();
    }

    pub fn test() -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
        )
    }

    pub fn with_storages(
        secure_storage: Arc<dyn SecureStorageDriver>,
        unsafe_storage: Arc<dyn UnsafeStorageDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            secure_storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            Self::file_system(),
            unsafe_storage,
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            RustProfileStateChangeDriver::new(),
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
            RustProfileStateChangeDriver::new(),
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
            Self::file_system(),
            unsafe_storage,
            RustProfileStateChangeDriver::new(),
        )
    }

    pub fn with_profile_state_change(
        profile_state_change: Arc<dyn ProfileStateChangeDriver>,
    ) -> Arc<Self> {
        Drivers::new(
            RustNetworkingDriver::new(),
            EphemeralSecureStorage::new(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            Self::file_system(),
            EphemeralUnsafeStorage::new(),
            profile_state_change,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Drivers;

    #[test]
    fn get_networking() {
        let d = RustNetworkingDriver::new();
        let sut = SUT::with_networking(d.clone());
        assert_eq!(Arc::as_ptr(&sut.networking), Arc::as_ptr(&d));
    }

    #[test]
    fn get_secure_storage() {
        let d = EphemeralSecureStorage::new();
        let sut = SUT::with_secure_storage(d.clone());
        assert_eq!(Arc::as_ptr(&sut.secure_storage), Arc::as_ptr(&d));
    }

    #[test]
    fn get_entropy_provider() {
        let d = RustEntropyDriver::new();
        let sut = SUT::with_entropy_provider(d.clone());
        assert_eq!(Arc::as_ptr(&sut.entropy_provider), Arc::as_ptr(&d));
    }

    #[test]
    fn get_host_info() {
        let d = RustHostInfoDriver::new();
        let sut = SUT::with_host_info(d.clone());
        assert_eq!(Arc::as_ptr(&sut.host_info), Arc::as_ptr(&d));
    }

    #[test]
    fn get_logging() {
        let d = RustLoggingDriver::new();
        let sut = SUT::with_logging(d.clone());
        assert_eq!(Arc::as_ptr(&sut.logging), Arc::as_ptr(&d));
    }

    #[test]
    fn get_event_bus() {
        let d = RustEventBusDriver::new();
        let sut = SUT::with_event_bus(d.clone());
        assert_eq!(Arc::as_ptr(&sut.event_bus), Arc::as_ptr(&d));
    }

    #[test]
    fn get_file_system() {
        let d = RustFileSystemDriver::new();
        let sut = SUT::with_file_system(d.clone());
        assert_eq!(Arc::as_ptr(&sut.file_system), Arc::as_ptr(&d));
    }

    #[test]
    fn get_unsafe_storage() {
        let d = EphemeralUnsafeStorage::new();
        let sut = SUT::with_unsafe_storage(d.clone());
        assert_eq!(Arc::as_ptr(&sut.unsafe_storage), Arc::as_ptr(&d));
    }
}
