use crate::prelude::*;

pub trait InteractionQueueChangeDriver: Send + Sync + Debug {
    /// Handles updates to the queued interactions.
    fn handle_update(&self, interactions: Vec<InteractionQueueItem>);
}
