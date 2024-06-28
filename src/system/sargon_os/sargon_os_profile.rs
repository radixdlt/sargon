#![allow(deprecated)]

use std::sync::RwLockWriteGuard;

use crate::prelude::*;

impl SargonOS {
    /// Returns `true` if claim was needed, i.e. if `profile.header.last_used_on_device` was
    /// different than `device_info` before claim occurred.
    fn claim_provided_profile(
        profile: &mut Profile,
        device_info: DeviceInfo,
    ) -> bool {
        let was_needed = profile.header.last_used_on_device != device_info;
        profile.update_header(device_info);
        was_needed
    }

    /// "Claims" the `profile`, meaning the last_used_on_device is updated in the
    /// header
    pub async fn claim_profile(&self, profile: &mut Profile) -> Result<()> {
        debug!("Claiming profile, id: {}", &profile.id());
        let device_info = self.device_info().await?;
        Self::claim_provided_profile(profile, device_info.clone());
        info!(
            "Claimed profile, id: {}, with device info: {}",
            &profile.id(),
            device_info
        );
        Ok(())
    }
}

#[uniffi::export]
impl SargonOS {
    /// Checks if current Profile contains any `ProfileNetwork`s.
    pub fn has_any_network(&self) -> bool {
        self.profile_holder
            .access_profile_with(|p| !p.networks.is_empty())
    }

    /// Has **any** account, at all, including hidden, on any network.
    pub fn has_any_account_on_any_network(&self) -> bool {
        self.profile_holder
            .access_profile_with(|p| p.has_any_account_on_any_network())
    }

    /// Returns the current profile in full. This is a COSTLY operation
    /// and hosts SHOULD NOT do it lightheartedly, prefer using more specific
    /// reading operations such as `os.current_network_id` or `os.accounts_for_display_on_current_network` etc, which are cheap operations compared
    /// to using this.
    ///
    /// In the future will will most likely deprecate this method.
    pub fn profile(&self) -> Profile {
        self.profile_holder.profile()
    }

    #[allow(non_snake_case)]
    #[deprecated(
        since = "0.0.1",
        note = "Hosts SHOULD migrate to use more specialized methods on SargonOS instead, e.g. `createAndSaveNewAccount`. And SargonOS should be the SOLE object to perform the mutation and persisting."
    )]
    pub async fn DEPRECATED_save_ffi_changed_profile(
        &self,
        profile: Profile,
    ) -> Result<()> {
        if profile.id() != self.profile().id() {
            return Err(
                CommonError::TriedToUpdateProfileWithOneWithDifferentID,
            );
        }
        self.update_profile_with(|mut p| {
            *p = profile.clone();
            Ok(())
        })
        .await
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
        self.claim_profile(&mut profile).await?;

        self.secure_storage
            .save_profile_and_active_profile_id(&profile)
            .await?;

        debug!(
            "Saved imported profile into secure storage, id: {}",
            imported_id
        );

        self.profile_holder.replace_profile_with(profile)?;
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

    /// Claims the active profile, meaning the `last_used_on_device` in `header`
    /// is updated.
    ///
    /// Returns `true` if the profile was changed (i.e. if claim was indeed needed),
    /// `false`` otherwise.
    pub async fn claim_active_profile(&self) -> Result<bool> {
        let device_info = self.device_info().await?;

        self.maybe_validate_ownership_update_profile_with(
            false, // we do NOT validate ownership, since this method is claiming
            |mut p| {
                Ok(Self::claim_provided_profile(&mut p, device_info.clone()))
            },
        )
        .await
    }

    /// Deletes the profile and the active profile id and all references Device
    /// factor sources from secure storage, and creates a new empty profile
    /// and a new bdfs, and saves those into secure storage, returns the ID of
    /// the new profile.
    pub async fn delete_profile_then_create_new_with_bdfs(
        &self,
    ) -> Result<ProfileID> {
        let (profile, bdfs) = self
            .delete_profile_and_mnemonics_replace_in_memory_without_persisting()
            .await?;
        let profile_id = profile.id();
        self.secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;

        self.secure_storage
            .save_profile_and_active_profile_id(&profile)
            .await?;

        Ok(profile_id)
    }

    /// Do NOT use in production. Instead use `delete_profile_then_create_new_with_bdfs`
    /// in production. This method does not persist the new profile.
    pub async fn emulate_fresh_install(&self) -> Result<()> {
        warn!("Emulate fresh install of app. Will delete Profile and secrets from secure storage, without saving the new. BAD state.");
        let _ = self
            .delete_profile_and_mnemonics_replace_in_memory_without_persisting()
            .await?;
        Ok(())
    }
}

