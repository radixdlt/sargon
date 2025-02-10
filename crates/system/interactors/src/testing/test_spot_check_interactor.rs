use crate::prelude::*;

pub struct TestSpotCheckInteractor {
    user: SpotCheckUser,
}

#[async_trait::async_trait]
impl SpotCheckInteractor for TestSpotCheckInteractor {
    async fn spot_check(
        &self,
        _factor_source: FactorSource,
        _allow_skip: bool,
    ) -> Result<SpotCheckResponse> {
        match self.user.clone() {
            SpotCheckUser::Failed(common_error) => Err(common_error),
            SpotCheckUser::Succeeded => Ok(SpotCheckResponse::Valid),
            SpotCheckUser::Skipped => Ok(SpotCheckResponse::Skipped),
        }
    }
}

#[derive(Clone)]
pub enum SpotCheckUser {
    Failed(CommonError),
    Succeeded,
    Skipped,
}

impl TestSpotCheckInteractor {
    pub fn new(user: SpotCheckUser) -> Self {
        Self { user }
    }

    pub fn new_failed(common_error: CommonError) -> Self {
        Self::new(SpotCheckUser::Failed(common_error))
    }

    pub fn new_succeeded() -> Self {
        Self::new(SpotCheckUser::Succeeded)
    }

    pub fn new_skipped() -> Self {
        Self::new(SpotCheckUser::Skipped)
    }
}
