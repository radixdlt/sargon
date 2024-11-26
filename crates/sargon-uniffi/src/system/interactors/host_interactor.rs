use crate::prelude::*;
use sargon::HostInteractor as InternalHostInteractor;
use sargon::SignInteractor as InternalSignInteractor;
use sargon::TransactionIntent as InternalTransactionIntent;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;
use sargon::Subintent as InternalSubintent;
use sargon::SubintentHash as InternalSubintentHash;
use sargon::KeyDerivationInteractor as InternalKeyDerivationInteractor;
use sargon::KeyDerivationRequest as InternalKeyDerivationRequest;
use sargon::KeyDerivationResponse as InternalKeyDerivationResponse;
use sargon::Result as InternalResult;

type InternalSignRequestForTransactionIntent = sargon::SignRequest<InternalTransactionIntent>;
type InternalSignWithFactorsOutcomeForTransactionIntent = sargon::SignWithFactorsOutcome<InternalTransactionIntentHash>;
type InternalSignRequestForSubintent = sargon::SignRequest<InternalSubintent>;
type InternalSignWithFactorsOutcomeForSubintent = sargon::SignWithFactorsOutcome<InternalSubintentHash>;

/// Sargon os
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HostInteractor: Send + Sync + std::fmt::Debug {
    async fn sign_transactions(
        &self,
        request: SignRequestForTransactionIntent,
    ) -> SignWithFactorsOutcomeForTransactionIntent;

    async fn sign_subintents(
        &self,
        request: SignRequestForSubintent,
    ) -> SignWithFactorsOutcomeForSubintent;

    async fn derive_keys(
        &self,
        request: KeyDerivationRequest,
    ) -> Result<KeyDerivationResponse>;

}

#[derive(Debug)]
pub struct HostInteractorAdapter {
    pub wrapped: Arc<dyn HostInteractor>,
}

impl HostInteractorAdapter {

    pub fn new(wrapped: Arc<dyn HostInteractor>) -> Self {
        Self { wrapped }
    }

}

impl InternalHostInteractor for HostInteractorAdapter {}

#[async_trait::async_trait]
impl InternalSignInteractor<InternalTransactionIntent> for HostInteractorAdapter {
    async fn sign(&self, request: InternalSignRequestForTransactionIntent) -> InternalSignWithFactorsOutcomeForTransactionIntent {
        self.wrapped
            .sign_transactions(request.into())
            .await
            .into()
    }
}

#[async_trait::async_trait]
impl InternalSignInteractor<InternalSubintent> for HostInteractorAdapter {
    async fn sign(&self, request: InternalSignRequestForSubintent) -> InternalSignWithFactorsOutcomeForSubintent {
        self.wrapped
            .sign_subintents(request.into())
            .await
            .into()
    }
}

#[async_trait::async_trait]
impl InternalKeyDerivationInteractor for HostInteractorAdapter {
    async fn derive(
        &self,
        request: InternalKeyDerivationRequest
    ) -> InternalResult<InternalKeyDerivationResponse> {
        self.wrapped
            .derive_keys(request.into())
            .await
            .into_internal_result()
    }
}
