use crate::prelude::*;
use sargon::SargonOsTransactionManifestModify;

#[uniffi::export]
impl SargonOS {
    pub async fn modify_transaction_manifest(
        &self,
        transaction_manifest: TransactionManifest,
        fee_payer_address: AccountAddress,
        fee: Decimal192,
        guarantees: Vec<TransactionGuarantee>,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .modify_transaction_manifest(
                transaction_manifest.into_internal(),
                fee_payer_address.into_internal(),
                fee.into_internal(),
                guarantees.iter().map(|g| g.into_internal()),
            )
            .into_result()
    }
}
