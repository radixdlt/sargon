use sargon_core_time_utils::now;

use crate::prelude::*;

/// A notification containing a timestamped and unique `event`, host client
/// can subscribe to these notifications by using the EventBusDriver.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventNotification {
    pub id: Uuid,
    pub event: Event,
    pub timestamp: Timestamp,
}

impl EventNotification {
    pub fn new(event: Event) -> Self {
        Self {
            id: Uuid::new_v4(),
            event,
            timestamp: now(),
        }
    }

    pub fn profile_modified(change: EventProfileModified) -> Self {
        Self::new(Event::profile_modified(change))
    }

    pub fn profile_used_on_other_device(other_device: DeviceInfo) -> Self {
        Self::new(Event::profile_used_on_other_device(other_device))
    }
}

impl HasSampleValues for EventNotification {
    fn sample() -> Self {
        Self {
            id: Uuid::sample(),
            event: Event::sample(),
            timestamp: Timestamp::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            id: Uuid::sample_other(),
            event: Event::sample_other(),
            timestamp: Timestamp::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EventNotification;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
