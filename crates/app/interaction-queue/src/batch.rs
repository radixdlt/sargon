use crate::prelude::*;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A batch of interactions that are to be processed in sequence.
pub struct InteractionQueueBatch {
    /// The identifier of this batch
    pub id: Uuid,

    /// The remaining interactions to process from this batch.
    /// We will be dispatching each of the interactions one by one, leaving a random amount of seconds among each.
    /// The first on the top will always have its status set to `Next`, with a random amount of seconds to wait,
    /// while the others will have `Queued` status.
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
    pub fn estimated_remaining_seconds(&self) -> usize {
        let average = ((INTERACTION_QUEUE_BATCH_MIN_DELAY
            + INTERACTION_QUEUE_BATCH_MAX_DELAY)
            / 2) as usize;
        self.interactions.len() * average
    }

    /// Returns the next interaction, if it is ready to be processed.
    /// This is, if the first interaction in the batch has a `Next` status and its time has completed,
    /// it will be dropped from the list and returned.
    /// Also, the following interaction in line (if any) will have its status updated
    /// to `Next` with a random delay.
    pub(crate) fn get_first_if_ready(
        &mut self,
    ) -> Option<InteractionQueueItem> {
        if let Some(first) = self.interactions.get(0).cloned() {
            if let InteractionQueueItemStatus::Next(next) = first.status {
                let now = Timestamp::now_utc();
                if next <= now {
                    self.drop_first_and_set_up_next();
                    return Some(first.clone());
                }
            }
        }
        None
    }

    /// Removes the first interaction from the batch, and updates the following one to have a
    /// `Next` status with a random delay.
    fn drop_first_and_set_up_next(&mut self) {
        self.interactions.remove(0);
        if let Some(first) = self.interactions.first_mut() {
            // TODO: Generate random number between range
            let delay = (INTERACTION_QUEUE_BATCH_MIN_DELAY
                + INTERACTION_QUEUE_BATCH_MAX_DELAY)
                / 2;
            first.status = InteractionQueueItemStatus::Next(
                Timestamp::now_utc().add(Duration::from_secs(delay)),
            );
        }
    }
}
