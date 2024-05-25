use crate::prelude::*;
use crate::EventKind::{
    AccountUpdated, AccountsAdded, Booted, ProfileImported, ProfileSaved,
};

/// SargonOS event contain information about something of interest that has
/// happened to the SargonOS, most prominently to the Profile, host device
/// can subscribe to these events by use of `EventBusDriver`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Event {
    /// The SargonOS just booted.
    Booted,

    /// Current Gateway changed
    GatewayChangedCurrent { to: Gateway, is_new: bool },

    /// Profile has been saved, typically it has been modified and the new
    /// changed Profile got persisted into secure storage.
    ProfileSaved,

    /// A profile has been imported and has been set to active profile,
    /// and saved into secure storage.
    ProfileImported { id: ProfileID },

    /// The active profile has been modified (might not have been saved yet).
    ProfileModified { change: EventProfileModified },

    /// The Profile was last used on another device, user ought to claim it.
    ProfileLastUsedOnOtherDevice(DeviceInfo),
}

impl Event {
    pub fn profile_modified(change: EventProfileModified) -> Self {
        Self::ProfileModified { change }
    }

    pub fn profile_last_used_on_other_device(device: DeviceInfo) -> Self {
        Self::ProfileLastUsedOnOtherDevice(device)
    }
}

impl HasEventKind for Event {
    fn kind(&self) -> EventKind {
        match self {
            Self::Booted => EventKind::Booted,
            Self::GatewayChangedCurrent { to: _, is_new: _ } => {
                EventKind::GatewayChangedCurrent
            }
            Self::ProfileModified { change } => change.kind(),
            Self::ProfileLastUsedOnOtherDevice(_) => {
                EventKind::ProfileLastUsedOnOtherDevice
            }
            Self::ProfileImported { id: _ } => EventKind::ProfileImported,
            Self::ProfileSaved => EventKind::ProfileSaved,
        }
    }
}

impl HasSampleValues for Event {
    fn sample() -> Self {
        Self::Booted
    }

    fn sample_other() -> Self {
        Self::ProfileModified {
            change: EventProfileModified::sample_other(),
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
        test(
            SUT::ProfileImported {
                id: ProfileID::sample(),
            },
            EventKind::ProfileImported,
        );
        test(SUT::ProfileSaved, EventKind::ProfileSaved);
        test(
            SUT::GatewayChangedCurrent {
                to: Gateway::sample(),
                is_new: false,
            },
            EventKind::GatewayChangedCurrent,
        );
        let change = EventProfileModified::AddedAccount {
            address: AccountAddress::sample(),
        };
        test(
            SUT::ProfileModified {
                change: change.clone(),
            },
            change.kind(),
        );
    }
}
