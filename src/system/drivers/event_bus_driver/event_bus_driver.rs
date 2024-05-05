use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait EventBusDriver: Send + Sync + std::fmt::Debug {
    async fn handle_event_notification(
        &self,
        event_notification: EventNotification,
    );
}
