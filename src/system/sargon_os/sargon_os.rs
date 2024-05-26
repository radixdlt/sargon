use std::sync::Once;

use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "clients" which are
/// created from "drivers" which the hosts (iOS/Android wallets) "installs"
/// during app launch, enabling the  Sargon "Operating System" to e.g read/write
/// to secure storage and make use of the network connection of the iPhone/Android
/// phone.
#[derive(Debug, uniffi::Object)]
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
            let is_owner = Self::check_is_allowed_to_update_provided_profile(
                &clients, &loaded, false,
            )
            .await?;

            if !is_owner {
                warn!("Loaded saved profile was last used on another device, will continue booting OS, but will unable to update Profile.");
            }

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

            let os = Arc::new(Self {
                clients,
                profile_holder: ProfileHolder::new(profile),
            });
            os.event_bus
                .emit(EventNotification::new(Event::Booted))
                .await;
            Ok(os)
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
        let profile = Profile::from_device_factor_source(
            private_bdfs.factor_source.clone(),
            device_info,
        );
        info!("Created new (unsaved) Profile with ID {}", profile.id());
        Ok((profile, private_bdfs))
    }

    pub(crate) async fn device_info(&self) -> Result<DeviceInfo> {
        Self::get_device_info(&self.clients).await
    }

    pub(crate) async fn get_device_info(
        clients: &Clients,
    ) -> Result<DeviceInfo> {
        debug!("Get device info");
        let secure_storage = &clients.secure_storage;

        let device_info = match secure_storage.load_device_info().await? {
            Some(loaded_device_info) => {
                debug!("Found saved device info: {:?}", &loaded_device_info);
                loaded_device_info
            }
            None => {
                debug!("Found no saved device info, creating new.");
                let new_device_info = clients.host.create_device_info().await;
                debug!("Created new device info: {:?}", &new_device_info);
                secure_storage.save_device_info(&new_device_info).await?;
                debug!("Saved new device info");
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

    #[actix_rt::test]
    async fn test_boot_with_existing_profile_is_profile_held() {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::sample();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        // ACT
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        // ASSERT
        let active_profile = os.profile();
        assert_eq!(active_profile.id(), profile.id());
    }

    #[actix_rt::test]
    async fn test_boot_with_existing_unowned_profile_cannot_be_mutated() {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::sample();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        let device_info = os.with_timeout(|x| x.device_info()).await.unwrap();

        // ACT
        let add_res =
            os.with_timeout(|x| x.add_account(Account::sample())).await;

        // ASSERT
        assert_eq!(
            add_res,
            Err(CommonError::ProfileLastUsedOnOtherDevice {
                other_device_id: profile.header.last_used_on_device.id,
                this_device_id: device_info.id
            })
        );
    }

    #[actix_rt::test]
    async fn test_boot_with_existing_unowned_profile_is_not_mutated_if_tried_to(
    ) {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::new(Mnemonic::sample(), DeviceInfo::sample());
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        let new_account = Account::sample_stokenet();
        // ACT
        let _ = os
            .with_timeout(|x| x.add_account(new_account.clone()))
            .await;

        // ASSERT
        assert_eq!(os.profile(), profile.clone()); // not changed in memory

        let loaded_profile = os
            .with_timeout(|x| x.secure_storage.load_active_profile())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(loaded_profile, profile); // not changed in secure storage
    }

    #[actix_rt::test]
    async fn test_boot_with_existing_unowned_profile_when_claimed_can_be_changed(
    ) {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::new(Mnemonic::sample(), DeviceInfo::sample());
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        let new_account = Account::sample_stokenet();
        // ACT
        let claim_was_needed =
            os.with_timeout(|x| x.claim_active_profile()).await.unwrap();
        let _ = os
            .with_timeout(|x| x.add_account(new_account.clone()))
            .await;

        // ASSERT
        assert!(claim_was_needed);
        assert_ne!(os.profile(), profile.clone()); // was changed in memory
        assert_eq!(
            os.profile()
                .networks
                .get_id(NetworkID::Stokenet)
                .unwrap()
                .accounts[0],
            new_account.clone()
        );

        let loaded_profile = os
            .with_timeout(|x| x.secure_storage.load_active_profile())
            .await
            .unwrap()
            .unwrap();
        assert_ne!(loaded_profile.clone(), profile); // was changed in secure storage

        assert_eq!(
            loaded_profile
                .networks
                .get_id(NetworkID::Stokenet)
                .unwrap()
                .accounts[0],
            new_account.clone()
        );
    }

    #[actix_rt::test]
    async fn test_boot_not_owned_emits_event() {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let event_bus_driver = RustEventBusDriver::new();
        let profile = Profile::sample();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::new(
            RustNetworkingDriver::new(),
            secure_storage_driver.clone(),
            RustEntropyDriver::new(),
            RustHostInfoDriver::new(),
            RustLoggingDriver::new(),
            event_bus_driver.clone(),
            RustFileSystemDriver::new(),
            EphemeralUnsafeStorage::new(),
        );
        let bios = Bios::new(drivers);

        // ACT
        let _ = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        // ASSERT
        assert!(
            event_bus_driver
                .recorded()
                .iter()
                .any(|e| e.event.kind()
                    == EventKind::ProfileLastUsedOnOtherDevice)
        );
    }

    #[actix_rt::test]
    async fn test_boot_with_existing_profile_active_profile_id() {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::sample();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        secure_storage_client
            .save_active_profile_id(profile.id())
            .await
            .unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        // ACT
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

        // ASSERT
        let active_profile_id = os
            .with_timeout(|x| x.secure_storage.load_active_profile_id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(active_profile_id, profile.id());
    }

    #[actix_rt::test]
    async fn test_change_log_level() {
        // ARRANGE (and ACT)
        let _ = SUT::fast_boot().await;

        rust_logger_get_all_filters().into_iter().for_each(|l| {
            rust_logger_set_level(l);
            assert_eq!(rust_logger_get_level(), l);
            println!(
                "Testing logging at every level with log level set to: {:?}",
                l
            );
            rust_logger_log_at_every_level()
        });
    }
}
