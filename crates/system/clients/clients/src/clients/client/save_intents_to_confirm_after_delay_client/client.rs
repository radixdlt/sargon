use manifests::IntentVariantToConfirmAfterDelay;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SaveIntentsToConfirmAfterDelayClient {
    #[allow(dead_code)]
    driver: Arc<dyn UnsafeStorageDriver>,
}
impl SaveIntentsToConfirmAfterDelayClient {
    pub fn new(driver: Arc<dyn UnsafeStorageDriver>) -> Self {
        Self { driver }
    }
}

#[async_trait::async_trait]
impl SaverOfIntentsToConfirmAfterDelay
    for SaveIntentsToConfirmAfterDelayClient
{
    async fn save_intents_to_confirm_after_delay(
        &self,
        _intents: IndexSet<IntentVariantToConfirmAfterDelay>,
    ) -> Result<()> {
        error!("save_intents_to_confirm_after_delay did NOT save anything. Not implemented.");
        Ok(())
    }
}
