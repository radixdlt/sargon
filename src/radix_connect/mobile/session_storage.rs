use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait SessionStorage: Send + Sync {
    async fn save_session(
        &self,
        session_id: SessionID,
        encoded_session: BagOfBytes,
    ) -> Result<()>;

    async fn load_session(
        &self,
        session_id: SessionID,
    ) -> Result<Option<BagOfBytes>>;
}
