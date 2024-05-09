use std::sync::Once;

use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "clients" which are
/// created from "drivers" which the hosts (iOS/Android wallets) "installs"
/// during app launch, enabling the  Sargon "Operating System" to e.g read/write
/// to secure storage and make use of the network connection of the iPhone/Android
/// phone.
#[derive(Debug, uniffi::Object)]
#[allow(dead_code)]
pub struct SargonOS {
    pub(crate) profile_holder: ProfileHolder,
    pub(crate) clients: Clients,
}

#[uniffi::export]
impl SargonOS {
    #[uniffi::constructor]
    pub async fn boot(bios: Arc<Bios>) -> Result<Arc<Self>> {
        let clients = Clients::new(bios);

        let sargon_info = SargonBuildInformation::get();
        let version = sargon_info.sargon_version;
        let ret_version = sargon_info.dependencies.radix_engine_toolkit;
        info!("Booting SargonOS {} (RET: {})", &version, &ret_version);
        let host_info = clients.host.summary().await;
        info!("Host: {}", host_info);

        let secure_storage = &clients.secure_storage;

        if let Some(loaded) = secure_storage.load_active_profile().await? {
            info!("Loaded saved profile {}", &loaded.header);
            Ok(Arc::new(Self {
                clients,
                profile_holder: ProfileHolder::new(loaded),
            }))
        } else {
            info!("No saved profile found, creating a new one...");
            let (profile, bdfs) =
                Self::create_new_profile_and_bdfs(&clients).await?;

            secure_storage.save_private_hd_factor_source(&bdfs).await?;

            secure_storage
                .save_profile_and_active_profile_id(&profile)
                .await?;

            info!("Saved new Profile and BDFS, finish booting SargonOS");

            Ok(Arc::new(Self {
                clients,
                profile_holder: ProfileHolder::new(profile),
            }))
        }
    }
}

impl SargonOS {
    pub(crate) async fn new_profile_and_bdfs(
        &self,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        Self::create_new_profile_and_bdfs(&self.clients).await
    }

    async fn create_new_profile_and_bdfs(
        clients: &Clients,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        debug!("Creating new Profile and BDFS");

        let device_info = Self::get_device_info(clients).await?;

        debug!("Generating mnemonic (using Host provided entropy) for a new 'Babylon' `DeviceFactorSource` ('BDFS')");

        let entropy: BIP39Entropy = clients.entropy.bip39_entropy();

        let private_bdfs = PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
            true,
            entropy,
            BIP39Passphrase::default(),
            &device_info
        );
        debug!("Created BDFS (unsaved)");

        debug!("Creating new Profile...");
        let profile =
            Profile::new(private_bdfs.factor_source.clone(), device_info);
        info!("Created new (unsaved) Profile with ID {}", profile.id());
        Ok((profile, private_bdfs))
    }

    pub(crate) async fn device_info(&self) -> Result<DeviceInfo> {
        Self::get_device_info(&self.clients).await
    }

    async fn get_device_info(clients: &Clients) -> Result<DeviceInfo> {
        debug!("Get device info");
        let secure_storage = &clients.secure_storage;

        let device_info = match secure_storage.load_device_info().await? {
            Some(loaded_device_info) => {
                info!("Found saved device info: {}", &loaded_device_info);
                loaded_device_info
            }
            None => {
                info!("Found no saved device info, creating new.");
                let new_device_info = clients.host.create_device_info().await;
                info!("Created new device info: {}", &new_device_info);
                secure_storage.save_device_info(&new_device_info).await?;
                info!("Saved new device info");
                new_device_info
            }
        };

        Ok(device_info)
    }
}
