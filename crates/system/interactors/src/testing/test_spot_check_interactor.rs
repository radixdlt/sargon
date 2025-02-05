use crate::prelude::*;

pub struct TestSpotCheckInteractor {}

#[async_trait::async_trait]
impl SpotCheckInteractor for TestSpotCheckInteractor {
    async fn spot_check(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Result<SpotCheckResponse> {
        Err(CommonError::Unknown)
    }
}

impl TestSpotCheckInteractor {
    pub fn new_failing() -> Self {
        Self {}
    }
}
