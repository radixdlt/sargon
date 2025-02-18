use crate::prelude::*;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "clients" which are
/// created from "drivers" which the hosts (iOS/Android wallets) "installs"
/// during app launch, enabling the  Sargon "Operating System" to e.g read/write
/// to secure storage and make use of the network connection of the iPhone/Android
/// phone.
pub struct SargonOS {
    pub(crate) profile_state_holder: ProfileStateHolder,
    pub(crate) clients: Clients,
    pub(crate) interactors: Interactors,
    pub(crate) host_id: HostId,
}

pub trait WithBios: Sized {
    fn new(bios: Arc<Bios>) -> Self;
}
impl WithBios for Clients {
    fn new(bios: Arc<Bios>) -> Self {
        Self::with_drivers(bios.drivers.clone())
    }
}

/// So that we do not have to go through `self.clients`,
/// but can use e.g. `self.secure_storage` directly.
impl std::ops::Deref for SargonOS {
    type Target = Clients;

    fn deref(&self) -> &Self::Target {
        &self.clients
    }
}

impl SargonOS {
    pub async fn boot(bios: Arc<Bios>, interactors: Interactors) -> Arc<Self> {
        let clients = Clients::new(bios);
        Self::boot_with_clients_and_interactor(clients, interactors).await
    }

