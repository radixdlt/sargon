use crate::prelude::*;
use sargon::Drivers as InternalDrivers;

#[derive(Debug, Clone, uniffi::Object)]
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
    pub arculus_csdk_driver: Arc<dyn ArculusCSDKDriver>,
    pub nfc_tag_driver: Arc<dyn NFCTagDriver>,
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
        arculus_csdk_driver: Arc<dyn ArculusCSDKDriver>,
        nfc_tag_driver: Arc<dyn NFCTagDriver>,
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
            arculus_csdk_driver,
            nfc_tag_driver,
        })
    }
}

impl From<Drivers> for InternalDrivers {
    fn from(val: Drivers) -> Self {
        InternalDrivers {
            networking: Arc::new(NetworkingDriverAdapter {
                wrapped: val.networking,
            }),
            secure_storage: Arc::new(SecureStorageDriverAdapter {
                wrapped: val.secure_storage,
            }),
            entropy_provider: Arc::new(EntropyProviderDriverAdapter {
                wrapped: val.entropy_provider,
            }),
            host_info: Arc::new(HostInfoDriverAdapter {
                wrapped: val.host_info,
            }),
            logging: Arc::new(LoggingDriverAdapter {
                wrapped: val.logging,
            }),
            event_bus: Arc::new(EventBusDriverAdapter {
                wrapped: val.event_bus,
            }),
            file_system: Arc::new(FileSystemDriverAdapter {
                wrapped: val.file_system,
            }),
            unsafe_storage: Arc::new(UnsafeStorageDriverAdapter {
                wrapped: val.unsafe_storage,
            }),
            profile_state_change_driver: Arc::new(
                ProfileStateChangeDriverAdapter {
                    wrapped: val.profile_state_change_driver,
                },
            ),
            arculus_csdk_driver: Arc::new(ArculusCSDKDriverAdapter {
                wrapped: val.arculus_csdk_driver,
            }),
            nfc_tag_driver: Arc::new(NFCTagDriverAdapter {
                wrapped: val.nfc_tag_driver,
            }),
        }
    }
}
