use crate::prelude::*;

#[derive(Debug)]
pub struct RustEventBusDriver {
    spy: fn(EventNotification) -> (),
}

#[async_trait::async_trait]
impl EventBusDriver for RustEventBusDriver {
    async fn handle_event_notification(
        &self,
        event_notification: EventNotification,
    ) {
        self.spy(event_notification)
    }
}

impl RustEventBusDriver {
    pub fn new(spy: fn(EventNotification) -> ()) -> Arc<Self> {
        Arc::new(Self { spy })
    }
}
