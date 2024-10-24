use std::sync::Once;

use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "clients" which are
/// created from "drivers" which the hosts (iOS/Android wallets) "installs"
/// during app launch, enabling the  Sargon "Operating System" to e.g read/write
/// to secure storage and make use of the network connection of the iPhone/Android
/// phone.
#[derive(Debug)]
pub struct SargonOS {
    pub(crate) profile_state_holder: ProfileStateHolder,
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

impl SargonOS {
    pub async fn boot(bios: Arc<Bios>) -> Arc<Self> {
        let clients = Clients::new(bios);

        let sargon_info = SargonBuildInformation::get();
        let version = sargon_info.sargon_version;
        let ret_version = sargon_info.dependencies.radix_engine_toolkit;
        info!("Booting SargonOS {} (RET: {})", &version, &ret_version);
        let host_info = clients.host.summary().await;
        info!("Host: {}", host_info);

        let secure_storage = &clients.secure_storage;
        let mut profile_state = secure_storage
            .load_profile()
            .await
            .map_or_else(ProfileState::Incompatible, |some_profile| {
                some_profile
                    .map(ProfileState::Loaded)
                    .unwrap_or(ProfileState::None)
            });

        // If an ephemeral profile was created (a profile with no networks) then it is not
        // considered as a Loaded profile.
        if let Some(profile) = profile_state.as_loaded()
            && profile.networks.is_empty()
        {
            // Delete profile and its associated mnemonics
            let device_factor_sources = profile.device_factor_sources();
            for dfs in device_factor_sources.iter() {
                let _ = secure_storage.delete_mnemonic(&dfs.id).await;
            }
            let _ = secure_storage.delete_profile(profile.id()).await;

            profile_state = ProfileState::None;
        }

        let os = Arc::new(Self {
            clients,
            profile_state_holder: ProfileStateHolder::new(
                profile_state.clone(),
            ),
        });
        os.clients
            .profile_state_change
            .emit(profile_state.clone())
            .await;

        os.event_bus
            .emit(EventNotification::new(Event::Booted))
            .await;

        info!("Sargon os Booted with profile state: {}", profile_state);
        os
    }

    pub async fn new_wallet(&self) -> Result<()> {
        let (profile, bdfs) = self.create_new_profile_with_bdfs(None).await?;

        self.secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;

        let set_profile_result = self.set_profile(profile).await;
        if let Some(error) = set_profile_result.err() {
            self.secure_storage
                .delete_mnemonic(&bdfs.factor_source.id)
                .await?;
            return Err(error);
        }

        info!("Saved new Profile and BDFS, finish creating wallet");

        Ok(())
    }

    pub async fn import_wallet(
        &self,
        profile: &Profile,
        bdfs_skipped: bool,
    ) -> Result<()> {
        let imported_id = profile.id();
        debug!("Importing profile, id: {}", imported_id);
        let mut profile = profile.clone();
        self.claim_profile(&mut profile).await?;
        self.secure_storage.save_profile(&profile).await?;
        self.profile_state_holder
            .replace_profile_state_with(ProfileState::Loaded(profile))?;
        debug!(
            "Saved imported profile into secure storage, id: {}",
            imported_id
        );

        if bdfs_skipped {
            let entropy: BIP39Entropy = self.clients.entropy.bip39_entropy();

            let host_info = self.host_info().await;
            let bdfs = PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
                true,
                entropy,
                BIP39Passphrase::default(),
                &host_info,
            );

            let bdfs_result = self
                .add_factor_source(FactorSource::from(
                    bdfs.clone().factor_source,
                ))
                .await;
            if let Some(error) = bdfs_result.err() {
                self.secure_storage.delete_profile(imported_id).await?;
                return Err(error);
            }
            self.secure_storage
                .save_private_hd_factor_source(&bdfs)
                .await?;
        }

        let profile_to_report = self.profile_state_holder.profile()?;
        self.clients
            .profile_state_change
            .emit(ProfileState::Loaded(profile_to_report))
            .await;
        self.event_bus
            .emit(EventNotification::new(Event::ProfileImported {
                id: imported_id,
            }))
            .await;

        info!("Successfully imported profile, id: {}", imported_id);

        Ok(())
    }

    pub async fn new_wallet_with_derived_bdfs(
        &self,
        hd_factor_source: PrivateHierarchicalDeterministicFactorSource,
        accounts: Accounts,
    ) -> Result<()> {
        debug!("Deriving Profile from BDFS");

        let hd_keys: Vec<HierarchicalDeterministicPublicKey> = accounts
            .iter()
            .map(|account| {
                account
                    .security_state
                    .into_unsecured()
                    .map(|c| c.transaction_signing.public_key)
                    .map_err(|_| CommonError::EntitiesNotDerivedByFactorSource)
            })
            .try_collect()?;

        if !hd_factor_source
            .mnemonic_with_passphrase
            .validate_public_keys(hd_keys)
        {
            return Err(CommonError::EntitiesNotDerivedByFactorSource);
        }

        self.secure_storage
            .save_private_hd_factor_source(&hd_factor_source)
            .await?;

        let host_id = self.host_id().await?;
        let host_info = self.host_info().await;

        let profile = Profile::from_device_factor_source(
            hd_factor_source.factor_source,
            host_id,
            host_info,
            Some(accounts),
        );

        self.set_profile(profile).await?;

        info!("Successfully derived Profile");
        Ok(())
    }

