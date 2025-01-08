use crate::prelude::*;
use sargon::DeviceID as InternalDeviceID;

uniffi::custom_newtype!(DeviceID, Uuid);
/// A stable and globally unique identifier of a device,
/// e.g. an Android phone.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct DeviceID(pub Uuid);

delegate_display_debug_into!(DeviceID, InternalDeviceID);
