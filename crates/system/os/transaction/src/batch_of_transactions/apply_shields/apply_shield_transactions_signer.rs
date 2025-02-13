use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: Send + Sync {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;
}

pub struct ApplyShieldTransactionsSignerImpl {}

impl ApplyShieldTransactionsSignerImpl {
    pub fn new(_os: &SargonOS) -> Self {
        warn!(
            "ApplyShieldTransactionsSignerImpl is not implemented yet. Actually might only need the `sign_transactions_interactor` here"
        );
        Self {}
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    async fn sign_transaction_intents(
        &self,
        _payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        #[cfg(not(test))]
        todo!("implement signing transaction intents");
        #[cfg(test)]
        self.fake_sign(_payload_to_sign)
    }
}

#[cfg(test)]
impl SecurityShieldApplicationWithTransactionIntents {
    fn first_transaction_intent(&self) -> TransactionIntent {
        match self {
        SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(sec) => match sec {
            SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents::Account(a) => a.initiate_with_primary_complete_with_confirmation.clone(),
            SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents::Persona(p) => p.initiate_with_primary_complete_with_confirmation.clone(),
        },
        SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(unsec) => match unsec {
            SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent::Account(a) => a.transaction_intent.clone(),
            SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent::Persona(p) => p.transaction_intent.clone()
        }
    }
    }
}

#[cfg(test)]
impl ApplyShieldTransactionsSignerImpl {
    fn fake_sign(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        error!("Signing transaction intents is not implemented yet");
        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions: payload_to_sign
                .applications_with_intents
                .into_iter()
                .map(|i| {
                    let intent = i.first_transaction_intent();

                    NotarizedTransaction::new(
                        SignedIntent {
                            intent,
                            intent_signatures: IntentSignatures::sample(),
                        },
                        NotarySignature::sample(),
                    )
                    .unwrap()
                })
                .collect_vec(),
        })
    }
}
