use crate::prelude::*;

pub struct TestHostInteractor {
    transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
    subintent_signing: Arc<dyn SignInteractor<Subintent>>,
    key_derivation: Arc<dyn KeyDerivationInteractor>,
}

impl TestHostInteractor {

    // TODO this should be deleted
    pub fn new_from_bios(bios: Arc<Bios>) -> Self {
        let clients = Clients::new(bios);

        Self::new_with_derivation_interactor(
            Arc::new(TestDerivationInteractor::new(
                false,
                Arc::new(clients.secure_storage.clone()),
            ))
        )
    }

    pub fn new(
        transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
        subintent_signing: Arc<dyn SignInteractor<Subintent>>,
        key_derivation: Arc<dyn KeyDerivationInteractor>,
    ) -> Self {
        Self {
            transaction_signing,
            subintent_signing,
            key_derivation,
        }
    }

    pub fn new_with_derivation_interactor(
        key_derivation: Arc<dyn KeyDerivationInteractor>,
    ) -> Self {
        Self::new(
            Arc::new(TestSignInteractor::<TransactionIntent>::new(
                SimulatedUser::prudent_no_fail()
            )),
            Arc::new(TestSignInteractor::<Subintent>::new(
                SimulatedUser::prudent_no_fail()
            )),
            key_derivation
        )
    }
}

impl HostInteractor for TestHostInteractor {}

#[async_trait::async_trait]
impl SignInteractor<TransactionIntent> for TestHostInteractor {
    async fn sign(
        &self,
        request: SignRequest<TransactionIntent>,
    ) -> SignWithFactorsOutcome<TransactionIntentHash> {
        self.transaction_signing.sign(request).await
    }
}

#[async_trait::async_trait]
impl SignInteractor<Subintent> for TestHostInteractor {
    async fn sign(
        &self,
        request: SignRequest<Subintent>,
    ) -> SignWithFactorsOutcome<SubintentHash> {
        self.subintent_signing.sign(request).await
    }
}

#[async_trait::async_trait]
impl KeyDerivationInteractor for TestHostInteractor {
    async fn derive(
        &self,
        request: KeyDerivationRequest
    ) -> Result<KeyDerivationResponse> {
        self.key_derivation.derive(request).await
    }
}