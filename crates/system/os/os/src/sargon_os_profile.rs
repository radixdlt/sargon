#![allow(deprecated)]

use crate::prelude::*;

impl SargonOS {
    /// Returns `true` if claim was needed, i.e. if `profile.header.last_used_on_device` was
    /// different than `device_info` before claim occurred.
    fn claim_provided_profile(
        profile: &mut Profile,
        device_info: DeviceInfo,
    ) -> bool {
        let was_needed =
            profile.header.last_used_on_device.id != device_info.id;
        profile.update_header(device_info);
        was_needed
    }

    /// "Claims" the `profile`, meaning the last_used_on_device is updated in the
    /// header
    pub fn claim_profile(&self, profile: &mut Profile) {
        debug!("Claiming profile, id: {}", &profile.id());
        let host_info = self.host_info();
        let claiming_device_info =
            DeviceInfo::new_from_info(&self.host_id, &host_info);

        Self::claim_provided_profile(profile, claiming_device_info);
        info!(
            "Claimed profile, id: {}, with device info: {}",
            &profile.id(),
            host_info
        );
    }
}

impl SargonOS {
    pub async fn set_profile(&self, profile: Profile) -> Result<()> {
        if let Ok(current_profile) = self.profile() {
            if current_profile.id() != profile.id() {
                return Err(
                    CommonError::TriedToUpdateProfileWithOneWithDifferentID,
                );
            }

            self.update_profile_with(|p| {
                *p = profile.clone();
                Ok(())
            })
            .await?;
        } else {
            self.profile_state_holder.replace_profile_state_with(
                ProfileState::Loaded(profile.clone()),
            )?;
            self.save_existing_profile().await?;

            self.clients
                .profile_state_change
                .emit(ProfileState::Loaded(profile))
                .await;
        };

        Ok(())
    }

    /// Checks if current Profile contains any `ProfileNetwork`s.
    pub fn has_any_network(&self) -> Result<bool> {
        self.profile_state_holder
            .access_profile_with(|p| !p.networks.is_empty())
    }

    /// Has **any** account, at all, including hidden, on any network.
    pub fn has_any_account_on_any_network(&self) -> Result<bool> {
        self.profile_state_holder
            .access_profile_with(|p| p.has_any_account_on_any_network())
    }

    /// Imports the `profile`, claims it, set it as active (current) one and
    /// saves it into secure storage (with the claim modification).
    ///
    /// # Emits Event
    /// Emits `EventNotification::new(Event::ProfileImported))` event if successful.
    pub async fn import_profile(&self, profile: Profile) -> Result<()> {
        let imported_id = profile.id();
        debug!("Importing profile, id: {}", imported_id);
        let mut profile = profile;
        self.claim_profile(&mut profile);

        self.secure_storage.save_profile(&profile).await?;

        debug!(
            "Saved imported profile into secure storage, id: {}",
            imported_id
        );
        self.profile_state_holder
            .replace_profile_state_with(ProfileState::Loaded(profile))?;
        debug!(
            "Replaced held profile with imported one, id: {}",
            imported_id
        );

        self.event_bus
            .emit(EventNotification::new(Event::ProfileImported {
                id: imported_id,
            }))
            .await;

        info!("Successfully imported profile, id: {}", imported_id);

        Ok(())
    }

    pub fn profile(&self) -> Result<Profile> {
        self.profile_state_holder.profile()
    }
}

impl SargonOS {
    /// Updates and **saves** profile to secure storage, after
    /// mutating it with `mutate`, optionally validating ownership of Profile
    /// first.
    ///
    /// The only function to pass `false` to the `validate_ownership` parameter
    /// is the `SargonOS::claim_active_profile` method.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(&mut Profile) -> Result<R>,
    {
        let res = self.profile_state_holder.update_profile_with(mutate)?;
        let profile = self.profile_state_holder.update_profile_with(|p| {
            p.update_header(None);
            Ok(p.clone())
        })?;
        self.save_existing_profile()
            // tarpaulin will incorrectly flag next line is missed
            .await?;

        self.clients
            .profile_state_change
            .emit(ProfileState::Loaded(profile))
            .await;

        Ok(res)
    }

