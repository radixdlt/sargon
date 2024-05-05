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
        self.delete_profile_and_mnemonics().await?;

        let (profile, bdfs) = self.new_profile_and_bdfs().await?;

        let profile_id = profile.id();

        self.clients
            .secure_storage
            .save_private_hd_factor_source(&bdfs)
            .await?;

        self.clients
            .secure_storage
            .save_profile_and_active_profile_id(&profile)
            .await?;

        self.profile_holder.replace_profile_with(profile)?;

        Ok(profile_id)
    }
}
