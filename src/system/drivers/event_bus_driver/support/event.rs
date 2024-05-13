use crate::prelude::*;

/// SargonOS event contain information about something of interest that has
/// happened to the SargonOS, most prominently to the Profile, host device
/// can subscribe to these events by use of `EventBusDriver`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Event {
    /// The SargonOS just booted.
    Booted,

    /// Profile has been saved, typically it has been modified and the new
    /// changed Profile got persisted into secure storage.
    ProfileSaved,

    /// The profile has change (might not have been saved yet).
    ProfileChanged { change: EventProfileChange },
}

impl Event {
    pub fn profile_changed(change: EventProfileChange) -> Self {
        Self::ProfileChanged { change }
    }
}

impl HasEventKind for Event {
    fn kind(&self) -> EventKind {
        match self {
            Self::Booted => EventKind::Booted,
            Self::ProfileSaved => EventKind::ProfileSaved,
            Self::ProfileChanged { change } => change.kind(),
        }
    }
}

impl HasSampleValues for Event {
    fn sample() -> Self {
        Self::Booted
    }

    fn sample_other() -> Self {
        Self::ProfileChanged {
            change: EventProfileChange::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Event;

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
