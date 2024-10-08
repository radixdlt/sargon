use crate::prelude::*;
use sargon::Header as InternalHeader;

/// The header of a Profile(Snapshot) contains crucial metadata
/// about this Profile, such as which JSON data format it is
/// compatible with and which device was used to create it and
/// a hint about its contents.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
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

impl From<InternalHeader> for Header {
    fn from(value: InternalHeader) -> Self {
        Self {
            snapshot_version: value.snapshot_version.into(),
            id: value.id.into(),
            creating_device: value.creating_device.into(),
            last_used_on_device: value.last_used_on_device.into(),
            last_modified: value.last_modified.into(),
            content_hint: value.content_hint.into(),
        }
    }
}

impl Into<InternalHeader> for Header {
    fn into(self) -> InternalHeader {
        InternalHeader {
            snapshot_version: self.snapshot_version.into(),
            id: self.id.into(),
            creating_device: self.creating_device.into(),
            last_used_on_device: self.last_used_on_device.into(),
            last_modified: self.last_modified.into(),
            content_hint: self.content_hint.into(),
        }
    }
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

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

    #[test]
    fn test_new_with_device() {
        assert_ne!(
            new_header_with_creating_device(DeviceInfo::sample()),
            SUT::sample()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_header_sample(),
                new_header_sample_other(),
                // duplicates should get removed
                new_header_sample(),
                new_header_sample_other(),
            ])
            .len(),
            2
        );
    }
}