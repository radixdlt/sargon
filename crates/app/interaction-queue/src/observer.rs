use crate::prelude::*;

/// Trait for observing interaction queue updates.
pub trait InteractionsQueueObserver: Send + Sync {
    /// Handles updates to the queued interactions.
    fn handle_update(&self, interactions: Vec<InteractionQueueItem>);
}
