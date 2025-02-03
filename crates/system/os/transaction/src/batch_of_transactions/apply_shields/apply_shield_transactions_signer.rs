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
        todo!(
            "Actually might only need the `sign_transactions_interactor` here"
        )
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        todo!()
    }
}
