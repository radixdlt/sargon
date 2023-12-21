use std::{
    cell::{Cell, RefCell},
    fmt::Display,
};

#[cfg(any(test, feature = "placeholder"))]
use std::str::FromStr;

use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use wallet_kit_common::{id, now};

use super::{
    content_hint::ContentHint, device_info::DeviceInfo,
    profilesnapshot_version::ProfileSnapshotVersion,
};

/// The header of a Profile(Snapshot) contains crucial metadata
/// about this Profile, such as which JSON data format it is
/// compatible with and which device was used to create it and
/// a hint about its contents.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// A versioning number that is increased when breaking
    /// changes is made to ProfileSnapshot JSON data format.
    snapshot_version: ProfileSnapshotVersion,

    /// An immutable and unique identifier of a Profile.
    id: Uuid,

    /// The device which was used to create the Profile.
    creating_device: DeviceInfo,

    /// The device on which the profile was last used.
    last_used_on_device: RefCell<DeviceInfo>,

    /// When the Profile was last modified.
    last_modified: Cell<Timestamp>,

    /// Hint about the contents of the profile, e.g. number of Accounts and Personas.
    content_hint: RefCell<ContentHint>, // `RefCell` needed because `ContentHint` does not impl `Copy`, which it cant because it contains `Cell`s, and `Cell` itself does not impl `Copy`.
}

impl Header {
    /// Instantiates a new `Header` using the default snapshot version and
    /// the specified values, most prominently a creating device (`DeviceInfo`).
    pub fn with_values(
        id: Uuid,
        creating_device: DeviceInfo,
        content_hint: ContentHint,
        last_modified: Timestamp,
    ) -> Self {
        Self {
            snapshot_version: ProfileSnapshotVersion::default(),
            id,
            creating_device: creating_device.clone(),
            last_used_on_device: RefCell::new(creating_device),
            last_modified: Cell::new(last_modified),
            content_hint: RefCell::new(content_hint),
        }
    }

    /// Instantiates a new `Header` with creating and last used on `DeviceInfo` with
    /// "Unknown device" as description, and empty content hint
    pub fn new(creating_device: DeviceInfo) -> Self {
        Self::with_values(id(), creating_device, ContentHint::new(), now())
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(DeviceInfo::default())
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{} v={}, content: {}",
            self.id,
            self.snapshot_version,
            self.content_hint()
        )
    }
}

// Getters
impl Header {
    /// A versioning number that is increased when breaking
    /// changes is made to ProfileSnapshot JSON data format.
    pub fn snapshot_version(&self) -> ProfileSnapshotVersion {
        self.snapshot_version
    }

    /// An immutable and unique identifier of a Profile.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// The device which was used to create the Profile.
    pub fn creating_device(&self) -> DeviceInfo {
        self.creating_device.clone()
    }

    /// Hint about the contents of the profile, e.g. number of Accounts and Personas.
    pub fn content_hint(&self) -> ContentHint {
        self.content_hint.borrow().clone()
    }

    /// The device on which the profile was last used.
    pub fn last_used_on_device(&self) -> DeviceInfo {
        self.last_used_on_device.borrow().clone()
    }

    /// When the Profile was last modified.
    pub fn last_modified(&self) -> Timestamp {
        self.last_modified.get().clone()
    }
}

// Setters
impl Header {
    /// Updates the `last_modified` field.
    pub fn updated(&self) {
        self.last_modified.set(now());
    }

    /// Sets the content hint WITHOUT updating `last_modified`, you SHOULD not
    /// use this, use `update_content_hint`, this is primarily meant for testing.
    pub fn set_content_hint(&self, new: ContentHint) {
        *self.content_hint.borrow_mut() = new;
    }

    /// Sets the `content_hint` and updates the `last_modified` field.
    pub fn update_content_hint(&self, new: ContentHint) {
        self.set_content_hint(new);
        self.updated()
    }

