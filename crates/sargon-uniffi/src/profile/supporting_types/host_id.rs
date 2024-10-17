use crate::prelude::*;
use sargon::HostId as InternalHostId;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct HostId {
    /// A best effort stable and unique identifier of this
    /// host's device.
    pub id: DeviceID,

    /// The date this id of the device was generated, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub generated_at: Timestamp,
}

json_data_convertible!(HostId);

#[uniffi::export]
pub fn new_host_id_sample() -> HostId {
    InternalHostId::sample().into()
}

#[uniffi::export]
pub fn new_host_id_sample_other() -> HostId {
    InternalHostId::sample_other().into()
}
