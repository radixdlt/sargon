use sargon::OsAnalysePreAuthPreview;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Creating the SubintentManifest.
    /// 2. Validating if the manifest is open or enclosed.
    /// 3. If open, the manifest with its summary is returned.
    /// 4. If enclosed, it extracts the transaction signers and then transaction preview GW request is executed.
    /// 3. The execution summary is created with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<PreAuthToReview> {
        self.wrapped
            .analyse_pre_auth_preview(
                instructions,
                blobs.into_internal(),
                nonce.into_internal(),
                notary_public_key.into_internal(),
            )
            .await
            .into_result()
    }
}
