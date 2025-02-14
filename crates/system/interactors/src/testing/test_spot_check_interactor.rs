use crate::prelude::*;

pub struct TestSpotCheckInteractor {
    request_count: RwLock<u32>,
    user: SpotCheckUser,
}

#[async_trait::async_trait]
impl SpotCheckInteractor for TestSpotCheckInteractor {
    async fn spot_check(
        &self,
        _factor_source: FactorSource,
        _allow_skip: bool,
    ) -> Result<SpotCheckResponse> {
        let result = match self.user.clone() {
            SpotCheckUser::Failed(common_error) => Err(common_error),
            SpotCheckUser::Succeeded => Ok(SpotCheckResponse::Valid),
            SpotCheckUser::SucceededFirstN(count, error) => {
                let request_count = *self
                    .request_count
                    .read()
                    .expect("Request count should not have been poisoned");

                if request_count < count.0 {
                    Ok(SpotCheckResponse::Valid)
                } else {
                    Err(error)
                }
            }
            SpotCheckUser::Skipped => Ok(SpotCheckResponse::Skipped),
        };

        self.request_count
            .write()
            .expect("Request count should not have been poisoned")
            .add_assign(1u32);

        result
    }
}

#[derive(Clone)]
pub enum SpotCheckUser {
    /// Spot check will fail with the provided error
    Failed(CommonError),

    /// Spot check will succeed
    Succeeded,

    /// The first n spot checks will succeed, the rest will fail with the provided error.
    SucceededFirstN(SpotCheckSucceededCount, CommonError),

    /// Spot check will be skipped
    Skipped,
}

#[derive(Clone)]
pub struct SpotCheckSucceededCount(u32);

impl TestSpotCheckInteractor {
    pub fn new(user: SpotCheckUser) -> Self {
        Self {
            request_count: RwLock::new(0),
            user,
        }
    }

    pub fn new_failed(common_error: CommonError) -> Self {
        Self::new(SpotCheckUser::Failed(common_error))
    }

    pub fn new_succeeded() -> Self {
        Self::new(SpotCheckUser::Succeeded)
    }

    pub fn new_succeeded_first_n(
        count: u32,
        common_error: CommonError,
    ) -> Self {
        Self::new(SpotCheckUser::SucceededFirstN(
            SpotCheckSucceededCount(count),
            common_error,
        ))
    }

    pub fn new_skipped() -> Self {
        Self::new(SpotCheckUser::Skipped)
    }
}
