use sargon::OsEntitiesLinkedToFactorSource;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns the entities linked to a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToFactorSource> {
        self.wrapped
            .entities_linked_to_factor_source(
                factor_source.into_internal(),
                profile_to_check.into_internal(),
            )
            .await
            .into_result()
    }
}
