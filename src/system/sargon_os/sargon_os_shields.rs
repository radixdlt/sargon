use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityShields
    pub fn security_shields(&self) -> SecurityShields {
        self.profile_holder
            .access_profile_with(|p| p.app_preferences.security.shields.clone())
    }
}
