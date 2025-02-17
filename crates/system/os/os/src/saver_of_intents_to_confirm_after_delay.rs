use manifests::IntentVariantToConfirmAfterDelay;

use crate::prelude::*;

/// A trait used to save intents to confirm after a delay.
/// We save these intents to confirm after a delay so that we can confirm them after a delay.
#[async_trait::async_trait]
pub trait SaverOfIntentsToConfirmAfterDelay: Send + Sync {
    async fn save_intents_to_confirm_after_delay(
        &self,
        intents: IndexSet<IntentVariantToConfirmAfterDelay>,
    ) -> Result<()>;
}
