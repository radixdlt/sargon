use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::creating_device::CreatingDevice;

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub id: Uuid,
    #[serde(rename = "snapshotVersion")]
    pub snapshot_version: u16,
    #[serde(rename = "creatingDevice")]
    pub creating_device: CreatingDevice,
    #[serde(rename = "lastUsedOnDevice")]
    pub last_used_on_device: CreatingDevice,
    #[serde(rename = "lastModified")]
    pub last_modified: NaiveDateTime,
}
