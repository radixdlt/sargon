use crate::prelude::*;

pub struct TestUseFactorSourcesInteractors {
    transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
    subintent_signing: Arc<dyn SignInteractor<Subintent>>,
    key_derivation: Arc<dyn KeyDerivationInteractor>,
    auth_signing: Arc<dyn AuthenticationSigningInteractor>,
}

impl TestUseFactorSourcesInteractors {
    pub fn new(
        transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
        subintent_signing: Arc<dyn SignInteractor<Subintent>>,
        key_derivation: Arc<dyn KeyDerivationInteractor>,
        auth_signing: Arc<dyn AuthenticationSigningInteractor>,
    ) -> Self {
        Self {
            transaction_signing,
            subintent_signing,
            key_derivation,
            auth_signing,
        }
    }
}

impl UseFactorSourcesInteractor for TestUseFactorSourcesInteractors {}

#[async_trait::async_trait]
impl SignInteractor<TransactionIntent> for TestUseFactorSourcesInteractors {
    async fn sign(
        &self,
        request: SignRequest<TransactionIntent>,
    ) -> Result<SignWithFactorsOutcome<TransactionIntentHash>> {
        self.transaction_signing.sign(request).await
    }
}

#[async_trait::async_trait]
impl SignInteractor<Subintent> for TestUseFactorSourcesInteractors {
    async fn sign(
        &self,
        request: SignRequest<Subintent>,
    ) -> Result<SignWithFactorsOutcome<SubintentHash>> {
        self.subintent_signing.sign(request).await
    }
}

#[async_trait::async_trait]
impl KeyDerivationInteractor for TestUseFactorSourcesInteractors {
    async fn derive(
        &self,
        request: KeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        self.key_derivation.derive(request).await
    }
}

#[async_trait::async_trait]
impl AuthenticationSigningInteractor for TestUseFactorSourcesInteractors {
    async fn sign(
        &self,
        request: AuthenticationSigningRequest,
    ) -> Result<AuthenticationSigningResponse> {
        self.auth_signing.sign(request).await
    }
}
