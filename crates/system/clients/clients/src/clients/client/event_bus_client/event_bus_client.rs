use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EventBusClient {
    driver: Arc<dyn EventBusDriver>,
}

impl EventBusClient {
    pub(crate) fn new(driver: Arc<dyn EventBusDriver>) -> Self {
        Self { driver }
    }
}

impl EventBusClient {
    pub async fn emit(&self, event_notification: EventNotification) {
        self.driver
            .handle_event_notification(event_notification)
            .await
    }
}