    ///  Saves the **active** profile into secure storage, if profile is 'owned by host'.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub(crate) async fn save_existing_profile(&self) -> Result<()> {
        let profile = &self.profile()?;
        self.save_profile(profile).await
    }

    ///  Saves **provided** `profile`` into secure storage, if it's 'owned by host'.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub(crate) async fn save_profile(&self, profile: &Profile) -> Result<()> {
        let secure_storage = &self.secure_storage;

        secure_storage
            .save(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id(),
                },
                profile,
            )
            .await?;

        self.event_bus
            .emit(EventNotification::new(Event::ProfileSaved))
            .await;

        Ok(())
    }

    /// Deletes the profile and all references Device
    /// factor sources from secure storage, does **NOT** change the in-memory
    /// profile in `profile_state_holder`.
    async fn delete_profile_and_mnemonics(&self) -> Result<()> {
        let secure_storage = &self.secure_storage;
        let device_factor_sources = self
            .profile_state_holder
            .access_profile_with(|p| p.device_factor_sources())?;

        for dfs in device_factor_sources.iter() {
            secure_storage.delete_mnemonic(&dfs.id).await?
        }

        secure_storage.delete_profile(self.profile()?.id()).await?;
        Ok(())
    }

    /// Deletes the profile and all references Device factor sources from secure storage.
    ///
    /// This method is typically only relevant for testing purposes, emulating a
    /// fresh install of wallet apps, wallet apps can call this method and then
    /// force quit, which should be equivalent with a fresh install of the app.
    pub async fn delete_profile_and_mnemonics_replace_in_memory_with_none(
        &self,
    ) -> Result<()> {
        self.delete_profile_and_mnemonics().await?;
        self.profile_state_holder
            .replace_profile_state_with(ProfileState::None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn new_profile_has_a_mainnet_network_which_is_empty() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT - nothing done.

        // ASSERT
        assert_eq!(
            os.current_network().unwrap(),
            ProfileNetwork::new_empty_on(NetworkID::Mainnet)
        );
    }

    #[actix_rt::test]
    async fn create_first_account_has_networks_is_true() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.create_and_save_new_unnamed_mainnet_account_with_main_bdfs()
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.has_any_network().unwrap());
    }

    #[actix_rt::test]
    async fn create_first_account_has_accounts_on_any_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| {
            x.create_and_save_new_unnamed_mainnet_account_with_main_bdfs()
        })
        .await
        .unwrap();

        // ASSERT
        assert!(os.has_any_account_on_any_network().unwrap());
    }

    #[actix_rt::test]
    async fn test_import_profile_is_current_by_id() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let p = Profile::sample();

        // ACT
        os.with_timeout(|x| x.import_profile(p.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().id(), p.id());
    }

    #[actix_rt::test]
    async fn test_import_profile_emits_event() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();

        let p = Profile::sample();

        // ACT
        os.with_timeout(|x| x.import_profile(p.clone()))
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::ProfileImported));
    }

    #[actix_rt::test]
    async fn test_import_profile_is_saved_into_storage() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let p = Profile::sample();

        // ACT
        os.with_timeout(|x| x.import_profile(p.clone()))
            .await
            .unwrap();

        // ASSERT
        let saved = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(saved.id(), p.id());
    }

    #[actix_rt::test]
    async fn test_import_profile_last_used_on_device_is_set() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.import_profile(Profile::sample()))
            .await
            .unwrap();

        // ASSERT
        let host_id = os.host_id;
        let host_info = os.host_info();
        assert_eq!(
            os.profile().unwrap().header.last_used_on_device,
            DeviceInfo::new_from_info(&host_id, &host_info)
        );
    }

    #[actix_rt::test]
    async fn test_import_profile_last_modified_is_set() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let profile = Profile::sample();
        let last_modified = &profile.header.last_modified;

        // ACT
        os.with_timeout(|x| x.import_profile(profile.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_ne!(&os.profile().unwrap().header.last_modified, last_modified);
    }

    #[actix_rt::test]
    async fn test_import_profile_is_claimed_and_can_be_edited() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let profile = Profile::sample();

        // ACT
        os.with_timeout(|x| x.import_profile(profile.clone()))
            .await
            .unwrap();

        let new_account = Account::sample_stokenet_paige();
        os.with_timeout(|x| x.add_account(new_account.clone()))
            .await
            .unwrap();

        // ASSERT
        assert!(os
            .profile()
            .unwrap()
            .networks
            .get_id(NetworkID::Stokenet)
            .unwrap()
            .accounts
            .contains_id(new_account.id()));

        let loaded = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap()
            .unwrap();
        assert!(loaded
            .networks
            .get_id(NetworkID::Stokenet)
            .unwrap()
            .accounts
            .contains_id(new_account.id()));
    }

    #[actix_rt::test]
    async fn test_delete_profile_then_create_new_with_bdfs_old_bdfs_is_deleted()
    {
        // ARRANGE
        let bdfs = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(bdfs.clone()).await;

        // ACT
        os.with_timeout(|x| {
            x.delete_profile_and_mnemonics_replace_in_memory_with_none()
        })
        .await
        .unwrap();

        // ASSERT
        let id = FactorSourceIDFromHash::new_for_device(&bdfs);
        let old_bdfs = os
            .with_timeout(|x| {
                x.secure_storage.load_mnemonic_with_passphrase(&id)
            })
            .await;

        assert!(old_bdfs.is_err());
    }

    #[actix_rt::test]
    async fn test_delete_profile_is_deleted() {
        // ARRANGE
        let bdfs = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(bdfs.clone()).await;

        // ACT
        os.with_timeout(|x| {
            x.delete_profile_and_mnemonics_replace_in_memory_with_none()
        })
        .await
        .unwrap();

        // ASSERT
        let load_current_profile = os
            .with_timeout(|x| x.secure_storage.load_profile())
            .await
            .unwrap();

        assert!(load_current_profile.is_none());
    }

    #[actix_rt::test]
    async fn test_set_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut profile = os.profile().unwrap();
        let new_network = ProfileNetwork::new(
            NetworkID::Stokenet,
            Accounts::just(Account::sample_stokenet()),
            Personas::new(),
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
        );

        profile.networks.append(new_network.clone());

        // ACT
        os.with_timeout(|x| x.set_profile(profile.clone()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.profile().unwrap().networks, profile.networks); // header has been updated so cannot do full profile comparison.
    }

    #[actix_rt::test]
    async fn test_set_profile_is_err_when_different_profile_id() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let res = os.with_timeout(|x| x.set_profile(Profile::sample())).await;

        // ASSERT
        assert_eq!(
            res,
            Err(CommonError::TriedToUpdateProfileWithOneWithDifferentID)
        );
    }

    #[actix_rt::test]
    async fn test_set_profile_when_no_profile_exists() {
        // ARRANGE
        let test_drivers = Drivers::test();
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SargonOS::boot_with_clients_and_interactor(clients, interactors)
                .await;

        // ACT
        let _ = os.with_timeout(|x| x.set_profile(Profile::sample())).await;

        // ASSERT
        assert_eq!(os.profile().unwrap(), Profile::sample());
    }
}
