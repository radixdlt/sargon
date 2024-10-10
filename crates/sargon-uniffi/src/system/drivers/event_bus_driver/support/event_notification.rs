use crate::prelude::*;
use sargon::EventNotification as InternalEventNotification;

/// A notification containing a timestamped and unique `event`, host client
/// can subscribe to these notifications by using the EventBusDriver.
#[derive( Clone, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct EventNotification {
    pub id: Uuid,
    pub event: Event,
    pub timestamp: Timestamp,
}

impl From<InternalEventNotification> for EventNotification {
    fn from(value: InternalEventNotification) -> Self {
        Self {
            id: value.id.into(),
            event: value.event.into(),
            timestamp: value.timestamp.into(),
        }
    }
}

impl Into<InternalEventNotification> for EventNotification {
    fn into(self) -> InternalEventNotification {
        InternalEventNotification {
            id: self.id.into(),
            event: self.event.into(),
            timestamp: self.timestamp.into(),
        }
    }
}