impl SargonOS {
    /// Returns `Err`` if the **active** profile is not 'owned by host',
    /// meaning `profile.header.last_used_on_device.id != device_info.id`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileUsedOnOtherDevice` if `profile` is not 'owned by
    /// host'.
    pub(crate) async fn validate_is_allowed_to_mutate_active_profile(
        &self,
    ) -> Result<()> {
        Self::validate_is_allowed_to_update_provided_profile(
            &self.clients,
            &self.profile(),
        )
        .await
    }

    /// Returns `Err` if the **provided** `profile` is not 'owned by host',
    /// meaning `profile.header.last_used_on_device.id != device_info.id`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileUsedOnOtherDevice` if `profile` is not 'owned by
    /// host'.
    pub(crate) async fn validate_is_allowed_to_update_provided_profile(
        clients: &Clients,
        profile: &Profile,
    ) -> Result<()> {
        Self::check_is_allowed_to_update_provided_profile(
            clients, profile, true,
        )
        .await?;
        Ok(())
    }

    /// Checks if the **provided** `profile` is not 'owned by host',
    /// meaning `profile.header.last_used_on_device.id != device_info.id`,
    /// and if `err_on_lack_of_ownership` an Err is returns, otherwise `Ok(false)`
    /// is returned.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileUsedOnOtherDevice` if `profile` is not 'owned by
    /// host'.
    pub(crate) async fn check_is_allowed_to_update_provided_profile(
        clients: &Clients,
        profile: &Profile,
        err_on_lack_of_ownership: bool,
    ) -> Result<bool> {
        debug!("Checking if profile.header.last_used_on_device is self.device_info");
        let device_info = Self::get_device_info(clients).await?;
        let last_used = profile.header.last_used_on_device.clone();
        if last_used == device_info {
            debug!("Ownership check passed (profile.header.last_used_on_device == self.device_info)");
            Ok(true)
        } else {
            warn!("Profile was last used on another device, will not be able to update it until it has been claimed.");
            clients
                .event_bus
                .emit(EventNotification::profile_used_on_other_device(
                    last_used.clone(),
                ))
                .await;
            if err_on_lack_of_ownership {
                Err(CommonError::ProfileUsedOnOtherDevice {
                    other_device_id: last_used.id,
                    this_device_id: device_info.id,
                })
            } else {
                // used by SargonOS::boot
                Ok(false)
            }
        }
    }

