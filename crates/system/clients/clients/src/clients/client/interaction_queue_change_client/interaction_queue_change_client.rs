use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InteractionQueueChangeClient {
    driver: Arc<dyn InteractionQueueChangeDriver>,
}

impl InteractionQueueChangeClient {
    pub fn new(driver: Arc<dyn InteractionQueueChangeDriver>) -> Self {
        Self { driver }
    }
}

impl InteractionQueueChangeClient {
    pub fn emit(&self, interactions: Vec<InteractionQueueItem>) {
        self.driver.handle_update(interactions)
    }
}
