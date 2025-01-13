use crate::prelude::*;
use std::sync::RwLock;

#[derive(Debug)]
pub struct RustEventBusDriver {
    recorded: RwLock<Vec<EventNotification>>,
    spy: fn(EventNotification) -> (),
}

#[async_trait::async_trait]
impl EventBusDriver for RustEventBusDriver {
    async fn handle_event_notification(
        &self,
        event_notification: EventNotification,
    ) {
        self.recorded
            .try_write()
            .unwrap()
            .push(event_notification.clone());
        (self.spy)(event_notification)
    }
}

impl RustEventBusDriver {
    pub fn recorded(&self) -> Vec<EventNotification> {
        self.recorded.try_read().unwrap().clone()
    }
    pub fn new() -> Arc<Self> {
        Self::with_spy(|_| {})
    }
    pub fn with_spy(spy: fn(EventNotification) -> ()) -> Arc<Self> {
        Arc::new(Self {
            spy,
            recorded: RwLock::new(Vec::new()),
        })
    }

    pub fn clear_recorded(&self) {
        self.recorded.try_write().unwrap().clear();
    }
}