    pub async fn delete_wallet(&self) -> Result<()> {
        self.delete_profile_and_mnemonics_replace_in_memory_with_none()
            .await?;
        self.clients
            .profile_state_change
            .emit(ProfileState::None)
            .await;
        Ok(())
    }

    pub async fn resolve_host_id(&self) -> Result<HostId> {
        self.host_id().await
    }

    pub async fn resolve_host_info(&self) -> HostInfo {
        self.host_info().await
    }
}

impl SargonOS {
    pub(crate) async fn create_new_profile_with_bdfs(
        &self,
        mnemonic_with_passphrase: Option<MnemonicWithPassphrase>,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        debug!("Creating new Profile and BDFS");

        let host_id = self.host_id().await?;
        let host_info = self.host_info().await;

        let is_main = true;
        let private_bdfs = match mnemonic_with_passphrase {
            Some(mwp) => {
                debug!("Using specified MnemonicWithPassphrase, perhaps we are running in at test...");

                PrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(is_main, mwp, &host_info)
            }
            None => {
                debug!("Generating mnemonic (using Host provided entropy) for a new 'Babylon' `DeviceFactorSource` ('BDFS')");

                let entropy: BIP39Entropy =
                    self.clients.entropy.bip39_entropy();

                PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
                    is_main,
                    entropy,
                    BIP39Passphrase::default(),
                    &host_info,
                )
            }
        };
        debug!("Created BDFS (unsaved)");

