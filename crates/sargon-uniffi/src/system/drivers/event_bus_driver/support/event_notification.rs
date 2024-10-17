use crate::prelude::*;
use sargon::EventNotification as InternalEventNotification;

/// A notification containing a timestamped and unique `event`, host client
/// can subscribe to these notifications by using the EventBusDriver.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct EventNotification {
    pub id: Uuid,
    pub event: Event,
    pub timestamp: Timestamp,
}
