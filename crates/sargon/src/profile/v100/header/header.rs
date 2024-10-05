use crate::prelude::*;

/// The header of a Profile(Snapshot) contains crucial metadata
/// about this Profile, such as which JSON data format it is
/// compatible with and which device was used to create it and
/// a hint about its contents.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
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

impl Header {
    /// Instantiates a new `Header` using the default snapshot version and
    /// the specified values, most prominently a creating device (`DeviceInfo`).
    pub fn with_values(
        id: ProfileID,
        creating_device: DeviceInfo,
        content_hint: ContentHint,
        last_modified: Timestamp,
    ) -> Self {
        Self {
            snapshot_version: ProfileSnapshotVersion::default(),
            id,
            creating_device: creating_device.clone(),
            last_used_on_device: creating_device,
            last_modified,
            content_hint,
        }
    }

    /// Instantiates a new `Header` with creating and last used on `DeviceInfo` with
    /// "Unknown device" as description, and empty content hint
    pub fn new(creating_device: DeviceInfo) -> Self {
        Self::with_values(
            profile_id(),
            creating_device,
            ContentHint::new(),
            now(),
        )
    }
}

impl Identifiable for Header {
    type ID = ProfileID;

    fn id(&self) -> Self::ID {
        self.id
    }
}

impl HasSampleValues for Header {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let device_info = DeviceInfo::sample();
        Header::with_values(
            ProfileID::from_str("12345678-bbbb-cccc-dddd-abcd12345678")
                .unwrap(),
            device_info.clone(),
            ContentHint::with_counters(4, 0, 2),
            device_info.date,
        )
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let device_info = DeviceInfo::sample_other();
        Header::with_values(
            ProfileID::from_str("87654321-bbbb-cccc-dddd-87654321dcba")
                .unwrap(),
            device_info.clone(),
            ContentHint::new(),
            device_info.date,
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

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
    fn json_roundtrip_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "snapshotVersion": 100,
                "id": "12345678-bbbb-cccc-dddd-abcd12345678",
                "creatingDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone (iPhone)"
                },
                "lastUsedOnDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone (iPhone)"
                },
                "lastModified": "2023-09-11T16:05:56.000Z",
                "contentHint": {
                    "numberOfAccountsOnAllNetworksInTotal": 4,
                    "numberOfPersonasOnAllNetworksInTotal": 0,
                    "numberOfNetworks": 2
                }
            }
            "#,
        );
    }

    #[test]
    fn display() {
        let sut = SUT::sample();
        pretty_assertions::assert_eq!(format!("{sut}"), "#12345678-bbbb-cccc-dddd-abcd12345678 v=100, content: #networks: 2, #accounts: 4, #personas: 0");
    }

    #[test]
    fn snapshot_version() {
        let value = ProfileSnapshotVersion::default();
        let sut = SUT::sample();
        assert_eq!(sut.snapshot_version, value)
    }
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Header;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_header_sample());
        assert_eq!(SUT::sample_other(), new_header_sample_other());
    }

    #[test]
    fn header_identifiable() {
        let sut = SUT::sample();
        assert_eq!(&sut.id(), &sut.id);
    }
}
