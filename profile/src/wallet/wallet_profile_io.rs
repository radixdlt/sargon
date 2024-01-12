use crate::prelude::*;

//========
// Wallet + SecureStorage
//========
impl Wallet {
    pub(crate) fn save_profile(&self, profile: &Profile) -> Result<()> {
        self.wallet_client_storage.save(
            SecureStorageKey::ProfileSnapshot {
                profile_id: profile.header.id.clone(),
            },
            profile,
        )
    }
    pub(crate) fn save_active_profile_id(&self, profile_id: &ProfileID) -> Result<()> {
        self.wallet_client_storage
            .save(SecureStorageKey::ActiveProfileID, profile_id)
    }
    pub(crate) fn save_active_profile_id_or_panic(&self, profile_id: &ProfileID) {
        match self.save_active_profile_id(profile_id) {
            Ok(_) => log::info!("Successfully saved active ProfileID: {}", profile_id),
            Err(e) => log::error!(
                "Failed to save active ProfileID: {}, error: {}",
                profile_id,
                e
            ),
        }
    }

    pub(crate) fn save_existing_profile(&self) -> Result<()> {
        self.save_profile(&self.profile())
    }

    pub(crate) fn save_profile_or_panic(&self, profile: &Profile) -> bool {
        match self.save_profile(profile) {
            Ok(_) => {
                log::info!("Successfully saved profile with ID: {}", profile.id());
                true
            }
            Err(e) => {
                log::error!(
                    "Failed to save profile with ID: {}, error: {}",
                    profile.id(),
                    e
                );
                false
            }
        }
    }
    pub(crate) fn save_new_profile_or_panic(&self, profile: &Profile) {
        if self.save_profile_or_panic(profile) {
            self.save_active_profile_id_or_panic(&profile.id());
        }
    }
}