    pub async fn boot_with_clients_and_interactor(
        clients: Clients,
        interactors: Interactors,
    ) -> Arc<Self> {
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

        let host_id = Self::get_host_id(&clients).await;
        let os = Arc::new(Self {
            clients,
            profile_state_holder: ProfileStateHolder::new(
                profile_state.clone(),
            ),
            interactors,
            host_id,
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

    pub async fn new_wallet(
        &self,
        should_pre_derive_instances: bool,
    ) -> Result<()> {
        if should_pre_derive_instances {
            #[cfg(not(test))]
            warn!("Pre-deriving instances is not supported in production yet. Param `should_pre_derive_instances` ignored.");
        }
        self.new_wallet_with_mnemonic(None, should_pre_derive_instances)
            .await
    }

    pub async fn new_wallet_with_mnemonic(
        &self,
        mnemonic: Option<MnemonicWithPassphrase>,
        should_pre_derive_instances: bool,
    ) -> Result<()> {
        let (profile, bdfs) =
            self.create_new_profile_with_bdfs(mnemonic).await?;

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

        if should_pre_derive_instances {
            #[cfg(debug_assertions)]
            // only tests for now, need more work in hosts before we can do this in prod
            self.pre_derive_and_fill_cache_with_instances_for_factor_source(
                bdfs.clone().factor_source.into(),
            )
            .await?;
        }

        info!("Saved new Profile and BDFS, finish creating wallet");

        Ok(())
    }

    pub async fn pre_derive_and_fill_cache_with_instances_for_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorInstancesProviderOutcome> {
        if !factor_source.factor_source_id().is_hash() {
            panic!("Unsupported FactorSource which is not HD.")
        }
        let profile_snapshot = self.profile()?;
        let keys_derivation_interactors = self.keys_derivation_interactor();
        let outcome = CacheFiller::for_new_factor_source(
            Arc::new(self.clients.factor_instances_cache.clone()),
            Arc::new(profile_snapshot),
            factor_source.clone(),
            NetworkID::Mainnet, // we care not about other networks here
            keys_derivation_interactors.clone(),
        )
        .await?;

        assert!(outcome.per_derivation_preset.values().all(|pf| pf
            .per_factor
            .keys()
            .collect_vec()
            == vec![&factor_source.id_from_hash()]));

        Ok(outcome)
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

            // Must save to secure storage first, then add, so that
            // we easily can implement KeysDerivationInteractors that
            // can try to load the mnemonic from secure storage when
            // we are Cache filling using the FactorInstancesProvider
            self.secure_storage
                .save_private_hd_factor_source(&bdfs)
                .await?;

            let bdfs_result = self
                .add_factor_source(FactorSource::from(
                    bdfs.clone().factor_source,
                ))
                .await;
            if let Some(error) = bdfs_result.err() {
                self.secure_storage.delete_profile(imported_id).await?;
                return Err(error);
            }
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
                    .clone()
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

        let host_info = self.host_info().await;

        let profile = Profile::from_device_factor_source(
            hd_factor_source.factor_source,
            self.host_id,
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

    pub fn sign_transactions_interactor(
        &self,
    ) -> Arc<dyn SignInteractor<TransactionIntent>> {
        self.interactors.use_factor_sources_interactor.clone()
            as Arc<dyn SignInteractor<TransactionIntent>>
    }

    pub fn sign_subintents_interactor(
        &self,
    ) -> Arc<dyn SignInteractor<Subintent>> {
        self.interactors.use_factor_sources_interactor.clone()
            as Arc<dyn SignInteractor<Subintent>>
    }

    pub fn sign_auth_interactor(&self) -> Arc<dyn SignInteractor<AuthIntent>> {
        self.interactors.use_factor_sources_interactor.clone()
            as Arc<dyn SignInteractor<AuthIntent>>
    }

    pub fn keys_derivation_interactor(
        &self,
    ) -> Arc<dyn KeyDerivationInteractor> {
        self.interactors.use_factor_sources_interactor.clone()
            as Arc<dyn KeyDerivationInteractor>
    }

    pub fn authorization_interactor(&self) -> Arc<dyn AuthorizationInteractor> {
        self.interactors.authorization_interactor.clone()
    }

    pub fn spot_check_interactor(&self) -> Arc<dyn SpotCheckInteractor> {
        self.interactors.spot_check_interactor.clone()
    }

    pub fn host_id(&self) -> HostId {
        self.host_id
    }

    pub async fn resolve_host_info(&self) -> HostInfo {
        self.host_info().await
    }
}

impl SargonOS {
    pub async fn create_new_profile_with_bdfs(
        &self,
        mnemonic_with_passphrase: Option<MnemonicWithPassphrase>,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        debug!("Creating new Profile and BDFS");

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
            Header::new(DeviceInfo::new_from_info(&self.host_id, &host_info)),
            FactorSources::with_bdfs(private_bdfs.factor_source.clone()),
            AppPreferences::default(),
            ProfileNetworks::default(),
        );
        info!("Created new (unsaved) Profile with ID {}", profile.id());
        Ok((profile, private_bdfs))
    }

    pub(crate) async fn get_host_id(clients: &Clients) -> HostId {
        debug!("Get Host ID");
        let secure_storage = &clients.secure_storage;
        let stored_host_id = secure_storage.load_host_id().await;

        match stored_host_id {
            Ok(Some(loaded_host_id)) => {
                debug!("Found saved host id: {:?}", &loaded_host_id);
                loaded_host_id
            }
            Ok(None) => {
                debug!("Found no saved host id, creating new.");
                let new_host_id = HostId::generate_new();
                debug!("Created new host id: {:?}", &new_host_id);

                let save_result =
                    secure_storage.save_host_id(&new_host_id).await;
                if let Err(error) = save_result {
                    debug!("Failed to save new host id {:?}", error);
                } else {
                    debug!("Saved new host id");
                }
                new_host_id
            }
            Err(error) => {
                debug!("Failed to load the host id {:?}", error);

                HostId::generate_new()
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

pub const SARGON_OS_TEST_MAX_ASYNC_DURATION: std::time::Duration =
    std::time::Duration::from_millis(50);

#[cfg(debug_assertions)]
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

    pub async fn boot_test_with_bdfs_mnemonic_and_interactors(
        bdfs_mnemonic: impl Into<Option<MnemonicWithPassphrase>>,
        derivation_interactor: impl Into<Option<Arc<dyn KeyDerivationInteractor>>>,
        authorization_interactor: impl Into<
            Option<Arc<dyn AuthorizationInteractor>>,
        >,
        spot_check_interactor: impl Into<Option<Arc<dyn SpotCheckInteractor>>>,
        pre_derive_factor_instance_for_bdfs: bool,
    ) -> Result<Arc<Self>> {
        let test_drivers =
            Drivers::with_file_system(InMemoryFileSystemDriver::new());
        let bios = Bios::new(test_drivers);
        let mut clients = Clients::new(bios.clone());
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let keys_derivation_interactor =
            derivation_interactor.into().unwrap_or_else(|| {
                Arc::new(TestDerivationInteractor::new(
                    false,
                    Arc::new(clients.secure_storage.clone()),
                ))
            });

        let authorization_interactor =
            authorization_interactor.into().unwrap_or_else(|| {
                Arc::new(TestAuthorizationInteractor::stubborn_authorizing())
            });

        let spot_check_interactor =
            spot_check_interactor.into().unwrap_or_else(|| {
                Arc::new(TestSpotCheckInteractor::new_succeeded())
            });

        let os = Self::boot_with_clients_and_interactor(
            clients,
            Interactors::new_with_derivation_authorization_and_spot_check_interactor(
                keys_derivation_interactor,
                authorization_interactor,
                spot_check_interactor
            ),
        )
        .await;
        os.new_wallet_with_mnemonic(
            bdfs_mnemonic.into(),
            pre_derive_factor_instance_for_bdfs,
        )
        .await?;

        os.update_profile_with(|p| {
            // Append Mainnet network since initial profile has no network
            p.networks
                .append(ProfileNetwork::new_empty_on(NetworkID::Mainnet));
            Ok(())
        })
        .await?;

        Ok(os)
    }

    pub async fn fast_boot() -> Arc<Self> {
        Self::fast_boot_bdfs(None).await
    }

    pub async fn fast_boot_bdfs_and_interactor(
        bdfs_mnemonic: impl Into<Option<MnemonicWithPassphrase>>,
        derivation_interactor: impl Into<Option<Arc<dyn KeyDerivationInteractor>>>,
        pre_derive_factor_instance_for_bdfs: bool,
    ) -> Arc<Self> {
        let req = Self::boot_test_with_bdfs_mnemonic_and_interactors(
            bdfs_mnemonic,
            derivation_interactor,
            None,
            None,
            pre_derive_factor_instance_for_bdfs,
        );

        actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn fast_boot_bdfs(
        bdfs_mnemonic: impl Into<Option<MnemonicWithPassphrase>>,
    ) -> Arc<Self> {
        Self::fast_boot_bdfs_and_interactor(bdfs_mnemonic, None, true).await
    }

    pub async fn boot_test() -> Result<Arc<Self>> {
        Self::boot_test_with_bdfs_mnemonic_and_interactors(
            None, None, None, None, true,
        )
        .await
    }

    /// Boot the SargonOS with a mocked networking driver.
    /// This is useful for testing the SargonOS without needing to connect to the internet.
    pub async fn boot_test_with_networking_driver(
        networking: Arc<dyn NetworkingDriver>,
    ) -> Result<Arc<Self>> {
        let drivers = Drivers::with_networking(networking);
        let bios = Bios::new(drivers);
        let mut clients = Clients::new(bios);
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os =
            Self::boot_with_clients_and_interactor(clients, interactors).await;

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

    pub async fn boot_test_empty_wallet_with_spot_check_interactor(
        spot_check_interactor: Arc<dyn SpotCheckInteractor>,
    ) -> Arc<Self> {
        let mut clients = Clients::new(Bios::new(Drivers::test()));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors =
            Interactors::new_from_clients_and_spot_check_interactor(
                &clients,
                spot_check_interactor,
            );
        let os = actix_rt::time::timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            Self::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        // Create empty Wallet
        os.with_timeout(|x| x.new_wallet(false)).await.unwrap();

        os
    }
}

#[cfg(test)]
mod tests {

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
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        // ACT
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        // ASSERT
        let active_profile = os.profile();
        assert_eq!(active_profile.unwrap().id(), profile.id());
    }

    #[actix_rt::test]
    async fn profile_not_loaded_error() {
        let os = SUT::fast_boot().await;
        os.profile_state_holder
            .replace_profile_state_with(ProfileState::None)
            .unwrap();

        let result = os.profile();

        assert!(matches!(
            result,
            Err(CommonError::ProfileStateNotLoaded { .. })
        ));
    }

    #[actix_rt::test]
    async fn test_boot_when_existing_profile_with_no_networks_profile_state_considered_none(
    ) {
        // ARRANGE
        let mut clients = Clients::new(Bios::new(Drivers::test()));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;
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
        let mut clients = Clients::new(Bios::new(Drivers::test()));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let new_os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

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
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        os.new_wallet(false).await.unwrap();

        let profile = os.profile().unwrap();
        let bdfs = profile.main_bdfs();

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

        assert_eq!(
            os.profile().unwrap().main_bdfs(),
            profile_to_import.main_bdfs()
        );
    }

    #[actix_rt::test]
    async fn test_wallet_import_with_bdfs_skip() {
        let os = SUT::fast_boot().await;
        let profile_to_import = Profile::sample();

        os.import_wallet(&profile_to_import, true).await.unwrap();

        assert_ne!(
            os.profile().unwrap().main_bdfs(),
            profile_to_import.main_bdfs()
        );
    }

    #[actix_rt::test]
    async fn test_new_wallet_through_derived_bdfs() {
        let test_drivers = Drivers::test();
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

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
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

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
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let other_hd =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let invalid_account = Account::new(
            other_hd._derive_entity_creation_factor_instance(
                NetworkID::Mainnet,
                HDPathComponent::unsecurified_hardened(0).unwrap(),
            ),
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
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;
        os.new_wallet(false).await.unwrap();
        let profile = os.profile().unwrap();
        let bdfs = profile.main_bdfs();

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

        assert_eq!(SargonOS::get_host_id(&os.clients).await, os.host_id())
    }

    #[actix_rt::test]
    async fn test_resolve_host_info() {
        let os = SUT::fast_boot().await;

        assert_eq!(os.resolve_host_info().await, os.host_info().await)
    }
}
