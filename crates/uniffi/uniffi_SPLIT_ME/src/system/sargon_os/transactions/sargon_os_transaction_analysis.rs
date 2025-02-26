use sargon::OsAnalyseTxPreview;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Extracting the transaction signers.
    /// 2. Executing the transaction preview GW request.
    /// 3. Running the execution summary with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        are_instructions_originating_from_host: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        self.wrapped
            .analyse_transaction_preview(
                instructions,
                blobs.into_internal(),
                are_instructions_originating_from_host,
                nonce.into_internal(),
                notary_public_key.into_internal(),
            )
            .await
            .into_result()
    }
}
