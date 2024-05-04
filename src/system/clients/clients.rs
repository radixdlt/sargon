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
        // let host = HostInfoClient::new(drivers.host_info);
        // let secure_storage =
        //     AppSecureStorageClient::new(drivers.secure_storage);
        todo!();
    }

    pub fn new(bios: Arc<Bios>) -> Self {
        // let bios_ptr = unsafe { Arc::into_raw(bios) };
        // let bios = unsafe { *bios_ptr };
        // Self::with_drivers(bios.drivers)
        todo!()
    }
}
