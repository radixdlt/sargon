use crate::prelude::*;
use sargon::DeviceID as InternalDeviceID;

/// A stable and globally unique identifier of a device,
/// e.g. an Android phone.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,    
     uniffi::Record,
)]
pub struct DeviceID {
    value: Uuid,
}

delegate_display_debug_into!(DeviceID, InternalDeviceID);

impl From<InternalDeviceID> for DeviceID {
    fn from(value: InternalDeviceID) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalDeviceID> for DeviceID {
    fn into(self) -> InternalDeviceID {
        InternalDeviceID(self.value)
    }
}