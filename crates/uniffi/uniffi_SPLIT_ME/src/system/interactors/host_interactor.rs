use crate::prelude::*;
use sargon::AuthIntent as InternalAuthIntent;
use sargon::AuthIntentHash as InternalAuthIntentHash;
use sargon::AuthorizationInteractor as InternalAuthorizationInteractor;
use sargon::AuthorizationPurpose as InternalAuthorizationPurpose;
use sargon::AuthorizationResponse as InternalAuthorizationResponse;
use sargon::FactorSource as InternalFactorSource;
use sargon::KeyDerivationInteractor as InternalKeyDerivationInteractor;
use sargon::KeyDerivationRequest as InternalKeyDerivationRequest;
use sargon::KeyDerivationResponse as InternalKeyDerivationResponse;
use sargon::Result as InternalResult;
use sargon::SignInteractor as InternalSignInteractor;
use sargon::SpotCheckInteractor as InternalSpotCheckInteractor;
use sargon::SpotCheckResponse as InternalSpotCheckResponse;
use sargon::Subintent as InternalSubintent;
use sargon::SubintentHash as InternalSubintentHash;
use sargon::TransactionIntent as InternalTransactionIntent;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;
use sargon::UseFactorSourcesInteractor as InternalUseFactorSourcesInteractor;

type InternalSignRequestForTransactionIntent =
    sargon::SignRequest<InternalTransactionIntent>;
type InternalSignWithFactorsOutcomeForTransactionIntent =
    sargon::SignResponse<InternalTransactionIntentHash>;
type InternalSignRequestForSubintent = sargon::SignRequest<InternalSubintent>;
type InternalSignWithFactorsOutcomeForSubintent =
    sargon::SignResponse<InternalSubintentHash>;
type InternalSignRequestForAuthIntent = sargon::SignRequest<InternalAuthIntent>;
type InternalSignWithFactorsOutcomeForAuthIntent =
    sargon::SignResponse<InternalAuthIntentHash>;

/// Sargon os
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HostInteractor: Send + Sync + std::fmt::Debug {
    async fn sign_transactions(
        &self,
        request: SignRequestOfTransactionIntent,
    ) -> Result<SignResponseOfTransactionIntentHash>;

    async fn sign_subintents(
        &self,
        request: SignRequestOfSubintent,
    ) -> Result<SignResponseOfSubintentHash>;

    async fn derive_keys(
        &self,
        request: KeyDerivationRequest,
    ) -> Result<KeyDerivationResponse>;

    async fn sign_auth(
        &self,
        request: SignRequestOfAuthIntent,
    ) -> Result<SignResponseOfAuthIntentHash>;

    async fn request_authorization(
        &self,
        purpose: AuthorizationPurpose,
    ) -> AuthorizationResponse;

    async fn spot_check(
        &self,
        factor_source: FactorSource,
        allow_skip: bool,
    ) -> Result<SpotCheckResponse>;
}

#[derive(Debug)]
pub struct AuthorizationInteractorAdapter {
    pub wrapped: Arc<dyn HostInteractor>,
}

impl AuthorizationInteractorAdapter {
    pub fn new(wrapped: Arc<dyn HostInteractor>) -> Self {
        Self { wrapped }
    }
}

#[async_trait::async_trait]
impl InternalAuthorizationInteractor for AuthorizationInteractorAdapter {
    async fn request_authorization(
        &self,
        purpose: InternalAuthorizationPurpose,
    ) -> InternalAuthorizationResponse {
        self.wrapped
            .request_authorization(purpose.into())
            .await
            .into_internal()
    }
}

#[derive(Debug)]
pub struct SpotCheckInteractorAdapter {
    pub wrapped: Arc<dyn HostInteractor>,
}

impl SpotCheckInteractorAdapter {
    pub fn new(wrapped: Arc<dyn HostInteractor>) -> Self {
        Self { wrapped }
    }
}

#[async_trait::async_trait]
impl InternalSpotCheckInteractor for SpotCheckInteractorAdapter {
    async fn spot_check(
        &self,
        factor_source: InternalFactorSource,
        allow_skip: bool,
    ) -> InternalResult<InternalSpotCheckResponse> {
        self.wrapped
            .spot_check(factor_source.into(), allow_skip)
            .await
            .into_internal_result()
    }
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
impl InternalSignInteractor<InternalAuthIntent>
    for UseFactorSourcesInteractorAdapter
{
    async fn sign(
        &self,
        request: InternalSignRequestForAuthIntent,
    ) -> InternalResult<InternalSignWithFactorsOutcomeForAuthIntent> {
        self.wrapped
            .sign_auth(request.into())
            .await
            .into_internal_result()
    }
}
