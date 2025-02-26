use crate::prelude::*;

#[derive(Debug)]
pub struct EphemeralInteractionQueueStorageDriver {
    should_fail: bool,
    storage: RwLock<Option<InteractionQueue>>,
}

#[async_trait::async_trait]
impl InteractionQueueStorageDriver for EphemeralInteractionQueueStorageDriver {
    async fn save_queue(&self, queue: InteractionQueue) -> Result<()> {
        if self.should_fail {
            return Err(CommonError::Unknown);
        }
        let mut res = self.storage.write().map_err(|_| CommonError::Unknown)?;
        *res = Some(queue);
        Ok(())
    }

    async fn load_queue(&self) -> Result<Option<InteractionQueue>> {
        if self.should_fail {
            return Err(CommonError::Unknown);
        }
        let res = self.storage.read().map_err(|_| CommonError::Unknown)?;
        Ok(res.clone())
    }
}

impl EphemeralInteractionQueueStorageDriver {
    pub fn saved(&self) -> Option<InteractionQueue> {
        self.storage.read().ok()?.clone()
    }

    fn new(should_fail: bool, stored: Option<InteractionQueue>) -> Self {
        EphemeralInteractionQueueStorageDriver {
            should_fail,
            storage: RwLock::new(stored),
        }
    }

    pub fn empty() -> Arc<Self> {
        Arc::new(EphemeralInteractionQueueStorageDriver::new(false, None))
    }

    pub fn with_queue(queue: InteractionQueue) -> Arc<Self> {
        Arc::new(EphemeralInteractionQueueStorageDriver::new(
            false,
            Some(queue),
        ))
    }

    pub fn failing() -> Arc<Self> {
        Arc::new(EphemeralInteractionQueueStorageDriver::new(true, None))
    }
}
