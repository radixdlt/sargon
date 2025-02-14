use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: Send + Sync {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;
}

pub struct ApplyShieldTransactionsSignerImpl {
    signing_manager: SigningManager,
}

impl ApplyShieldTransactionsSignerImpl {
    pub fn new(os: &SargonOS) -> Result<Self> {
        os.profile()
            .map(|profile| {
                SigningManager::new(
                    profile.factor_sources(),
                    os.sign_transactions_interactor(),
                )
            })
            .map(|signing_manager| Self { signing_manager })
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    /// Signs and notarized one transaction intent per intent set.
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        // Prepare the notary manager
        let notary_manager = NotaryManager::new(payload_to_sign.notary_keys);

        // Kick off the complex signing process using 4 passes to the signatures collector.
        let outcome = self
            .signing_manager
            .sign_intent_sets(payload_to_sign.applications_with_intents)
            .await?;

        // TODO: Implement support for handling of failed transactions, i.e. submit the successful ones even if some failed and do SOMETHING with the failed ones
        let signed_sets = outcome.validate_all_intent_sets_signed()?;

        // We are not going to submit multiple manifest variants for each "manifest set",
        // we only want the "best one" for each set.
        let signed_intents = signed_sets
            .into_iter()
            .map(|signed_set| signed_set.get_best_signed_intent())
            .collect::<Result<Vec<SignedIntent>>>()?;

        // Notarize the signed intents
        let notarized_transactions = notary_manager.notarize(signed_intents)?;

        // Done
        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions,
        })
    }
}
