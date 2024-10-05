use crate::prelude::*;
use sargon::RadixConnectMobileSessionStorage as InternalRadixConnectMobileSessionStorage;
use sargon::SessionID as InternalSessionID;
use sargon::BagOfBytes as InternalBagOfBytes;

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


#[derive(Debug)]
pub struct RadixConnectMobileSessionStorageAdapter {
    pub wrapped: Arc<dyn RadixConnectMobileSessionStorage>,
}

#[async_trait::async_trait]
impl InternalRadixConnectMobileSessionStorage for RadixConnectMobileSessionStorageAdapter {
    async fn save_session(
        &self,
        session_id: InternalSessionID,
        encoded_session: InternalBagOfBytes,
    ) -> InternalResult<()> {
        map_result_to_internal(self.wrapped.save_session(session_id.into(), encoded_session.into()).await)
    }

    async fn load_session(
        &self,
        session_id: InternalSessionID,
    ) -> InternalResult<Option<InternalBagOfBytes>> {
        map_result_to_internal_optional(self.wrapped.load_session(session_id.into()).await)
    }
}
