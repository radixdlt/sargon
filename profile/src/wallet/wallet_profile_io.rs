use crate::prelude::*;

//========
// Wallet + SecureStorage
//========
impl Wallet {
    pub(crate) fn save_profile(&self, profile: &Profile) -> Result<()> {
        self.wallet_client_storage.save(
            SecureStorageKey::ProfileSnapshot {
                profile_id: profile.header.id.clone(),
            },
            profile,
        )
    }
    pub(crate) fn save_active_profile_id(
        &self,
        profile_id: &ProfileID,
    ) -> Result<()> {
        self.wallet_client_storage
            .save(SecureStorageKey::ActiveProfileID, profile_id)
    }
    pub(crate) fn save_active_profile_id_or_panic(
        &self,
        profile_id: &ProfileID,
    ) {
        match self.save_active_profile_id(profile_id) {
            Ok(_) => log::info!(
                "Successfully saved active ProfileID: {}",
                profile_id
            ),
            Err(e) => fatal_error(format!(
                "Failed to save active ProfileID: {}, error: {}",
                profile_id, e
            )),
        }
    }

    pub(crate) fn save_existing_profile(&self) -> Result<()> {
        self.save_profile(&self.profile())
    }

    pub(crate) fn save_profile_or_panic(&self, profile: &Profile) -> bool {
        match self.save_profile(profile) {
            Ok(_) => {
                log::info!(
                    "Successfully saved profile with ID: {}",
                    profile.id()
                );
                true
            }
            Err(e) => {
                fatal_error(format!(
                    "Failed to save profile with ID: {}, error: {}",
                    profile.id(),
                    e
                ));
                false
            }
        }
    }
    pub(crate) fn save_new_profile_or_panic(&self, profile: &Profile) {
        if self.save_profile_or_panic(profile) {
            self.save_active_profile_id_or_panic(&profile.id());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[should_panic(
        expected = "Fatal error: 'Failed to save active ProfileID: 12345678-bbbb-cccc-dddd-abcd12345678, error: Unknown Error'"
    )]
    #[test]
    fn save_active_profile_id_or_panic_fail() {
        #[derive(Debug)]
        struct FailSaveActiveProfileIDStorage {}

        impl SecureStorage for FailSaveActiveProfileIDStorage {
            fn load_data(
                &self,
                _key: SecureStorageKey,
            ) -> Result<Option<Vec<u8>>> {
                todo!()
            }

            fn save_data(
                &self,
                key: SecureStorageKey,
                _data: Vec<u8>,
            ) -> Result<()> {
                match key {
                    SecureStorageKey::ActiveProfileID => {
                        Err(CommonError::Unknown)
                    }
                    _ => Ok(()),
                }
            }

            fn delete_data_for_key(
                &self,
                _key: SecureStorageKey,
            ) -> Result<()> {
                todo!()
            }
        }
        let storage = Arc::new(FailSaveActiveProfileIDStorage {});

        _ = Wallet::by_importing_profile(Profile::placeholder(), storage);
    }

    #[should_panic(
        expected = "Fatal error: 'Failed to save profile with ID: 12345678-bbbb-cccc-dddd-abcd12345678, error: Unknown Error'"
    )]
    #[test]
    fn save_profile_or_panic_fail() {
        #[derive(Debug)]
        struct FailSaveProfileStorage {}

        impl SecureStorage for FailSaveProfileStorage {
            fn load_data(
                &self,
                _key: SecureStorageKey,
            ) -> Result<Option<Vec<u8>>> {
                todo!()
            }

            fn save_data(
                &self,
                key: SecureStorageKey,
                _data: Vec<u8>,
            ) -> Result<()> {
                match key {
                    SecureStorageKey::ProfileSnapshot { profile_id: _ } => {
                        Err(CommonError::Unknown)
                    }
                    _ => Ok(()),
                }
            }

            fn delete_data_for_key(
                &self,
                _key: SecureStorageKey,
            ) -> Result<()> {
                todo!()
            }
        }
        let storage = Arc::new(FailSaveProfileStorage {});

        _ = Wallet::by_importing_profile(Profile::placeholder(), storage);
    }

    #[should_panic(
        expected = "Fatal error: 'Failed to save active ProfileID: ffffffff-ffff-ffff-ffff-ffffffffffff, error: Unknown Error'"
    )]
    #[test]
    fn new_load_profile_with_id_fail() {
        #[derive(Debug)]
        struct FailSaveActiveProfileIDStorage {}

        impl SecureStorage for FailSaveActiveProfileIDStorage {
            fn load_data(
                &self,
                key: SecureStorageKey,
            ) -> Result<Option<Vec<u8>>> {
                match key {
                    SecureStorageKey::ProfileSnapshot { profile_id: _ } => {
                        serde_json::to_vec(&Profile::placeholder())
                            .map(Some)
                            .map_err(|_| CommonError::Unknown)
                    }
                    _ => todo!(),
                }
            }

            fn save_data(
                &self,
                key: SecureStorageKey,
                _data: Vec<u8>,
            ) -> Result<()> {
                match key {
                    SecureStorageKey::ActiveProfileID => {
                        Err(CommonError::Unknown)
                    }
                    _ => Ok(()),
                }
            }

            fn delete_data_for_key(
                &self,
                _key: SecureStorageKey,
            ) -> Result<()> {
                todo!()
            }
        }
        let storage = Arc::new(FailSaveActiveProfileIDStorage {});

        _ = Wallet::by_loading_profile_with_id(
            ProfileID::placeholder(),
            storage,
        )
        .unwrap();
    }
}
