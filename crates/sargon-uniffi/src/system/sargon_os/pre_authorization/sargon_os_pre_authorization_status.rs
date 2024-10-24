use crate::prelude::*;

// ==================
// Poll PreAuthorization Status (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Polls the state of a `PreAuthorization` until we can determine the parent Transaction's status.
    /// This means, we will first poll the subintent status, and once it has been submitted we
    /// will continue polling the
    pub async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<TransactionStatus> {
        let expiration = expiration.map(|exp| exp.into_internal());
        self.wrapped
            .poll_pre_authorization_status(
                intent_hash.into_internal(),
                expiration,
            )
            .await
            .into_result()
    }
}
