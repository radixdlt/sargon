use crate::prelude::*;

// Helper methods used in Tests

#[cfg(test)]
impl InteractionsQueue {
    pub fn with_items(items: Vec<InteractionQueueItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            batches: Vec::new(),
        }
    }

    pub fn with_batches(batches: Vec<InteractionQueueBatch>) -> Self {
        Self {
            items: IndexSet::new(),
            batches,
        }
    }

    pub fn with_items_and_batches(
        items: Vec<InteractionQueueItem>,
        batches: Vec<InteractionQueueBatch>,
    ) -> Self {
        Self {
            items: items.into_iter().collect(),
            batches,
        }
    }
}

#[cfg(test)]
impl InteractionQueueItem {
    pub fn with_status(&self, status: InteractionQueueItemStatus) -> Self {
        Self {
            status,
            ..self.clone()
        }
    }
}
