use crate::prelude::*;

#[async_trait::async_trait]
pub trait SignInteractor<S: Signable>: Send + Sync {
    async fn sign(
        &self,
        request: SignRequest<S>,
    ) -> SignWithFactorsOutcome<S::ID>;
}