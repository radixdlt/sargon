use crate::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
/// An item that is part of the interaction queue.
pub struct InteractionQueueItem {
    /// The identifier of this item.
    pub id: Uuid,

    /// The status of this item.
    pub status: InteractionQueueItemStatus,

    /// Whether this interaction was triggered form a mobile browser.
    pub is_from_browser: bool,

    /// The summary of this item, used to represent visually in the hosts.
    pub summary: InteractionQueueItemSummary,

    /// The kind of this item
    pub kind: InteractionQueueItemKind,
}

impl InteractionQueueItem {
    pub fn new(
        id: Uuid,
        status: InteractionQueueItemStatus,
        is_from_browser: bool,
        summary: InteractionQueueItemSummary,
        kind: InteractionQueueItemKind,
    ) -> Self {
        Self {
            id,
            status,
            is_from_browser,
            summary,
            kind,
        }
    }

    pub fn new_in_progress(
        is_from_browser: bool,
        kind: InteractionQueueItemKind,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            InteractionQueueItemStatus::InProgress,
            is_from_browser,
            InteractionQueueItemSummary::new(),
            kind,
        )
    }
}

impl PartialEq for InteractionQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for InteractionQueueItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
