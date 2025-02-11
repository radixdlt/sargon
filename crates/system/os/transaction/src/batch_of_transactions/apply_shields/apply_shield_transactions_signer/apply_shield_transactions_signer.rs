use serde_json::value::Index;

use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: Send + Sync {
    async fn sign_transaction_intents(
        self,
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
                    profile.factor_sources.clone(),
                    os.sign_transactions_interactor(),
                )
            })
            .map(|signing_manager| Self { signing_manager })
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    async fn sign_transaction_intents(
        self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        let notary_manager = NotaryManager {
            keys_for_intents: payload_to_sign.notary_keys,
        };
        let intent_sets = payload_to_sign.applications_with_intents;
        let signed_sets =
            self.signing_manager.sign_intent_sets(intent_sets).await?;

        let signed_intents = signed_sets
            .into_iter()
            .map(|signed_set| signed_set.get_best_signed_intent())
            .collect_vec();

        let notarized_transactions = notary_manager.notarize(signed_intents)?;

        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions,
        })
    }
}
