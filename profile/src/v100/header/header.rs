use std::{
    cell::{Cell, RefCell},
    fmt::Display,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::factory::{id, now};

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
    pub snapshot_version: ProfileSnapshotVersion,

    /// An immutable and unique identifier of a Profile.
    pub id: Uuid,

    /// The device which was used to create the Profile.
    pub creating_device: DeviceInfo,

    /// The device on which the profile was last used.
    last_used_on_device: RefCell<DeviceInfo>, // `RefCell` needed, because `Cell` requires `Copy` and `DeviceInfo` contains `String` (which does not impl `Copy`). We could potentially use `fstr` from `fixedstr` crate for `description` inside `DeviceInfo`? and thus use `Cell` here?

    /// When the Profile was last modified.
    last_modified: Cell<NaiveDateTime>,

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
        last_modified: NaiveDateTime,
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
            self.get_content_hint()
        )
    }
}

// Getters
impl Header {
    /// Hint about the contents of the profile, e.g. number of Accounts and Personas.
    pub fn get_content_hint(&self) -> ContentHint {
        self.content_hint.borrow().clone()
    }

    pub fn get_last_used_on_device(&self) -> DeviceInfo {
        self.last_used_on_device.borrow().clone()
    }

    pub fn get_last_modified(&self) -> NaiveDateTime {
        self.last_modified.get().clone()
    }
}

// Setters
impl Header {
    /// Updates the `last_modified` field.
    pub fn updated(&self) {
        self.last_modified.set(now());
    }

    /// Sets the `content_hint` and updates the `last_modified` field.
    pub fn update_content_hint(&self, new: ContentHint) {
        *self.content_hint.borrow_mut() = new;
        self.updated()
    }

    /// Sets the `last_used_on_device` and updates the `last_modified` field.
    pub fn update_last_used_on_device(&self, new: DeviceInfo) {
        *self.last_used_on_device.borrow_mut() = new;
        self.updated()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::v100::header::{content_hint::ContentHint, device_info::DeviceInfo};
    use chrono::NaiveDateTime;
    use uuid::uuid;
    use wallet_kit_test_utils::json::assert_eq_after_json_roundtrip;

    use super::Header;

    #[test]
    fn json_roundtrip() {
        let date =
            NaiveDateTime::parse_from_str("2023-09-11T16:05:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        let device = DeviceInfo::with_values(
            uuid!("66f07ca2-a9d9-49e5-8152-77aca3d1dd74"),
            date.clone(),
            "iPhone".to_string(),
        );
        let model = Header::with_values(
            uuid!("12345678-bbbb-cccc-dddd-abcd12345678"),
            device,
            ContentHint::new(),
            date,
        );
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "snapshotVersion": 100,
                "id": "12345678-bbbb-cccc-dddd-abcd12345678",
                "creatingDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56",
                    "description": "iPhone"
                },
                "lastUsedOnDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56",
                    "description": "iPhone"
                },
                "lastModified": "2023-09-11T16:05:56",
                "contentHint": {
                    "numberOfAccountsOnAllNetworksInTotal": 0,
                    "numberOfPersonasOnAllNetworksInTotal": 0,
                    "numberOfNetworks": 0
                }
            }
            "#,
        );
    }

    #[test]
    fn updated() {
        let sut = Header::default();
        let d0 = sut.last_modified.get();
        for _ in 0..10 {
            // rust is too fast, if we run it once, unit tests fails.
            sut.updated();
        }
        let d1 = sut.last_modified.get();
        assert!(d1 > d0);
    }

    #[test]
    fn update_content_hint() {
        let sut = Header::default();
        let d0 = sut.last_modified.get();
        let content_hint_0 = sut.get_content_hint();
        let end = 10;
        for n in 1..end {
            // rust is too fast, if we run it once, unit tests fails.
            sut.update_content_hint(ContentHint::all(n));
        }
        let content_hint_n = sut.get_content_hint();
        assert_ne!(content_hint_n, content_hint_0);
        let d1 = sut.last_modified.get();
        assert!(d1 > d0);
    }

    #[test]
    fn update_last_used_on_device() {
        let sut = Header::default();
        let d0: NaiveDateTime = sut.last_modified.get();
        let device_0 = sut.get_last_used_on_device();
        let end = 10;
        for n in 1..end {
            // rust is too fast, if we run it once, unit tests fails.
            sut.update_last_used_on_device(DeviceInfo::with_description(n.to_string().as_str()));
        }
        let device_n = sut.get_last_used_on_device();
        assert_ne!(device_n, device_0);
        let d1 = sut.last_modified.get();
        assert!(d1 > d0);
    }
}
