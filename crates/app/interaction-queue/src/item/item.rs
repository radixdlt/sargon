use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// An item that is part of the interaction queue.
pub struct InteractionQueueItem {
    /// The identifier of this item.
    pub id: Uuid,

    /// If set, this item belongs to a batch with the corresponding identifier.
    /// This means that when completed, it should trigger the dispatch of next item such batch.
    pub batch_id: Uuid,

    /// The status of this item.
    pub status: InteractionQueueItemStatus,

    /// Whether this interaction was triggered form a mobile browser.
    pub is_from_browser: bool,

    /// The summary of this item, used to represent visually in the hosts.
    pub summary: InteractionQueueItemSummary,

    /// The kind of this item
    pub kind: InteractionQueueItemKind,
}
