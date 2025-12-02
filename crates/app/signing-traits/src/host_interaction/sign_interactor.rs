use crate::prelude::*;

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

/// Driver to interact with natice Arculus CSDK library
#[cfg_attr(any(test, feature = "mock"), automock)]
/// An "interactor" which can sign signables (transaction intents & subintents) .
///
/// By "interactor" we mean a bridge between Sargon and Host application,
/// The SignaturesCollector (Sargon) will dispatch "request" to the Host application,
/// and async await the signed outcome.
#[async_trait::async_trait]
pub trait SignInteractor<S: Signable>: Send + Sync {
    async fn sign(
        &self,
        request: SignRequest<S>,
    ) -> Result<SignResponse<S::ID>>;
}
