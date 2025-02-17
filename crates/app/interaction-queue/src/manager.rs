use crate::prelude::*;
use std::sync::Arc;

pub struct InteractionsQueueManager {
    /// The queue of interactions.
    queue: InteractionsQueue,

    /// Observer to handle updates to the interactions queue.
    observer: Arc<dyn InteractionQueueObserver>,
}

impl InteractionsQueueManager {
    pub fn new(observer: Arc<dyn InteractionQueueObserver>) -> Self {
        Self {
            queue: InteractionsQueue::new(),
            observer,
        }
    }
}

// Exported methods (called by Hosts)
impl InteractionsQueueManager {
    /// Method to be called by hosts every time the app is started (or whenever we want)
    /// It will remove the stale data (success transactions).
    pub fn bootstrap(&self) {
        // Load queue from local storage.
        // Removes every successful interaction from `queue.items` and every batch with no remaining interaction from `queue.batches`.
        // Call observer to notify of updated queue
        // Set up timer to call `poll_status` every 3 seconds
    }

    pub fn retry_interaction(&self, interaction_id: Uuid) {
        // Find interaction in `queue.items` and retry it
    }

    pub fn dismiss_interaction(&self, interaction_id: Uuid) {
        // Remove interaction from `queue.items`
    }

    pub fn cancel_interaction(&self, interaction_id: Uuid, batch_id: Uuid) {
        // Find batch in `queue.batches` and remove the interaction with given id from its list.
    }
}

// Internal methods (called by other places inside Sargon)
impl InteractionsQueueManager {
    pub fn add_interaction(&self, item: InteractionQueueItem) {
        // Adds interaction to `queue.items` and process it
        // Call observer to notify of updated queue
    }

    pub fn add_batch(&self, batch: InteractionQueueBatch) {
        // Adds batch to `queue.batches` and process the first interaction from its list.
        // Call observer to notify of updated queue
    }
}

// Private methods
impl InteractionsQueueManager {
    fn poll_status(&self) {
        // Poll the status of every interaction that is `InProgress`
        // For each of them that has finished and has a `batchId` set, handle the next interaction of its batch.
        // Call observer to notify of updated queue
    }

    fn process_interaction(&self, item: InteractionQueueItem) {
        // Set interaction status to `InProgress`
        // If it is a Transaction, submit it to network.
    }
}
