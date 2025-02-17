use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A batch of interactions that are to be processed in sequence.
pub struct InteractionQueueBatch {
    /// The identifier of this batch
    pub id: Uuid,

    /// The remaining interaction to process from this batch.
    /// We will be dispatching each of the interactions one by one, leaving a random amount of seconds among each.
    /// The first on the top will always have its status set to `Next`, while the others will have `Queued`.
    /// Once it is time to submit an interaction, it will be removed from this list, switch its status to `InProgress` and place it in the `InteractionQueue.items`
    /// Next one in line will have its status updated to `Next`.
    pub interactions: Vec<InteractionQueueItem>,

    /// The identifiers of all the interactions that were originally part of this batch.
    /// Used to know how many interactions it contained, as well as for allowing further lookup from
    /// the already finished interactions (if ever needed).
    pub original_interactions: Vec<Uuid>,
}

impl InteractionQueueBatch {
    /// Returns the estimated amount of time in seconds before this batch is fully processed.
    fn remaining_seconds(&self) -> usize {
        let average = ((INTERACTION_QUEUE_BATCH_MIN_DELAY
            + INTERACTION_QUEUE_BATCH_MAX_DELAY)
            / 2) as usize;
        self.interactions.len() * average
    }
}
