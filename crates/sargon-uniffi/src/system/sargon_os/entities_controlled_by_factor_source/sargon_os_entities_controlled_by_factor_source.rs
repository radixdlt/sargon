use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns the entities controlled by a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn entities_controlled_by_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesControlledByFactorSource> {
        self.wrapped
            .entities_controlled_by_factor_source(
                factor_source.into_internal(),
                profile_to_check.into_internal(),
            )
            .await
            .into_result()
    }
}
