use crate::prelude::*;

// ==================
// Create Subintent
// ==================
#[uniffi::export]
impl SargonOS {
    /// Creates a Subintent given its discriminator, manifest and expiration.
    pub async fn create_subintent(
        &self,
        intent_discriminator: IntentDiscriminator,
        subintent_manifest: SubintentManifest,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> Result<Subintent> {
        self.wrapped
            .create_subintent(
                intent_discriminator.into_internal(),
                subintent_manifest.into_internal(),
                expiration.into_internal(),
            )
            .await
            .into_result()
    }
}
