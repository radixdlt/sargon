use crate::prelude::*;
use sargon::OsNewFactorAdding;

#[uniffi::export]
impl SargonOS {
    async fn is_factor_already_in_use(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        self.wrapped
            .is_factor_already_in_use(factor_source.into_internal())
            .await
            .into_result()
    }

    async fn add_factor(&self, factor_source: FactorSource) -> Result<()> {
        self.wrapped
            .add_factor(factor_source.into_internal())
            .await
            .into_result()
    }
}
