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
    async fn start_session(&self) -> Result<()> {
        todo!()
    }

    async fn end_session(&self) {
        todo!()
    }

    async fn send_receive(&self, command: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    async fn send_receive_command_chain(&self, command: Vec<BagOfBytes>) -> Result<BagOfBytes> {
        todo!()
    }
}