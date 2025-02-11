use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsNewFactorAdding {
    fn make_device_factor_adding_manager(&self) -> DeviceFactorAddingManager;

    async fn is_factor_already_in_use(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool>;

    async fn add_factor(&self, factor_source: FactorSource) -> Result<()>;
}

#[async_trait::async_trait]
impl OsNewFactorAdding for Arc<SargonOS> {
    fn make_device_factor_adding_manager(&self) -> DeviceFactorAddingManager {
        DeviceFactorAddingManager::new(Arc::clone(self))
    }

    async fn is_factor_already_in_use(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        Ok(self
            .profile()?
            .factor_sources
            .contains_by_id(&factor_source))
    }

    async fn add_factor(&self, factor_source: FactorSource) -> Result<()> {
        self.update_profile_with(|p| {
            p.factor_sources.append(factor_source.clone());
            Ok(())
        })
        .await?;
        // 1. Save factor source into profile
        // 2. Pre-derive and fill cache with instances for factor source
        // 3. If pre-derived -> return Result::Ok, if not -> delete factor source from profile
        // (assert that it's not used anywhere else) and return Result::Err(CommonError::FactorNotCreated)
        let result = self
            .pre_derive_and_fill_cache_with_instances_for_factor_source(
                factor_source.clone(),
            )
            .await;
        let fsid = factor_source.factor_source_id();

        if result.is_err() {
            self.update_profile_with(|p| {
                p.factor_sources.remove_id(&fsid);
                Ok(())
            })
            .await?;
            result?;
        }

        Ok(())
    }
}
