use crate::prelude::*;
use sargon::Event as InternalEvent;
use sargon::HasEventKind;

/// SargonOS event contain information about something of interest that has
/// happened to the SargonOS, most prominently to the Profile. Host device
/// can subscribe to these events by use of `EventBusDriver`.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
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
    ProfileUsedOnOtherDevice(DeviceInfo),
}

#[uniffi::export]
pub fn event_kind(event: &Event) -> EventKind {
    event.into_internal().kind().into()
}
