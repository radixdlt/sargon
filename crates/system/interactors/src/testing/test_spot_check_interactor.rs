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
            SpotCheckUser::Failure(common_error) => Err(common_error),
            SpotCheckUser::Valid => Ok(SpotCheckResponse::Valid),
            SpotCheckUser::Skipped => Ok(SpotCheckResponse::Skipped),
        }
    }
}

#[derive(Clone)]
pub enum SpotCheckUser {
    Failure(CommonError),
    Valid,
    Skipped,
}

impl TestSpotCheckInteractor {
    pub fn new(user: SpotCheckUser) -> Self {
        Self { user }
    }

    pub fn new_failing(common_error: CommonError) -> Self {
        Self::new(SpotCheckUser::Failure(common_error))
    }

    pub fn new_valid() -> Self {
        Self::new(SpotCheckUser::Valid)
    }

    pub fn new_skipped() -> Self {
        Self::new(SpotCheckUser::Skipped)
    }
}