    /// Sets the `last_used_on_device` and updates the `last_modified` field.
    pub fn update_last_used_on_device(&self, new: DeviceInfo) {
        *self.last_used_on_device.borrow_mut() = new;
        self.updated()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Header {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        //let date =  NaiveDateTime::parse_from_str("2023-09-11T16:05:56.000Z", "%Y-%m-%dT%H:%M:%S").unwrap();
        let date = Timestamp::parse("2023-09-11T16:05:56Z").unwrap();
        let device = DeviceInfo::with_values(
            Uuid::from_str("66f07ca2-a9d9-49e5-8152-77aca3d1dd74").unwrap(),
            date.clone(),
            "iPhone".to_string(),
        );
        Header::with_values(
            Uuid::from_str("12345678-bbbb-cccc-dddd-abcd12345678").unwrap(),
            device,
            ContentHint::with_counters(4, 0, 2),
            date,
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_other() -> Self {
        //let date =  NaiveDateTime::parse_from_str("2023-09-11T16:05:56.000Z", "%Y-%m-%dT%H:%M:%S").unwrap();
        let date = Timestamp::parse("2023-12-20T16:05:56Z").unwrap();
        let device = DeviceInfo::with_values(
            Uuid::from_str("aabbccdd-a9d9-49e5-8152-beefbeefbeef").unwrap(),
            date.clone(),
            "iPhone".to_string(),
        );
        Header::with_values(
            Uuid::from_str("87654321-bbbb-cccc-dddd-87654321dcba").unwrap(),
            device,
            ContentHint::new(),
            date,
        )
    }
}

#[cfg(test)]
pub mod tests {

    use std::str::FromStr;

    use crate::v100::header::{
        content_hint::ContentHint, device_info::DeviceInfo,
        profilesnapshot_version::ProfileSnapshotVersion,
    };
    use iso8601_timestamp::Timestamp;
    use uuid::Uuid;
    use wallet_kit_common::{assert_eq_after_json_roundtrip, id};

    use super::Header;

    #[test]
    fn inequality() {
        assert_ne!(Header::placeholder(), Header::placeholder_other());
    }

    #[test]
    fn equality() {
        assert_eq!(Header::placeholder(), Header::placeholder());
        assert_eq!(Header::placeholder_other(), Header::placeholder_other());
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let sut = Header::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "snapshotVersion": 100,
                "id": "12345678-bbbb-cccc-dddd-abcd12345678",
                "creatingDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone"
                },
                "lastUsedOnDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone"
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
    fn updated() {
        let sut = Header::default();
        let d0 = sut.last_modified();
        for _ in 0..10 {
            // rust is too fast, if we run it once, unit tests fails.
            sut.updated();
        }
        let d1 = sut.last_modified();
        assert!(d1 > d0);
    }

    #[test]
    fn update_content_hint() {
        let sut = Header::default();
        let d0 = sut.last_modified();
        let content_hint_0 = sut.content_hint();
        let end = 10;
        for n in 1..end {
            // rust is too fast, if we run it once, unit tests fails.
            sut.update_content_hint(ContentHint::all(n));
        }
        let content_hint_n = sut.content_hint();
        assert_ne!(content_hint_n, content_hint_0);
        let d1 = sut.last_modified();
        assert!(d1 > d0);
    }

    #[test]
    fn update_last_used_on_device() {
        let sut = Header::default();
        let d0 = sut.last_modified();
        let device_0 = sut.last_used_on_device();
        let end = 10;
        for n in 1..end {
            // rust is too fast, if we run it once, unit tests fails.
            sut.update_last_used_on_device(DeviceInfo::with_description(n.to_string().as_str()));
        }
        let device_n = sut.last_used_on_device();
        assert_ne!(device_n, device_0);
        assert!(sut.last_modified() > d0);
    }

    #[test]
    fn last_updated() {
        let sut = Header::default();
        assert_eq!(sut.last_modified.get(), sut.last_modified());
    }

    #[test]
    fn display() {
        let date = Timestamp::parse("2023-09-11T16:05:56Z").unwrap();
        let device = DeviceInfo::with_values(
            Uuid::from_str("66f07ca2-a9d9-49e5-8152-77aca3d1dd74").unwrap(),
            date.clone(),
            "iPhone".to_string(),
        );
        let sut = Header::with_values(
            Uuid::from_str("12345678-bbbb-cccc-dddd-abcd12345678").unwrap(),
            device,
            ContentHint::new(),
            date,
        );
        assert_eq!(format!("{sut}"), "#12345678-bbbb-cccc-dddd-abcd12345678 v=100, content: #networks: 0, #accounts: 0, #personas: 0");
    }

    #[test]
    fn creating_device() {
        let value = DeviceInfo::new_iphone();
        let sut = Header {
            creating_device: value.clone(),
            ..Default::default()
        };
        assert_eq!(sut.creating_device(), value)
    }

    #[test]
    fn get_id() {
        let value = id();
        let sut = Header {
            id: value.clone(),
            ..Default::default()
        };
        assert_eq!(sut.id(), value)
    }

    #[test]
    fn snapshot_version() {
        let value = ProfileSnapshotVersion::default();
        let sut = Header {
            snapshot_version: value.clone(),
            ..Default::default()
        };
        assert_eq!(sut.snapshot_version(), value)
    }
}
