use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A struct representing all the interactions that were dispatched, or are waiting to be dispatched.
pub struct InteractionsQueue {
    /// Interactions that were already dispatched.
    /// Their status will always be `InProgress`, `Success` or `Failure`.
    pub items: Vec<InteractionQueueItem>,

    /// Batches of interactions that are waiting to be dispatched.
    pub batches: Vec<InteractionQueueBatch>,
}

impl InteractionsQueue {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            batches: Vec::new(),
        }
    }

    /// Returns every interaction in the queue, sorted by its status.
    pub fn sorted_items(&self) -> Vec<InteractionQueueItem> {
        let mut all_items: Vec<InteractionQueueItem> = self
            .items
            .iter()
            .cloned()
            .chain(
                self.batches
                    .iter()
                    .flat_map(|batch| batch.interactions.clone()),
            )
            .collect();

        all_items.sort_by_key(|item| item.status.clone());
        all_items
    }
}
