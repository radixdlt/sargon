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
