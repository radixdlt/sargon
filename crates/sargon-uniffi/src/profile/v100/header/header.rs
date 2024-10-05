use crate::prelude::*;

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
    derive_more::Display,
    uniffi::Record,
)]
#[display("#{} v={}, content: {}", id, snapshot_version, content_hint)]
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

impl Identifiable for Header {
    type ID = ProfileID;

    fn id(&self) -> Self::ID {
        self.id
    }
}

json_data_convertible!(Header);

#[uniffi::export]
pub fn new_header_sample() -> Header {
    Header::sample()
}

#[uniffi::export]
pub fn new_header_sample_other() -> Header {
    Header::sample_other()
}

/// Instantiates a new `Header` with creating and last used on `DeviceInfo` with
/// "Unknown device" as description, and empty content hint
#[uniffi::export]
pub fn new_header_with_creating_device(creating_device: DeviceInfo) -> Header {
    Header::new(creating_device)
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