use crate::prelude::*;

#[derive(Debug)]
pub struct RustInteractionQueueChangeDriver {
    recorded: RwLock<Vec<Vec<InteractionQueueItem>>>,
    spy: fn(Vec<InteractionQueueItem>) -> (),
}

impl InteractionQueueChangeDriver for RustInteractionQueueChangeDriver {
    fn handle_update(&self, interactions: Vec<InteractionQueueItem>) {
        self.recorded
            .try_write()
            .unwrap()
            .push(interactions.clone());
        (self.spy)(interactions)
    }
}

impl RustInteractionQueueChangeDriver {
    pub fn recorded(&self) -> Vec<Vec<InteractionQueueItem>> {
        self.recorded.try_read().unwrap().clone()
    }

    pub fn new() -> Arc<Self> {
        Self::with_spy(|_| {})
    }

    pub fn with_spy(spy: fn(Vec<InteractionQueueItem>) -> ()) -> Arc<Self> {
        Arc::new(Self {
            spy,
            recorded: RwLock::new(Vec::new()),
        })
    }
}
