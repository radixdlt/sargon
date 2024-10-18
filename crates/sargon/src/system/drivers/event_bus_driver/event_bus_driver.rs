use crate::prelude::*;

/// A driver which received and asynchronously *handles* event notifications
/// emitted by the `SargonOS`. Letting the method be async allows for Rust side
/// to wait for host clients to complete something which might require user
/// attention. E.g. presentation of an alert and await user input.
#[async_trait::async_trait]
pub trait EventBusDriver: Send + Sync + std::fmt::Debug {
    /// Asynchronously *handles* event notifications
    /// emitted by the `SargonOS`. Letting the method be async allows for Rust side
    /// to wait for host clients to complete something which might require user
    /// attention. E.g. presentation of an alert and await user input.
    async fn handle_event_notification(
        &self,
        event_notification: EventNotification,
    );
}
