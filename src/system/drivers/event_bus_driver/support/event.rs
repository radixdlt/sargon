use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ProfileChange {
    AddedAccount { address: AccountAddress },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Event {
    ProfileChanged { change: ProfileChange },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct EventNotification {
    pub event: Event,
    pub timestamp: Timestamp,
}

impl EventNotification {
    pub fn new(event: Event) -> Self {
        Self {
            event,
            timestamp: now(),
        }
    }
}
