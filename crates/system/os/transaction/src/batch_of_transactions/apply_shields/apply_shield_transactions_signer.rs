use crate::prelude::*;


#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: shaku::Interface {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;
}



#[derive(Provider)]
#[shaku(interface = ApplyShieldTransactionsSigner)]
pub struct ApplyShieldTransactionsSignerImpl {}
#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        todo!()
    }
}