        debug!("Creating new Profile...");
        let profile = Profile::with(
            Header::new(DeviceInfo::new_from_info(&host_id, &host_info)),
            FactorSources::with_bdfs(private_bdfs.factor_source.clone()),
            AppPreferences::default(),
            ProfileNetworks::default(),
        );
        info!("Created new (unsaved) Profile with ID {}", profile.id());
        Ok((profile, private_bdfs))
    }

    pub(crate) async fn host_id(&self) -> Result<HostId> {
        Self::get_host_id(&self.clients).await
    }

    pub(crate) async fn get_host_id(clients: &Clients) -> Result<HostId> {
        debug!("Get Host ID");
        let secure_storage = &clients.secure_storage;

        match secure_storage.load_host_id().await? {
            Some(loaded_host_id) => {
                debug!("Found saved host id: {:?}", &loaded_host_id);
                Ok(loaded_host_id)
            }
            None => {
                debug!("Found no saved host id, creating new.");
                let new_host_id = HostId::generate_new();
                debug!("Created new host id: {:?}", &new_host_id);
                secure_storage.save_host_id(&new_host_id).await?;
                debug!("Saved new host id");
                Ok(new_host_id)
            }
        }
    }

    pub(crate) async fn host_info(&self) -> HostInfo {
        Self::get_host_info(&self.clients).await
    }

    pub(crate) async fn get_host_info(clients: &Clients) -> HostInfo {
        debug!("Get Host info");
        clients.host.resolve_host_info().await
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
        let os = Self::boot(bios).await;
        let (mut profile, bdfs) = os
            .create_new_profile_with_bdfs(bdfs_mnemonic.into())
            .await?;

        // Append Mainnet network since initial profile has no network
        profile
            .networks
            .append(ProfileNetwork::new_empty_on(NetworkID::Mainnet));

        os.secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;
        os.secure_storage.save_profile(&profile).await?;
        os.profile_state_holder.replace_profile_state_with(
            ProfileState::Loaded(profile.clone()),
        )?;

        Ok(os)
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

    /// Boot the SargonOS with a mocked networking driver.
    /// This is useful for testing the SargonOS without needing to connect to the internet.
    pub async fn boot_test_with_networking_driver(
        networking: Arc<dyn NetworkingDriver>,
    ) -> Result<Arc<Self>> {
        let drivers = Drivers::with_networking(networking);
        let bios = Bios::new(drivers);
        let os = Self::boot(bios).await;

        let (mut profile, bdfs) = os.create_new_profile_with_bdfs(None).await?;

        // Append Stokenet network since initial profile has no network
        profile
            .networks
            .append(ProfileNetwork::new_empty_on(NetworkID::Stokenet));

        os.secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;
        os.secure_storage.save_profile(&profile).await?;
        os.profile_state_holder.replace_profile_state_with(
            ProfileState::Loaded(profile.clone()),
        )?;

        Ok(os)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use actix_rt::time::timeout;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_new_profile_is_active_profile() {
        // ARRANGE (and ACT)
        let os = SUT::fast_boot().await;

        // ASSERT
        let active_profile = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(active_profile.id(), os.profile().unwrap().id());
    }

    #[actix_rt::test]
    async fn test_boot_with_existing_profile_is_profile_held() {
        // ARRANGE (and ACT)
        let secure_storage_driver = EphemeralSecureStorage::new();
        let profile = Profile::sample();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(&profile).await.unwrap();
        let drivers = Drivers::with_secure_storage(secure_storage_driver);
        let bios = Bios::new(drivers);

        // ACT
        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap();

        // ASSERT
        let active_profile = os.profile();
        assert_eq!(active_profile.unwrap().id(), profile.id());
    }

    #[actix_rt::test]
    async fn test_boot_when_existing_profile_with_no_networks_profile_state_considered_none(
    ) {
        // ARRANGE
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios.clone()).await;
        let (first_profile, first_bdfs) =
            os.create_new_profile_with_bdfs(None).await.unwrap();

        os.secure_storage
            .save_private_hd_factor_source(&first_bdfs)
            .await
            .unwrap();
        os.secure_storage
            .save_profile(&first_profile)
            .await
            .unwrap();
        os.profile_state_holder
            .replace_profile_state_with(ProfileState::Loaded(
                first_profile.clone(),
            ))
            .unwrap();

        // ACT
        let new_os = SUT::boot(bios.clone()).await;

        // ASSERT
        assert!(new_os.profile().is_err());
        assert!(new_os
            .secure_storage
            .load_profile()
            .await
            .unwrap()
            .is_none());
        assert!(new_os
            .secure_storage
            .load_mnemonic_with_passphrase(&first_bdfs.factor_source.id)
            .await
            .is_err())
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

    #[actix_rt::test]
    async fn test_new_wallet() {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios).await;

        os.new_wallet().await.unwrap();

        let profile = os.profile().unwrap();
        let bdfs = profile.bdfs();

        assert!(os
            .clients
            .secure_storage
            .load_mnemonic_with_passphrase(&bdfs.id)
            .await
            .is_ok());

        assert!(profile.networks.is_empty());
    }

    #[actix_rt::test]
    async fn test_wallet_import_without_bdfs_skip() {
        let os = SUT::fast_boot().await;
        let profile_to_import = Profile::sample();

        os.import_wallet(&profile_to_import, false).await.unwrap();

        assert_eq!(os.profile().unwrap().bdfs(), profile_to_import.bdfs());
    }

    #[actix_rt::test]
    async fn test_wallet_import_with_bdfs_skip() {
        let os = SUT::fast_boot().await;
        let profile_to_import = Profile::sample();

        os.import_wallet(&profile_to_import, true).await.unwrap();

        assert_ne!(os.profile().unwrap().bdfs(), profile_to_import.bdfs());
    }

    #[actix_rt::test]
    async fn test_new_wallet_through_derived_bdfs() {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios).await;

        os.new_wallet_with_derived_bdfs(
            PrivateHierarchicalDeterministicFactorSource::sample(),
            Accounts::sample_mainnet(),
        )
        .await
        .unwrap();

        let profile = os.profile().unwrap();

        assert!(profile.has_any_account_on_any_network());
    }

    #[actix_rt::test]
    async fn test_new_wallet_through_derived_bdfs_with_empty_accounts() {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios).await;

        os.new_wallet_with_derived_bdfs(
            PrivateHierarchicalDeterministicFactorSource::sample(),
            Accounts::new(),
        )
        .await
        .unwrap();

        let profile = os.profile().unwrap();

        assert!(!profile.networks.is_empty());
    }

    #[actix_rt::test]
    async fn test_new_wallet_through_derived_bdfs_with_accounts_derived_from_other_hd_factor_source(
    ) {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios).await;

        let other_hd =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let invalid_account = Account::new(
            other_hd
                .derive_entity_creation_factor_instance(NetworkID::Mainnet, 0),
            DisplayName::new("Invalid Account").unwrap(),
            AppearanceID::sample(),
        );

        let result = os
            .new_wallet_with_derived_bdfs(
                PrivateHierarchicalDeterministicFactorSource::sample(),
                Accounts::just(invalid_account),
            )
            .await;

        assert_eq!(
            result.unwrap_err(),
            CommonError::EntitiesNotDerivedByFactorSource
        )
    }

    #[actix_rt::test]
    async fn test_delete_wallet() {
        let test_drivers = Drivers::test();
        let bios = Bios::new(test_drivers);
        let os = SUT::boot(bios).await;
        os.new_wallet().await.unwrap();
        let profile = os.profile().unwrap();
        let bdfs = profile.bdfs();

        os.delete_wallet().await.unwrap();

        // Assert in memory profile is None
        assert!(os.profile().is_err());
        // Assert in profile is deleted from storage
        assert_eq!(
            os.clients.secure_storage.load_profile().await.unwrap(),
            None
        );
        // Assert mnemonic is deleted from storage
        assert!(os
            .clients
            .secure_storage
            .load_mnemonic_with_passphrase(&bdfs.id)
            .await
            .is_err());
    }

    #[actix_rt::test]
    async fn test_resolve_host_id() {
        let os = SUT::fast_boot().await;

        assert_eq!(
            os.resolve_host_id().await.unwrap(),
            os.host_id().await.unwrap()
        )
    }

    #[actix_rt::test]
    async fn test_resolve_host_info() {
        let os = SUT::fast_boot().await;

        assert_eq!(os.resolve_host_info().await, os.host_info().await)
    }
}
