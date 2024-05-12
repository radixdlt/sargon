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

/// So that we do not have to go through `self.clients`,
/// but can use e.g. `self.secure_storage` directly.
impl Deref for SargonOS {
    type Target = Clients;

    fn deref(&self) -> &Self::Target {
        &self.clients
    }
}

#[uniffi::export]
impl SargonOS {
    #[uniffi::constructor]
    pub async fn boot(bios: Arc<Bios>) -> Result<Arc<Self>> {
        Self::boot_with_bdfs(bios, None).await
    }
}

impl SargonOS {
    pub async fn boot_with_bdfs(
        bios: Arc<Bios>,
        bdfs_mnemonic: Option<MnemonicWithPassphrase>,
    ) -> Result<Arc<Self>> {
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
                Self::create_new_profile_with_bdfs(&clients, bdfs_mnemonic)
                    .await?;

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
        Self::create_new_profile_with_bdfs(clients, None).await
    }

    async fn create_new_profile_with_bdfs(
        clients: &Clients,
        mnemonic_with_passphrase: Option<MnemonicWithPassphrase>,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        debug!("Creating new Profile and BDFS");

        let device_info = Self::get_device_info(clients).await?;

        let is_main = true;
        let private_bdfs = match mnemonic_with_passphrase {
            Some(mwp) => {
                debug!("Using specified MnemonicWithPassphrase, perhaps we are running in at test...");

                PrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(is_main, mwp, &device_info)
            }
            None => {
                debug!("Generating mnemonic (using Host provided entropy) for a new 'Babylon' `DeviceFactorSource` ('BDFS')");

                let entropy: BIP39Entropy = clients.entropy.bip39_entropy();

                PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
                    is_main,
                    entropy,
                    BIP39Passphrase::default(),
                    &device_info
                )
            }
        };
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
                info!("Found saved device info: {:?}", &loaded_device_info);
                loaded_device_info
            }
            None => {
                info!("Found no saved device info, creating new.");
                let new_device_info = clients.host.create_device_info().await;
                info!("Created new device info: {:?}", &new_device_info);
                secure_storage.save_device_info(&new_device_info).await?;
                info!("Saved new device info");
                new_device_info
            }
        };

        Ok(device_info)
    }
}

#[cfg(test)]
pub(crate) const SARGON_OS_TEST_MAX_ASYNC_DURATION: std::time::Duration =
    std::time::Duration::from_millis(50);

#[cfg(test)]
impl SargonOS {
    pub async fn with_timeout<'a, F, Fut, T>(&'a self, func: F) -> T
    where
        F: Fn(&'a SargonOS) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let sut = func(self);
        actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, sut)
            .await
            .unwrap()
    }

    pub async fn boot_test() -> Result<Arc<Self>> {
        Self::boot_test_with_bdfs_mnemonic(None).await
    }

    pub async fn boot_test_with_bdfs_mnemonic(
        bdfs_mnemonic: impl Into<Option<MnemonicWithPassphrase>>,
    ) -> Result<Arc<Self>> {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        Self::boot_with_bdfs(bios, bdfs_mnemonic.into()).await
    }

    pub async fn fast_boot() -> Arc<Self> {
        Self::fast_boot_bdfs(None).await
    }

    pub async fn fast_boot_bdfs(
        bdfs_mnemonic: impl Into<Option<MnemonicWithPassphrase>>,
    ) -> Arc<Self> {
        let req = Self::boot_test_with_bdfs_mnemonic(bdfs_mnemonic);

        actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::time::Duration;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_new_profile_is_active_profile() {
        // ARRANGE (and ACT)
        let os = SUT::fast_boot().await;

        // ASSERT
        let active_profile_id = os
            .with_timeout(|x| x.secure_storage.load_active_profile_id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(active_profile_id, os.profile().id());
    }
}
