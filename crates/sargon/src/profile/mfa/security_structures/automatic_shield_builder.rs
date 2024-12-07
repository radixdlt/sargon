use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    all_factors: Vec<FactorSource>,
    picked_primary_role_factors: Vec<FactorSource>,
    shield_builder: SecurityShieldBuilder,
}

impl AutomaticShieldBuilder {
    fn find_primary_role_candidates(all: &[FactorSource]) -> Vec<FactorSource> {
        todo!()
    }

    fn build_shield(self) -> Result<SecurityStructureOfFactorSourceIDs> {
        todo!()
    }
}
impl AutomaticShieldBuilder {
    pub async fn build<Fut>(
        all_factors: Vec<FactorSource>,
        pick_primary_role_factors: impl Fn(Vec<FactorSource>) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = Vec<FactorSource>>,
    {
        let candidates = Self::find_primary_role_candidates(&all_factors);
        let picked = pick_primary_role_factors(candidates).await;
        let security_shield_builder = SecurityShieldBuilder::new();
        let auto_builder = Self {
            all_factors,
            picked_primary_role_factors: picked,
            shield_builder: security_shield_builder,
        };
        auto_builder.build_shield()
    }
}
