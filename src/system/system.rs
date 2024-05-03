use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "drivers" which the
/// client hosts (iOS/Android wallets) "installs" during app launch, enabling the
/// Sargon "Operating System" to e.g read/write to secure storage and make use
/// of the network connection of the iPhone/Android phone.
#[derive(Debug, uniffi::Object)]
#[allow(dead_code)]
pub struct SargonOS {
    pub(crate) profile_holder: ProfileHolder,
    pub(crate) drivers: Arc<Drivers>,
}

#[uniffi::export]
impl SargonOS {
    #[uniffi::constructor]
    pub async fn with_drivers(drivers: Arc<Drivers>) -> Result<Arc<Self>> {
        let app_secure_storage =
            AppSecureStorageClient::new(drivers.secure_storage.clone());
        if let Some(loaded) = app_secure_storage.load_active_profile().await? {
            Ok(Arc::new(Self {
                drivers,
                profile_holder: ProfileHolder::new(loaded),
            }))
        } else {
            let device_info =
                match app_secure_storage.load_device_info().await? {
                    Some(loaded_device_info) => loaded_device_info,
                    None => {
                        let host_info_client =
                            HostInfoClient::new(drivers.host_info.clone());
                        let new_device_info =
                            host_info_client.create_device_info().await;
                        app_secure_storage
                            .save_device_info(&new_device_info)
                            .await?;
                        new_device_info
                    }
                };
            let entropy_client =
                EntropyClient::new(drivers.entropy_provider.clone());
            let entropy: BIP39Entropy = entropy_client.bip39_entropy();
            let private_bdfs = PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
                true,
                entropy,
                BIP39Passphrase::default(),
                 &device_info
            );

            app_secure_storage
                .save_private_hd_factor_source(&private_bdfs)
                .await?;

            let profile = Profile::new(private_bdfs.factor_source, device_info);

            app_secure_storage
                .save_profile_and_active_profile_id(&profile)
                .await?;

            Ok(Arc::new(Self {
                profile_holder: ProfileHolder::new(profile),
                drivers,
            }))
        }
    }
}