    /// Validates ownership of Profile, then updates and **saves** it to
    /// secure storage, after mutating it with `mutate`.
    ///
    /// # Emits
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub(crate) async fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        self.maybe_validate_ownership_update_profile_with(true, mutate)
            .await
    }

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
    pub(crate) async fn maybe_validate_ownership_update_profile_with<F, R>(
        &self,
        validate_ownership: bool, // should only ever pass `false` from `claim`
        mutate: F,
    ) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        if validate_ownership {
            self.validate_is_allowed_to_mutate_active_profile().await?;
        }
        let res = self.profile_holder.update_profile_with(mutate)?;
        self.profile_holder.update_profile_with(|mut p| {
            p.update_header(None);
            Ok(())
        })?;
        self.save_existing_profile()
            // tarpaulin will incorrectly flag next line is missed
            .await?;
        Ok(res)
    }

    ///  Saves the **active** profile into secure storage, if profile is 'owned by host'.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub(crate) async fn save_existing_profile(&self) -> Result<()> {
        self.save_profile(&self.profile()).await
    }

    ///  Saves **provided** `profile`` into secure storage, if it's 'owned by host'.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub(crate) async fn save_profile(&self, profile: &Profile) -> Result<()> {
        self.validate_is_allowed_to_mutate_active_profile().await?;

        let secure_storage = &self.secure_storage;

        secure_storage
            .save(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.header.id,
                },
                profile,
            )
            .await?;

        self.event_bus
            .emit(EventNotification::new(Event::ProfileSaved))
            .await;

        Ok(())
    }

    /// Deletes the profile and the active profile id and all references Device
    /// factor sources from secure storage, does **NOT** change the in-memory
    /// profile in `profile_holder`.
    async fn delete_profile_and_mnemonics(&self) -> Result<()> {
        let secure_storage = &self.secure_storage;
        let device_factor_sources = self
            .profile_holder
            .access_profile_with(|p| p.device_factor_sources());

        for dfs in device_factor_sources.iter() {
            secure_storage.delete_mnemonic(&dfs.id).await?
        }

        secure_storage.delete_profile(self.profile().id()).await?;
        secure_storage.delete_active_profile_id().await?;
        Ok(())
    }

    /// Deletes the profile and the active profile id and all references Device
    /// factor sources from secure storage, and creates a new empty profile
    /// and a new bdfs and replaces the in-memory profile held by profile_holder,
    /// **without** persisting the neither the profile nor the new BDFS to secure
    /// storage.
    ///
    /// This method is typically only relevant for testing purposes, emulating a
    /// fresh install of wallet apps, wallet apps can call this method and then
    /// force quit, which should be equivalent with a fresh install of the app.
    pub async fn delete_profile_and_mnemonics_replace_in_memory_without_persisting(
        &self,
    ) -> Result<(Profile, PrivateHierarchicalDeterministicFactorSource)> {
        self.delete_profile_and_mnemonics().await?;
        let (profile, bdfs) = self.new_profile_and_bdfs().await?;
        self.profile_holder.replace_profile_with(profile.clone())?;
        Ok((profile, bdfs))
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
            os.current_network(),
            ProfileNetwork::new_empty_on(NetworkID::Mainnet)
        );
    }

    #[actix_rt::test]
    async fn create_first_account_has_networks_is_true() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert!(os.has_any_network());
    }

    #[actix_rt::test]
    async fn create_first_account_has_accounts_on_any_network() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.create_and_save_new_unnamed_mainnet_account())
            .await
            .unwrap();

        // ASSERT
        assert!(os.has_any_account_on_any_network());
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
        assert_eq!(os.profile().id(), p.id());
    }

    #[actix_rt::test]
    async fn test_import_profile_emits_event() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
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
            .with_timeout(|x| x.secure_storage.load_active_profile())
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
        let device_info = os.device_info().await.unwrap();
        assert_eq!(os.profile().header.last_used_on_device, device_info);
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
        assert_ne!(&os.profile().header.last_modified, last_modified);
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
            .networks
            .get_id(NetworkID::Stokenet)
            .unwrap()
            .accounts
            .contains_id(new_account.id()));

        let loaded = os
            .with_timeout(|x| x.secure_storage.load_active_profile())
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
    async fn test_import_profile_active_profile_id_is_set() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.import_profile(Profile::sample()))
            .await
            .unwrap();

        // ASSERT
        let active_profile_id = os
            .with_timeout(|x| x.secure_storage.load_active_profile_id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(active_profile_id, os.profile().id());
    }

    #[actix_rt::test]
    async fn test_delete_profile_then_create_new_with_bdfs_old_bdfs_is_deleted()
    {
        // ARRANGE
        let bdfs = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(bdfs.clone()).await;

        // ACT
        os.with_timeout(|x| x.delete_profile_then_create_new_with_bdfs())
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
    async fn test_delete_profile_then_create_new_with_bdfs_old_profile_is_deleted(
    ) {
        // ARRANGE
        let bdfs = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(bdfs.clone()).await;
        let profile_id = os.profile().id();

        // ACT
        os.with_timeout(|x| x.delete_profile_then_create_new_with_bdfs())
            .await
            .unwrap();

        // ASSERT
        let load_old_profile_result = os
            .with_timeout(|x| x.secure_storage.load_profile_with_id(profile_id))
            .await;

        assert!(load_old_profile_result.is_err());
    }

    #[actix_rt::test]
    async fn test_delete_profile_then_create_new_with_bdfs_new_bdfs_is_saved() {
        // ARRANGE
        let bdfs = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(bdfs.clone()).await;

        // ACT
        os.with_timeout(|x| x.delete_profile_then_create_new_with_bdfs())
            .await
            .unwrap();

        // ASSERT
        let saved_bdfs = os
            .with_timeout(|x| x.main_bdfs_mnemonic_with_passphrase())
            .await
            .unwrap();

        assert_ne!(saved_bdfs, bdfs);
    }

    #[actix_rt::test]
    async fn test_delete_profile_then_create_new_with_bdfs_new_profile_is_saved(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let profile = Profile::sample();
        os.with_timeout(|x| x.import_profile(profile.clone()))
            .await
            .unwrap();

        // ACT
        os.with_timeout(|x| x.delete_profile_then_create_new_with_bdfs())
            .await
            .unwrap();

        // ASSERT
        let active_profile = os
            .with_timeout(|x| x.secure_storage.load_active_profile())
            .await
            .unwrap()
            .unwrap();

        assert_ne!(active_profile.id(), profile.id());
    }

    #[actix_rt::test]
    async fn test_delete_profile_then_create_new_with_bdfs_device_info_is_unchanged(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let device_info = os.with_timeout(|x| x.device_info()).await.unwrap();
        assert_eq!(&os.profile().header.creating_device, &device_info);

        // ACT
        os.with_timeout(|x| x.delete_profile_then_create_new_with_bdfs())
            .await
            .unwrap();

        // ASSERT
        let device_info = os.with_timeout(|x| x.device_info()).await.unwrap();
        assert_eq!(&os.profile().header.creating_device, &device_info);
    }

    #[actix_rt::test]
    async fn test_emulate_fresh_install_does_not_save_new() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let first = os.profile().id();

        // ACT
        os.with_timeout(|x| x.emulate_fresh_install())
            .await
            .unwrap();

        // ASSERT
        let second = os.profile().id();
        assert_ne!(second, first);
        let load_profile_res = os
            .with_timeout(|x| x.secure_storage.load_profile_with_id(second))
            .await;

        assert_eq!(
            load_profile_res,
            Err(CommonError::UnableToLoadProfileFromSecureStorage {
                profile_id: second
            })
        );
    }

    #[actix_rt::test]
    async fn test_deprecated_save_ffi_changed_profile() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let mut profile = os.profile();
        let new_network = ProfileNetwork::new(
            NetworkID::Stokenet,
            Accounts::just(Account::sample_stokenet()),
            Personas::new(),
            AuthorizedDapps::new(),
        );

        profile.networks.append(new_network.clone());

        // ACT
        os.with_timeout(|x| {
            x.DEPRECATED_save_ffi_changed_profile(profile.clone())
        })
        .await
        .unwrap();

        // ASSERT
        assert_eq!(os.profile().networks, profile.networks); // header has been updated so cannot do full profile comparison.
    }

    #[actix_rt::test]
    async fn test_deprecated_save_ffi_changed_profile_is_err_when_different_profile_id(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let res = os
            .with_timeout(|x| {
                x.DEPRECATED_save_ffi_changed_profile(Profile::sample())
            })
            .await;

        // ASSERT
        assert_eq!(
            res,
            Err(CommonError::TriedToUpdateProfileWithOneWithDifferentID)
        );
    }
}
