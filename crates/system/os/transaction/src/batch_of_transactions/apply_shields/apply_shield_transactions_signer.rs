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
    pub fn new<'a>(_os: &'a SargonOS) -> Self {
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
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        error!("Signing transaction intents is not implemented yet");
        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions: vec![],
        })
    }
}
