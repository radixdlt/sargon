use crate::prelude::*;
use sargon::Header as InternalHeader;

/// The header of a Profile(Snapshot) contains crucial metadata
/// about this Profile, such as which JSON data format it is
/// compatible with and which device was used to create it and
/// a hint about its contents.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Header {
    /// A versioning number that is increased when breaking
    /// changes is made to ProfileSnapshot JSON data format.
    pub snapshot_version: ProfileSnapshotVersion,

    /// An immutable and unique identifier of a Profile.
    pub id: ProfileID,

    /// The device which was used to create the Profile.
    pub creating_device: DeviceInfo,

    /// The device on which the profile was last used.
    pub last_used_on_device: DeviceInfo,

    /// When the Profile was last modified.
    pub last_modified: Timestamp,

    /// Hint about the contents of the profile, e.g. number of Accounts and Personas.
    pub content_hint: ContentHint,
}

json_data_convertible!(Header);

#[uniffi::export]
pub fn new_header_sample() -> Header {
    InternalHeader::sample().into()
}

#[uniffi::export]
pub fn new_header_sample_other() -> Header {
    InternalHeader::sample_other().into()
}

/// Instantiates a new `Header` with creating and last used on `DeviceInfo` with
/// "Unknown device" as description, and empty content hint
#[uniffi::export]
pub fn new_header_with_creating_device(creating_device: DeviceInfo) -> Header {
    InternalHeader::new(creating_device.into()).into()
}
