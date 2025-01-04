use crate::prelude::*;

// ==================
// Poll PreAuthorization Status (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// Polls the status of a `SubintentHash` until it is either `Success` or `Expired`.
    pub async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration_timestamp: Instant,
    ) -> Result<PreAuthorizationStatus> {
        self.wrapped
            .poll_pre_authorization_status(
                intent_hash.into_internal(),
                expiration_timestamp.into_internal(),
            )
            .await
            .into_result()
    }
}
