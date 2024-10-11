use crate::prelude::*;
use sargon::Drivers as InternalDrivers;

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
    pub profile_state_change_driver: Arc<dyn ProfileStateChangeDriver>,
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

impl Into<InternalDrivers> for Drivers {
    fn into(self) -> InternalDrivers {
        InternalDrivers {
            networking: Arc::new(NetworkingDriverAdapter {
                wrapped: self.networking,
            }),
            secure_storage: Arc::new(SecureStorageDriverAdapter {
                wrapped: self.secure_storage,
            }),
            entropy_provider: Arc::new(EntropyProviderDriverAdapter {
                wrapped: self.entropy_provider,
            }),
            host_info: Arc::new(HostInfoDriverAdapter {
                wrapped: self.host_info,
            }),
            logging: Arc::new(LoggingDriverAdapter {
                wrapped: self.logging,
            }),
            event_bus: Arc::new(EventBusDriverAdapter {
                wrapped: self.event_bus,
            }),
            file_system: Arc::new(FileSystemDriverAdapter {
                wrapped: self.file_system,
            }),
            unsafe_storage: Arc::new(UnsafeStorageDriverAdapter {
                wrapped: self.unsafe_storage,
            }),
            profile_state_change_driver: Arc::new(
                ProfileStateChangeDriverAdapter {
                    wrapped: self.profile_state_change_driver,
                },
            ),
        }
    }
}
