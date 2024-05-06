use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn import_profile(&self, profile: Profile) -> Result<()> {
        let device_info = self.device_info().await?;

        let mut profile = profile;

        profile.header.last_used_on_device = device_info;

        self.clients
            .secure_storage
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
        self.clients
            .secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;

        self.clients
            .secure_storage
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
    /// Deletes the profile and the active profile id and all references Device
    /// factor sources from secure storage, does **NOT** change the in-memory
    /// profile in `profile_holder`.
    async fn delete_profile_and_mnemonics(&self) -> Result<()> {
        let secure_storage = &self.clients.secure_storage;
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
