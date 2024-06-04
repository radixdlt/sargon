use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SchematicOfSecurityShields
    pub fn security_shields(&self) -> SchematicOfSecurityShields {
        self.profile_holder
            .access_profile_with(|p| p.app_preferences.security.shields.clone())
    }
}
