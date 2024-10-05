#![allow(deprecated)]

use std::sync::RwLockWriteGuard;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn set_profile(&self, profile: Profile) -> Result<()> {
        map_result_from_internal(self.wrapped.set_profile(profil.into()).await)
    }

    /// Checks if current Profile contains any `ProfileNetwork`s.
    pub fn has_any_network(&self) -> Result<bool> {
        map_result_from_internal(self.wrapped.has_any_network())
    }

    /// Has **any** account, at all, including hidden, on any network.
    pub fn has_any_account_on_any_network(&self) -> Result<bool> {
        map_result_from_internal(self.wrapped.has_any_account_on_any_network())
    }

    /// Imports the `profile`, claims it, set it as active (current) one and
    /// saves it into secure storage (with the claim modification).
    ///
    /// # Emits Event
    /// Emits `EventNotification::new(Event::ProfileImported))` event if successful.
    pub async fn import_profile(&self, profile: Profile) -> Result<()> {
        map_result_from_internal(self.wrapped.import_profile(profile.into()).await)
    }

    pub fn profile(&self) -> Result<Profile> {
        map_result_from_internal(self.wrapped.profile())
    }
}