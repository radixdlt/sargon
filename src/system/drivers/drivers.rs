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
    pub use_device_factor_source_driver: Arc<dyn UseDeviceFactorSourceDriver>,
    pub use_security_questions_factor_source_driver:
        Arc<dyn UseSecurityQuestionsFactorSourceDriver>,
    pub use_arculus_factor_source_driver:
        Arc<dyn GenericMnemonicFactorSourceDriver>,
    pub use_off_device_mnemonic_factor_source_driver:
        Arc<dyn GenericMnemonicFactorSourceDriver>,
    pub use_ledger_hardware_wallet_factor_source_driver:
        Arc<dyn GenericMnemonicFactorSourceDriver>,
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
        use_device_factor_source_driver: Arc<dyn UseDeviceFactorSourceDriver>,
        use_security_questions_factor_source_driver: Arc<
            dyn UseSecurityQuestionsFactorSourceDriver,
        >,
        use_arculus_factor_source_driver: Arc<
            dyn GenericMnemonicFactorSourceDriver,
        >,
        use_off_device_mnemonic_factor_source_driver: Arc<
            dyn GenericMnemonicFactorSourceDriver,
        >,
        use_ledger_hardware_wallet_factor_source_driver: Arc<
            dyn GenericMnemonicFactorSourceDriver,
        >,
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
            use_device_factor_source_driver,
            use_arculus_factor_source_driver,
            use_off_device_mnemonic_factor_source_driver,
            use_ledger_hardware_wallet_factor_source_driver,
            use_security_questions_factor_source_driver,
        })
    }
}

#[cfg(test)]
impl Drivers {
    pub fn test() -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage.clone(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_networking(networking: Arc<dyn NetworkingDriver>) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            networking,
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_secure_storage(
        storage: Arc<dyn SecureStorageDriver>,
    ) -> Arc<Self> {
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_entropy_provider(
        entropy_provider: Arc<dyn EntropyProviderDriver>,
    ) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            entropy_provider,
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_host_info(host_info: Arc<dyn HostInfoDriver>) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            host_info,
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_logging(logging: Arc<dyn LoggingDriver>) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            logging,
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_event_bus(event_bus: Arc<dyn EventBusDriver>) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            event_bus,
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_file_system(
        file_system: Arc<dyn FileSystemDriver>,
    ) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            file_system,
            EphemeralUnsafeStorage::new(),
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
        )
    }

    pub fn with_unsafe_storage(
        unsafe_storage: Arc<dyn UnsafeStorageDriver>,
    ) -> Arc<Self> {
        let storage = EphemeralSecureStorage::new();
        let generic_factor_source_driver =
            RustUseMnemonicBasedFactorSourceDriver::new(storage.clone());
        Drivers::new(
            RustNetworkingDriver::new(),
            storage,
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            RustEventBusDriver::new(),
            RustFileSystemDriver::new(),
            unsafe_storage,
            generic_factor_source_driver.clone(),
            RustAnswerSecurityQuestionsDriver::new(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            ),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
            generic_factor_source_driver.clone(),
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
