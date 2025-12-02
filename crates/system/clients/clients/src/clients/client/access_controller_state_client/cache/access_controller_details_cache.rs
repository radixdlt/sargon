use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AccessControllerDetailsCache {
    /// AccessControllerStateDetails by AccessControllerAddress
    map: RwLock<ACDCStorage>,
}

pub type ACDCStorage =
    IndexMap<AccessControllerAddress, AccessControllerStateDetails>;

impl AccessControllerDetailsCache {
    pub fn with_storage(storage: ACDCStorage) -> Self {
        Self {
            map: RwLock::new(storage),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            map: RwLock::new(self.map.read().unwrap().clone()),
        }
    }

    pub fn serializable_snapshot(
        &self,
    ) -> AccessControllerDetailsCacheSnapshot {
        AccessControllerDetailsCacheSnapshot::from(
            self.map.read().unwrap().clone(),
        )
    }

    pub fn insert(&self, details: &AccessControllerStateDetails) -> Result<()> {
        let mut binding =
            self.map.write().map_err(|_| CommonError::Unknown {
                error_message:
                    "Insert AC details into cache write lock failure"
                        .to_string(),
            })?;
        binding.insert(details.address, details.clone());
        Ok(())
    }

    pub fn get(
        &self,
        address: &AccessControllerAddress,
    ) -> Result<AccessControllerStateDetails> {
        self.map
            .read()
            .unwrap()
            .get(address)
            .cloned()
            .ok_or(CommonError::EntityNotFound)
    }
}
