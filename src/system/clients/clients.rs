use crate::prelude::*;

#[derive(Debug)]
pub struct Clients {
    pub host: HostInfoClient,
    pub secure_storage: SecureStorageClient,
    pub entropy: EntropyClient,
    pub http_client: HttpClient,
    pub gateway_client: GatewayClient,
    pub unsafe_storage: UnsafeStorageClient,
    pub file_system: FileSystemClient,
    pub event_bus: EventBusClient,
    pub signing: SigningClient,
}

impl Clients {
    pub fn with_drivers(drivers: Arc<Drivers>) -> Self {
        let host = HostInfoClient::new(drivers.host_info.clone());
        let secure_storage =
            SecureStorageClient::new(drivers.secure_storage.clone());
        let entropy = EntropyClient::new(drivers.entropy_provider.clone());
        let http_client = HttpClient::new(drivers.networking.clone());
        let gateway_client = GatewayClient {
            http_client: http_client.clone(),
            gateway: Gateway::mainnet(),
        };
        let unsafe_storage =
            UnsafeStorageClient::new(drivers.unsafe_storage.clone());
        let file_system = FileSystemClient::new(drivers.file_system.clone());
        let event_bus = EventBusClient::new(drivers.event_bus.clone());
        let signing = SigningClient::new(
            drivers.use_device_factor_source_driver.clone(),
            drivers.use_security_questions_factor_source_driver.clone(),
            drivers.use_arculus_factor_source_driver.clone(),
            drivers.use_off_device_mnemonic_factor_source_driver.clone(),
            drivers
                .use_ledger_hardware_wallet_factor_source_driver
                .clone(),
        );
        Self {
            host,
            secure_storage,
            entropy,
            http_client,
            gateway_client,
            unsafe_storage,
            file_system,
            event_bus,
            signing,
        }
    }

    pub fn new(bios: Arc<Bios>) -> Self {
        Self::with_drivers(bios.drivers.clone())
    }
}
