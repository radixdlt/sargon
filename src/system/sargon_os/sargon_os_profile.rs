use std::sync::RwLockWriteGuard;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub fn has_any_network(&self) -> bool {
        self.profile_holder
            .access_profile_with(|p| !p.networks.is_empty())
    }

    pub fn profile(&self) -> Profile {
        self.profile_holder.profile()
    }

    pub async fn import_profile(&self, profile: Profile) -> Result<()> {
        let device_info = self.device_info().await?;

        let mut profile = profile;

        profile.header.last_used_on_device = device_info;
        profile.header.last_modified = now(); // FIXME: find a reusable home for updating `last_modified`

        self.secure_storage
            .save_profile_and_active_profile_id(&profile)
            .await?;

        self.profile_holder.replace_profile_with(profile)
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
    /// Updates and **saves** profile to secure storage, after
    /// mutating it with `mutate`.
    pub(crate) async fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        let res = self.profile_holder.update_profile_with(mutate)?;
        self.save_existing_profile().await?;
        Ok(res)
    }

    pub(crate) async fn save_existing_profile(&self) -> Result<()> {
        self.save_profile(&self.profile()).await
    }

    pub(crate) async fn save_profile(&self, profile: &Profile) -> Result<()> {
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
    async fn no_networks_has_any_network_false() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT - nothing done.

        // ASSERT
        assert!(!os.has_any_network());
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
}
