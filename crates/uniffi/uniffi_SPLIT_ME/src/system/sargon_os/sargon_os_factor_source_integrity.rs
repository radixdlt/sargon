use sargon::OsEntitiesLinkedToFactorSource;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns the entities linked to a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn factor_source_integrity(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorSourceIntegrity> {
        self.wrapped
            .integrity(factor_source.into_internal())
            .await
            .into_result()
    }
}
