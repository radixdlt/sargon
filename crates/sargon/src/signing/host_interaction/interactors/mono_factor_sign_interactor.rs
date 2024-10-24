use crate::prelude::*;

/// An interactor for a factor source kind which supports performing
/// *Batch* signing *serially*.
///
/// Meaning we initiate and prompt user for signing with one factor source
/// at a time, where each signing operation is support batch signing, that is
/// signing multiple transactions each with multiple keys (derivations paths).
///
/// The user might choose to SKIP the current factor source, and move on to the
/// next one.
///
/// Example of a MonoFactor Batch Signing Driver is SecurityQuestionsFactorSource,
/// where it does not make any sense to let user in poly answer multiple
/// questions from different security questions factor sources (in fact we
/// might not even allow multiple SecurityQuestionsFactorSources to be used).
#[async_trait::async_trait]
pub trait MonoFactorSignInteractor<SP: SignablePayload> {
    async fn sign(
        &self,
        request: MonoFactorSignRequest<SP>,
    ) -> SignWithFactorsOutcome<SP::PayloadId>;
}
