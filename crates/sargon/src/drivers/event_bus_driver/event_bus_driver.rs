use crate::prelude::*;

/// A driver which received and asynchronously *handles* event notifications
/// emitted by the `SargonOS`. Letting the method be async allows for Rust side
/// to wait for host clients to complete something which might require user
/// attention. E.g. presentation of an alert and await user input.
///
/// Due to limitations in UniFFI and lack of first class citizen support of
/// async sequences (like we have in Swift) we cannot export an accessor of the
/// received events here. Instead implementing types on the FFI side SHOULD
/// create the driver as a singleton object they can reference later and build
/// async streams in that implementing type.
///
/// See Swifts EventBus implementation for more details.
#[uniffi::export(with_foreign)]
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
