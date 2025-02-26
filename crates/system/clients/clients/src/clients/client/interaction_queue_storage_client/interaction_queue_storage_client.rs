use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InteractionQueueStorageClient {
    driver: Arc<dyn InteractionQueueStorageDriver>,
}

impl InteractionQueueStorageClient {
    pub fn new(driver: Arc<dyn InteractionQueueStorageDriver>) -> Self {
        Self { driver }
    }
}

impl InteractionQueueStorageClient {
    pub async fn save(&self, queue: InteractionQueue) -> Result<()> {
        self.driver.save_queue(queue).await
    }

    pub async fn load(&self) -> Result<Option<InteractionQueue>> {
        self.driver.load_queue().await
    }
}
