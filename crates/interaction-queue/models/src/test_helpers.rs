use crate::prelude::*;
use std::time::Duration;

#[cfg(debug_assertions)]
impl InteractionQueueItem {
    pub fn sample_queued() -> Self {
        Self::sample_transaction(InteractionQueueItemStatus::Queued)
    }

    pub fn sample_next() -> Self {
        Self::sample_next_in(500)
    }

    pub fn sample_next_in(milliseconds: u64) -> Self {
        let timestamp =
            Timestamp::now_utc().add(Duration::from_millis(milliseconds));
        Self::sample_transaction(InteractionQueueItemStatus::Next(timestamp))
    }

    pub fn sample_in_progress() -> Self {
        Self::sample_transaction(InteractionQueueItemStatus::InProgress)
    }

    pub fn sample_success() -> Self {
        Self::sample_transaction(InteractionQueueItemStatus::Success)
    }

    pub fn sample_failed() -> Self {
        Self::sample_transaction(InteractionQueueItemStatus::Failure(
            InteractionQueueItemFailureStatus::Failed,
        ))
    }

    pub fn sample_transaction(status: InteractionQueueItemStatus) -> Self {
        Self::new(
            Uuid::new_v4(),
            status,
            false,
            InteractionQueueItemSummary::new(),
            InteractionQueueItemKind::sample(),
        )
    }

    pub fn sample_pre_authorization(
        status: InteractionQueueItemStatus,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            status,
            false,
            InteractionQueueItemSummary::new(),
            InteractionQueueItemKind::sample_other(),
        )
    }

    pub fn with_status(&self, status: InteractionQueueItemStatus) -> Self {
        Self {
            status,
            ..self.clone()
        }
    }
}

#[cfg(debug_assertions)]
impl InteractionQueue {
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
        items: impl IntoIterator<Item = InteractionQueueItem>,
        batches: impl IntoIterator<Item = InteractionQueueBatch>,
    ) -> Self {
        Self {
            items: items.into_iter().collect(),
            batches: Vec::from_iter(batches),
        }
    }
}

#[cfg(debug_assertions)]
impl InteractionQueueBatch {
    pub fn empty() -> Self {
        Self::new(Uuid::new_v4(), Vec::new(), Vec::new())
    }

    pub fn with_items(
        items: impl IntoIterator<Item = InteractionQueueItem> + Clone,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            items.clone(),
            items.into_iter().map(|i| i.id),
        )
    }

    pub fn dropping_first(&self) -> Self {
        let mut interactions = self.interactions.clone();
        interactions.remove(0);
        if let Some(first) = interactions.get_mut(0) {
            // Note: this will need to be updated when the duration is actually random
            first.status = InteractionQueueItemStatus::Next(
                Timestamp::now_utc().add(Duration::from_secs(35)),
            );
        }
        Self::new(self.id, interactions, self.original_interactions.clone())
    }
}
