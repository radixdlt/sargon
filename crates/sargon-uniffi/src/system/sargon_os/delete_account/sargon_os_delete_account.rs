use crate::prelude::*;

// ==================
// Delete Account
// ==================
#[uniffi::export]
impl SargonOS {
    /// Creates the `TransactionManifest` for deleting the given `account_address`. If a
    /// `recipient_account_address` is provided, the manifest will also send all the resources from
    /// the deleted account to the recipient one.
    pub async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
        recipient_account_address: Option<AccountAddress>,
    ) -> Result<CreateDeleteAccountManifestResult> {
        self.wrapped
            .create_delete_account_manifest(
                account_address.into_internal(),
                recipient_account_address.map(|x| x.into_internal()),
            )
            .await
            .into_result()
    }
}
