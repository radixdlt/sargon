use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A struct representing all the interactions that were dispatched, or are waiting to be dispatched.
pub struct InteractionsQueue {
    /// Interactions that were already dispatched.
    /// Their status will always be `InProgress`, `Success` or `Failure`.
    pub items: IndexSet<InteractionQueueItem>,

    /// Batches of interactions that are waiting to be dispatched.
    pub batches: Vec<InteractionQueueBatch>,
}

impl InteractionsQueue {
    pub fn new() -> Self {
        Self {
            items: IndexSet::new(),
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

    /// Removes every successful interaction from `items`, and clears `batches` that have no remaining interactions.
    pub fn removing_stale(&mut self) {
        self.items
            .retain(|item| item.status != InteractionQueueItemStatus::Success);
        self.batches.retain(|batch| !batch.interactions.is_empty());
    }

    /// Adds as interaction to the queue.
    pub fn add_interaction(&mut self, interaction: InteractionQueueItem) {
        assert_eq!(interaction.status, InteractionQueueItemStatus::InProgress);
        self.items.insert(interaction);
    }

    /// Replace an interaction in the queue with an updated version of it.
    pub fn replace_interaction(&mut self, interaction: InteractionQueueItem) {
        self.items.replace(interaction);
    }

    /// Adds a new batch to the queue.
    pub fn add_batch(&mut self, batch: InteractionQueueBatch) {
        self.batches.push(batch);
    }
}
