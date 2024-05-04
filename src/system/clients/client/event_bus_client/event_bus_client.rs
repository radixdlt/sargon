use crate::prelude::*;

#[derive(Debug)]
pub struct EventBusClient {
    driver: Arc<dyn EventBusDriver>,
}

impl EventBusClient {
    pub(crate) fn new(driver: Arc<dyn EventBusDriver>) -> Self {
        Self { driver }
    }
}

impl EventBusClient {
    pub async fn emit(&self, event: Event) {
        self.driver.handle_event(event).await
    }
}
