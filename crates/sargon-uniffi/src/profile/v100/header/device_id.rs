use crate::prelude::*;
use sargon::DeviceID as InternalDeviceID;

/// A stable and globally unique identifier of a device,
/// e.g. an Android phone.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DeviceID {
    value: Uuid,
}

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