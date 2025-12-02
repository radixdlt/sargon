use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccessControllerDetailsCacheSnapshot(pub ACDCStorage);

impl AccessControllerDetailsCacheSnapshot {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<AccessControllerDetailsCacheSnapshot> for ACDCStorage {
    fn from(value: AccessControllerDetailsCacheSnapshot) -> Self {
        value.0
    }
}

impl From<AccessControllerDetailsCacheSnapshot>
    for AccessControllerDetailsCache
{
    fn from(value: AccessControllerDetailsCacheSnapshot) -> Self {
        Self::with_storage(ACDCStorage::from(value))
    }
}

impl From<ACDCStorage> for AccessControllerDetailsCacheSnapshot {
    fn from(value: ACDCStorage) -> Self {
        Self(value)
    }
}
