use crate::prelude::*;

#[async_trait::async_trait]
pub trait InteractionsQueueStorage: Send + Sync {
    async fn save_queue(&self, queue: InteractionsQueue) -> Result<()>;
    async fn load_queue(&self) -> Result<Option<InteractionsQueue>>;
}
