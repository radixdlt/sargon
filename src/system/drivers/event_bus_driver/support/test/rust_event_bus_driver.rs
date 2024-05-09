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
        (self.spy)(event_notification)
    }
}

impl RustEventBusDriver {
    pub fn new() -> Arc<Self> {
        Self::with_spy(|_| {})
    }
    pub fn with_spy(spy: fn(EventNotification) -> ()) -> Arc<Self> {
        Arc::new(Self { spy })
    }
}
