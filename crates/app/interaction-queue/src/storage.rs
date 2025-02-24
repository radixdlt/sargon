use crate::prelude::*;

#[async_trait::async_trait]
pub trait InteractionQueueStorage: Send + Sync {
    async fn save_queue(&self, queue: InteractionQueue) -> Result<()>;
    async fn load_queue(&self) -> Result<Option<InteractionQueue>>;
}
