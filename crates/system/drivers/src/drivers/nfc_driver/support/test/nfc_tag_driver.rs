use crate::prelude::*;

#[derive(Debug)]
pub struct RustNFCTagDriver;

impl RustNFCTagDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustNFCTagDriver)
    }
}

#[async_trait::async_trait]
impl NFCTagDriver for RustNFCTagDriver {
    async fn start_session(&self, _purpose: NFCTagDriverPurpose) -> Result<()> {
        todo!()
    }

    async fn end_session(&self, _with_failure: Option<CommonError>) {
        todo!()
    }

    async fn send_receive(&self, _command: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    async fn send_receive_command_chain(
        &self,
        _command: Vec<BagOfBytes>,
    ) -> Result<BagOfBytes> {
        todo!()
    }

    async fn set_progress(&self, _progress: u8) {
        todo!()
    }
}
