use crate::prelude::*;

/// An "interactor" which can derive keys from a single factor source.
///
/// By "interactor" we mean a bridge between Sargon and Host application,
/// The KeysCollector (Sargon) will dispatch "request" to the Host application,
/// and async await a Result with either successful response - derived keys -
/// which it will use to update it internal state and continue with the next
/// factor source, or in case of failure the whole process will be aborted.
#[async_trait::async_trait]
pub trait MonoFactorKeyDerivationInteractor:
    Send + Sync + std::fmt::Debug
{
    async fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse>;
}
