use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{content_hint::ContentHint, device_info::DeviceInfo};

/// The header of a Profile
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    /// An immutable and unique identifier of a Profile.
    pub id: Uuid,

    /// A versioning number that is increased when breaking
    /// changes is made to ProfileSnapshot JSON data format.
    #[serde(rename = "snapshotVersion")]
    pub snapshot_version: u16,

    /// The device which was used to create this Profile.
    #[serde(rename = "creatingDevice")]
    pub creating_device: DeviceInfo,

    #[serde(rename = "lastUsedOnDevice")]
    pub last_used_on_device: DeviceInfo,

    #[serde(rename = "lastModified")]
    pub last_modified: NaiveDateTime,

    #[serde(rename = "contentHint")]
    pub content_hint: ContentHint,
}
