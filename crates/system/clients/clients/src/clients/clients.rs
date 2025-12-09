use std::sync::Arc;

use crate::prelude::*;

pub struct Clients {
    pub host: HostInfoClient,
    pub secure_storage: SecureStorageClient,
    pub entropy: EntropyClient,
    pub http_client: HttpClient,
    pub unsafe_storage: UnsafeStorageClient,
    pub file_system: Arc<FileSystemClient>,
    pub event_bus: EventBusClient,
    pub profile_state_change: ProfileStateChangeClient,
    pub factor_instances_cache: FactorInstancesCacheClient,
    pub arculus_wallet_client: ArculusWalletClient,
    pub nft_prices_client: NonFungiblePricesClient,
    pub access_controller_state_repository_client:
        AccessControllerStateRepositoryClient,
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
        let file_system =
            Arc::new(FileSystemClient::new(drivers.file_system.clone()));
        let event_bus = EventBusClient::new(drivers.event_bus.clone());
        let profile_change = ProfileStateChangeClient::new(
            drivers.profile_state_change_driver.clone(),
        );
        let factor_instances_cache =
            FactorInstancesCacheClient::new(file_system.clone());
        let arculus_wallet_client = ArculusWalletClient::new(
            drivers.arculus_csdk_driver.clone(),
            drivers.nfc_tag_driver.clone(),
        );
        let nft_prices_client = NonFungiblePricesClient::new(
            Arc::new(http_client.clone()),
            file_system.clone(),
        );
        let access_controller_cache_client =
            AccessControllerDetailsCacheClient::new(file_system.clone());
        let access_controller_state_repository_client =
            AccessControllerStateRepositoryClient::new(
                http_client.clone(),
                access_controller_cache_client,
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
            factor_instances_cache,
            arculus_wallet_client,
            nft_prices_client,
            access_controller_state_repository_client,
        }
    }
}
