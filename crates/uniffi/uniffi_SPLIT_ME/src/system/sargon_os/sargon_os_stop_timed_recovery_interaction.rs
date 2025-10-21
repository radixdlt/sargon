use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn make_stop_timed_recovery_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .make_stop_timed_recovery_manifest(address.into())
            .await
            .into_result()
    }
}
