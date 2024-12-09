use crate::prelude::*;
use sargon::AuthenticationSigningInteractor as InternalAuthenticationSigningInteractor;
use sargon::AuthenticationSigningRequest as InternalAuthenticationSigningInteractorRequest;
use sargon::AuthenticationSigningResponse as InternalAuthenticationSigningResponse;
use sargon::KeyDerivationInteractor as InternalKeyDerivationInteractor;
use sargon::KeyDerivationRequest as InternalKeyDerivationRequest;
use sargon::KeyDerivationResponse as InternalKeyDerivationResponse;
use sargon::Result as InternalResult;
use sargon::SignInteractor as InternalSignInteractor;
use sargon::Subintent as InternalSubintent;
use sargon::SubintentHash as InternalSubintentHash;
use sargon::TransactionIntent as InternalTransactionIntent;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;
use sargon::UseFactorSourcesInteractor as InternalUseFactorSourcesInteractor;

type InternalSignRequestForTransactionIntent =
    sargon::SignRequest<InternalTransactionIntent>;
type InternalSignWithFactorsOutcomeForTransactionIntent =
    sargon::SignWithFactorsOutcome<InternalTransactionIntentHash>;
type InternalSignRequestForSubintent = sargon::SignRequest<InternalSubintent>;
type InternalSignWithFactorsOutcomeForSubintent =
    sargon::SignWithFactorsOutcome<InternalSubintentHash>;

/// Sargon os
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HostInteractor: Send + Sync + std::fmt::Debug {
    async fn sign_transactions(
        &self,
        request: SignRequestOfTransactionIntent,
    ) -> Result<SignWithFactorsOutcomeOfTransactionIntentHash>;

    async fn sign_subintents(
        &self,
        request: SignRequestOfSubintent,
    ) -> Result<SignWithFactorsOutcomeOfSubintentHash>;

    async fn derive_keys(
        &self,
        request: KeyDerivationRequest,
    ) -> Result<KeyDerivationResponse>;

    async fn sign_auth(
        &self,
        request: AuthenticationSigningRequest,
    ) -> Result<AuthenticationSigningResponse>;
}

#[derive(Debug)]
pub struct UseFactorSourcesInteractorAdapter {
    pub wrapped: Arc<dyn HostInteractor>,
}

impl UseFactorSourcesInteractorAdapter {
    pub fn new(wrapped: Arc<dyn HostInteractor>) -> Self {
        Self { wrapped }
    }
}

impl InternalUseFactorSourcesInteractor for UseFactorSourcesInteractorAdapter {}

#[async_trait::async_trait]
impl InternalSignInteractor<InternalTransactionIntent>
    for UseFactorSourcesInteractorAdapter
{
    async fn sign(
        &self,
        request: InternalSignRequestForTransactionIntent,
    ) -> InternalResult<InternalSignWithFactorsOutcomeForTransactionIntent>
    {
        self.wrapped
            .sign_transactions(request.into())
            .await
            .into_internal_result()
    }
}

#[async_trait::async_trait]
impl InternalSignInteractor<InternalSubintent>
    for UseFactorSourcesInteractorAdapter
{
    async fn sign(
        &self,
        request: InternalSignRequestForSubintent,
    ) -> InternalResult<InternalSignWithFactorsOutcomeForSubintent> {
        self.wrapped
            .sign_subintents(request.into())
            .await
            .into_internal_result()
    }
}

#[async_trait::async_trait]
impl InternalKeyDerivationInteractor for UseFactorSourcesInteractorAdapter {
    async fn derive(
        &self,
        request: InternalKeyDerivationRequest,
    ) -> InternalResult<InternalKeyDerivationResponse> {
        self.wrapped
            .derive_keys(request.into())
            .await
            .into_internal_result()
    }
}

#[async_trait::async_trait]
impl InternalAuthenticationSigningInteractor
    for UseFactorSourcesInteractorAdapter
{
    async fn sign(
        &self,
        request: InternalAuthenticationSigningInteractorRequest,
    ) -> InternalResult<InternalAuthenticationSigningResponse> {
        self.wrapped
            .sign_auth(request.into())
            .await
            .into_internal_result()
    }
}
