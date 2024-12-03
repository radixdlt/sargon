use crate::prelude::*;
use sargon::NFCTagDriver as InternalNFCTagDriver;
use sargon::Result as InternalResult;
use sargon::BagOfBytes as InternalBagOfBytes;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NFCTagDriver: Send + Sync + std::fmt::Debug {
    async fn start_session(&self) -> Result<()>;
    async fn end_session(&self);

    async fn send_receive(&self, command: BagOfBytes) -> Result<BagOfBytes>;
}

#[derive(Debug)]
pub struct NFCTagDriverAdapter {
    pub wrapped: Arc<dyn NFCTagDriver>
}


#[async_trait::async_trait]
impl InternalNFCTagDriver for NFCTagDriverAdapter {
    async fn start_session(&self) -> InternalResult<()> {
        self.wrapped.start_session().await.into_internal_result()
    }

    async fn end_session(&self) {
        self.wrapped.end_session().await
    }

    async fn send_receive(&self, command: InternalBagOfBytes) -> InternalResult<InternalBagOfBytes> {
        self.wrapped.send_receive(command.into()).await.into_internal_result()
    }
}