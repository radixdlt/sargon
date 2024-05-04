use crate::prelude::*;

#[derive(Debug)]
pub struct Clients {
    pub host: HostInfoClient,
    pub secure_storage: AppSecureStorageClient,
    pub entropy: EntropyClient,
    pub http_client: HttpClient,
}

impl Clients {
    pub fn with_drivers(drivers: Arc<Drivers>) -> Self {
        let host = HostInfoClient::new(drivers.host_info.clone());
        let secure_storage =
            AppSecureStorageClient::new(drivers.secure_storage.clone());
        let entropy = EntropyClient::new(drivers.entropy_provider.clone());
        let http_client = HttpClient::new(drivers.networking.clone());
        Self {
            host,
            secure_storage,
            entropy,
            http_client,
        }
    }

    pub fn new(bios: Arc<Bios>) -> Self {
        Self::with_drivers(bios.drivers.clone())
    }
}
