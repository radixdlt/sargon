use crate::prelude::*;

// ==================
// Delete Account
// ==================
#[uniffi::export]
impl SargonOS {
    /// Creates a Subintent given its discriminator, manifest and expiration.
    pub async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .create_delete_account_manifest(account_address.into_internal())
            .await
            .into_result()
    }
}
