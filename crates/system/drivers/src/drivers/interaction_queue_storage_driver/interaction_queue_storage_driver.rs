use crate::prelude::*;

#[async_trait::async_trait]
pub trait InteractionQueueStorageDriver: Send + Sync + Debug {
    /// Handles updates to the queued interactions.
    async fn save_queue(&self, queue: InteractionQueue) -> Result<()>;
    async fn load_queue(&self) -> Result<Option<InteractionQueue>>;
}
