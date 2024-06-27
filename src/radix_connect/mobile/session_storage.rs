use crate::prelude::*;

/// A trait for storing and loading Radix Connect Mobile sessions.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait RadixConnectMobileSessionStorage: Send + Sync {
    /// Saves the session to the storage identified by the session id.
    async fn save_session(
        &self,
        session_id: SessionID,
        encoded_session: BagOfBytes,
    ) -> Result<()>;

    /// Loads the session from the storage identified by the session id.
    async fn load_session(
        &self,
        session_id: SessionID,
    ) -> Result<Option<BagOfBytes>>;
}
