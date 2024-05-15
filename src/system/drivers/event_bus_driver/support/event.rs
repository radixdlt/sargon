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

    /// The Profile was last used on another device, user ought to claim it.
    ProfileLastUsedOnOtherDevice(DeviceInfo),

    /// The profile has change (might not have been saved yet).
    ProfileChanged { change: EventProfileChange },
}

impl Event {
    pub fn profile_changed(change: EventProfileChange) -> Self {
        Self::ProfileChanged { change }
    }

    pub fn profile_last_used_on_other_device(device: DeviceInfo) -> Self {
        Self::ProfileLastUsedOnOtherDevice(device)
    }
}

impl HasEventKind for Event {
    fn kind(&self) -> EventKind {
        match self {
            Self::Booted => EventKind::Booted,
            Self::ProfileSaved => EventKind::ProfileSaved,
            Self::ProfileLastUsedOnOtherDevice(_) => {
                EventKind::ProfileLastUsedOnOtherDevice
            }
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

    #[test]
    fn last_used_on_other_device() {
        let device = DeviceInfo::sample();
        let sut = SUT::profile_last_used_on_other_device(device.clone());
        assert_eq!(sut, SUT::ProfileLastUsedOnOtherDevice(device))
    }

    #[test]
    fn test_kind() {
        let test = |s: SUT, exp: EventKind| {
            assert_eq!(s.kind(), exp);
        };
        test(SUT::ProfileSaved, EventKind::ProfileSaved);
        let change = EventProfileChange::AddedAccount {
            address: AccountAddress::sample(),
        };
        test(
            SUT::ProfileChanged {
                change: change.clone(),
            },
            change.kind(),
        );
    }
}
