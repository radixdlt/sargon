use crate::prelude::*;

#[async_trait::async_trait]
pub trait NFCTagDriver: Send + Sync + std::fmt::Debug {
    async fn start_session(&self) -> Result<()>;
    async fn end_session(&self);

    async fn send_receive(&self, command: BagOfBytes) -> Result<BagOfBytes>;
    async fn send_receive_command_chain(&self, commands: Vec<BagOfBytes>) -> Result<BagOfBytes>;
}