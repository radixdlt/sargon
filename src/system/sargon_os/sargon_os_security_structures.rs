use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_source_ids(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.profile_holder.access_profile_with(|p| {
            p.security_structures_of_factor_source_ids()
        })
    }
}
