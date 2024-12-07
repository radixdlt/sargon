use std::future::Future;

use crate::prelude::*;

pub struct AutomaticShieldBuilder;

impl AutomaticShieldBuilder {
    pub async fn build<Fut>(
        all_factors: Vec<FactorSource>,
        pick_primary_role_factors: impl Fn(Vec<FactorSource>) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = Vec<FactorSource>>,
    {
        todo!()
    }
}
