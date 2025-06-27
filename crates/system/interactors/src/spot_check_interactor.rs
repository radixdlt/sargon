use crate::prelude::*;

/// An interactor responsible for communicating with the user on host, to perform a spot check
/// on a factor source.
#[async_trait::async_trait]
pub trait SpotCheckInteractor: Send + Sync {
    /// Perform a spot check on the given `FactorSource`.
    /// If `allow_skip`, host should allow users to skip the spot check.
    async fn spot_check(
        &self,
        factor_source: FactorSource,
        allow_skip: bool,
    ) -> Result<SpotCheckResponse>;
}

/// An enum indicating the result of a spot check.
///
/// Note that there isn't a failure case since user never fails a spot check. It either:
/// - succeeds (`Valid` returned by host),
/// - skips (`Skipped` returned by host),
/// - aborts (`CommonError::HostInteractionAborted` thrown by host)
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum SpotCheckResponse {
    /// The factor source was successfully validated.
    Valid,

    /// The user skipped the spot check.
    Skipped,
}
