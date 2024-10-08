use crate::prelude::*;

#[derive(Debug)]
pub struct Clients {
    pub host: HostInfoClient,
    pub secure_storage: SecureStorageClient,
    pub entropy: EntropyClient,
    pub http_client: HttpClient,
    pub unsafe_storage: UnsafeStorageClient,
    pub file_system: FileSystemClient,
    pub event_bus: EventBusClient,
    pub profile_state_change: ProfileStateChangeClient,
}

impl Clients {
    pub fn with_drivers(drivers: Arc<Drivers>) -> Self {
        let host = HostInfoClient::new(drivers.host_info.clone());
        let secure_storage =
            SecureStorageClient::new(drivers.secure_storage.clone());
        let entropy = EntropyClient::new(drivers.entropy_provider.clone());
        let http_client = HttpClient::new(drivers.networking.clone());
        let unsafe_storage =
            UnsafeStorageClient::new(drivers.unsafe_storage.clone());
        let file_system = FileSystemClient::new(drivers.file_system.clone());
        let event_bus = EventBusClient::new(drivers.event_bus.clone());
        let profile_change = ProfileStateChangeClient::new(
            drivers.profile_state_change_driver.clone(),
        );
        Self {
            host,
            secure_storage,
            entropy,
            http_client,
            unsafe_storage,
            file_system,
            event_bus,
            profile_state_change: profile_change,
        }
    }

    pub fn new(bios: Arc<Bios>) -> Self {
        Self::with_drivers(bios.drivers.clone())
    }
}
