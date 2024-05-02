use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "drivers" which the
/// client hosts (iOS/Android wallets) "installs" during app launch, enabling the
/// Sargon "Operating System" to e.g read/write to secure storage and make use
/// of the network connection of the iPhone/Android phone.
#[derive(Debug, uniffi::Object)]
pub struct SargonOS {
    pub(crate) profile_holder: ProfileHolder,
    pub(crate) drivers: Drivers,
}

#[uniffi::export]
impl SargonOS {
    #[uniffi::constructor]
    pub async fn with_drivers(drivers: Arc<Drivers>) -> Arc<Self> {
        // let app_secure_storage = AppSecureStorageClient::new(drivers.secure_storage_driver);
        todo!()
    }
}
