use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        message: Message,
        is_wallet_transaction: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        self.perform_transaction_preview_analysis(
            instructions,
            blobs,
            message,
            is_wallet_transaction,
            nonce,
            notary_public_key,
        )
        .await
    }
}
