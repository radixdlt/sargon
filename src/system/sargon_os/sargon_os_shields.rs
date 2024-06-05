use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSourceIDs
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> SecurityStructuresOfFactorSourceIDs {
        self.profile_holder.access_profile_with(|p| {
            p.app_preferences
                .security
                .security_structures_of_factor_source_ids
                .clone()
        })
    }
}
