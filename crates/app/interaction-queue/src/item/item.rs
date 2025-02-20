use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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
        id: Uuid,
        is_from_browser: bool,
        kind: InteractionQueueItemKind,
    ) -> Self {
        Self::new(
            id,
            InteractionQueueItemStatus::InProgress,
            is_from_browser,
            InteractionQueueItemSummary::new(),
            kind,
        )
    }
}
