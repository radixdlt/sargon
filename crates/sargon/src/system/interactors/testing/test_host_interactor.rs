use crate::prelude::*;

pub struct TestHostInteractor {
    transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
    subintent_signing: Arc<dyn SignInteractor<Subintent>>,
    key_derivation: Arc<dyn KeysDerivationInteractors>,
}

impl TestHostInteractor {

    // TODO this should be deleted
    pub fn new_from_bios(bios: Arc<Bios>) -> Self {
        let clients = Clients::new(bios);

        Self::new_with_derivation_interactor(
            Arc::new(TestDerivationInteractors::with_secure_storage(
                Arc::new(clients.secure_storage.clone()),
            ))
        )
    }

    pub fn new(
        transaction_signing: Arc<dyn SignInteractor<TransactionIntent>>,
        subintent_signing: Arc<dyn SignInteractor<Subintent>>,
        key_derivation: Arc<dyn KeysDerivationInteractors>,
    ) -> Self {
        Self {
            transaction_signing,
            subintent_signing,
            key_derivation,
        }
    }

    pub fn new_with_derivation_interactor(
        key_derivation: Arc<dyn KeysDerivationInteractors>,
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
impl KeysDerivationInteractors for TestHostInteractor {
    fn interactor_for(&self, kind: FactorSourceKind) -> KeyDerivationInteractor {
        self.key_derivation.interactor_for(kind)
    }
}