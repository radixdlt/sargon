use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: Send + Sync {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;
}

pub struct ApplyShieldTransactionsSignerImpl {
    factor_sources_in_profile: IndexSet<FactorSource>,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
}

impl ApplyShieldTransactionsSignerImpl {
    pub fn new(os: &SargonOS) -> Result<Self> {
        os.profile().map(|profile| Self {
            factor_sources_in_profile: profile.factor_sources(),
            interactor: os.sign_transactions_interactor(),
        })
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    /// Signs and notarized one transaction intent per intent set.
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        let signing_manager = SigningManager::new(
            self.factor_sources_in_profile.clone(),
            self.interactor.clone(),
            payload_to_sign.applications_with_intents,
        );
        // Prepare the notary manager
        let notary_manager = NotaryManager::new(payload_to_sign.notary_keys);

        // Kick off the complex signing process using 4 passes to the signatures collector.
        let outcome = signing_manager.sign_intent_sets().await?;

        // TODO: Implement support for handling of failed transactions, i.e. submit the successful ones even if some failed and do SOMETHING with the failed ones
        let signed_intents = outcome.0;

        // Notarize the signed intents
        let notarized_transactions = notary_manager.notarize(signed_intents)?;

        // Done
        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions,
        })
    }
}
