use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SchematicsOfSecurityShields
    pub fn security_shields(&self) -> SchematicsOfSecurityShields {
        self.profile_holder
            .access_profile_with(|p| p.app_preferences.security.shields.clone())
    }
}
