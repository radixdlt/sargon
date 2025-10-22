use std::borrow::Borrow;

use crate::prelude::*;

/// An abstraction of an implementing host's secure storage, used to
/// save and load models, most prominently `Profile` and `MnemonicWithPassphrase`.
///
/// It uses the lower level CRUD trait `SecureStorageDriver` which works on bytes (Vec<u8>),
/// by instead working with JSON.
#[derive(Debug, Clone)]
pub struct SecureStorageClient {
    /// Low level CRUD traits that works on bytes, passed from host via BIOS when
    /// booting the SargonOS
    driver: Arc<dyn SecureStorageDriver>,
}

#[async_trait::async_trait]
impl MnemonicLoading for SecureStorageClient {
    async fn load_mnemonic(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_mnemonic_with_passphrase(id).await
    }
}

impl SecureStorageClient {
    /// Creates a new SecureStorageClient using an implementation of
    /// `SecureStorageDriver`.
    pub fn new(driver: Arc<dyn SecureStorageDriver>) -> Self {
        Self { driver }
    }
}

impl SecureStorageClient {
    //======
    // Save T
    //======
    pub async fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)?;
        self.driver
            .save_data(key, BagOfBytes::from(json))
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    //======
    // Load T
    //======
    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns `Ok(None)` if no bytes were found, returns Err if failed
    /// to load bytes or failed to deserialize the JSON into a `T`.
    pub async fn load<T>(&self, key: SecureStorageKey) -> Result<Option<T>>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.driver.load_data(key).await.and_then(|o| match o {
            None => Ok(None),
            Some(j) => serde_json::from_slice(j.as_slice())
                .map_failed_to_deserialize_bytes::<T>(j.as_slice()),
        })
    }

    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns given `err` if `None` bytes were found.
    pub async fn load_or<T>(
        &self,
        key: SecureStorageKey,
        err: CommonError,
    ) -> Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.load(key).await.and_then(|o| o.ok_or(err))
    }

    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns `default` if `None` bytes were found.
    pub async fn load_unwrap_or<T>(
        &self,
        key: SecureStorageKey,
        default: T,
    ) -> T
    where
        T: for<'a> serde::Deserialize<'a> + Clone,
    {
        self.load(key)
            .await
            .map(|o| o.unwrap_or(default.clone()))
            .unwrap_or(default)
    }

    //======
    // Profile CR(U)D
    //======

    /// Loads the Profile.
    pub async fn load_profile(&self) -> Result<Option<Profile>> {
        debug!("Loading profile");
        self.load(SecureStorageKey::load_profile_snapshot())
            .await
            .inspect(|some_profile| {
                if some_profile.is_some() {
                    debug!("Loaded profile")
                } else {
                    debug!("No profile available")
                }
            })
            .inspect_err(|e| error!("Failed to load profile, error {e}"))
    }

    /// Save `profile`
    pub async fn save_profile(&self, profile: &Profile) -> Result<()> {
        let profile_id = profile.id();
        debug!("Saving profile with id: {}", profile_id);
        self.save(
            SecureStorageKey::ProfileSnapshot {
                profile_id: profile.id(),
            },
            profile,
        )
        .await
        .inspect(|_| debug!("Saved profile with id {}", profile_id))
        .inspect_err(|e| error!("Failed to save profile, error {e}"))
    }

    //======
    // HostId CR(U)D
    //======

    /// Loads the HostId if any
    pub async fn load_host_id(&self) -> Result<Option<HostId>> {
        trace!("Loading host id");
        self.load(SecureStorageKey::HostID).await
    }

    /// Saves [`DeviceInfo`]
    pub async fn save_host_id(&self, host_id: &HostId) -> Result<()> {
        debug!("Saving new host id: {:?}", host_id);
        self.save(SecureStorageKey::HostID, host_id)
            .await
            .inspect(|_| debug!("Saved new host id."))
            .map_err(|e| {
                error!("Failed to save host id to secure storage - error {e}",);
                CommonError::UnableToSaveHostIdToSecureStorage
            })
    }

    //======
    // Mnemonic CR(U)D
    //======

    /// Saves the MnemonicWithPassphrase of the private hd factor source
    pub async fn save_private_hd_factor_source(
        &self,
        private_hd_factor_source: &PrivateHierarchicalDeterministicFactorSource,
    ) -> Result<()> {
        self.save_mnemonic_with_passphrase(
            &private_hd_factor_source.mnemonic_with_passphrase,
            &private_hd_factor_source.factor_source.id,
        )
        .await
    }

    /// Saves a MnemonicWithPassphrase under a given `FactorSourceIDFromHash`
    pub async fn save_mnemonic_with_passphrase(
        &self,
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
        id: &FactorSourceIDFromHash,
    ) -> Result<()> {
        self.save(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: *id,
            },
            mnemonic_with_passphrase,
        )
        .await
    }

    /// Loads a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub async fn load_mnemonic_with_passphrase(
        &self,
        id: impl Borrow<FactorSourceIDFromHash>,
    ) -> Result<MnemonicWithPassphrase> {
        let id = id.borrow();
        self.load_or(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: *id,
            },
            CommonError::UnableToLoadMnemonicFromSecureStorage {
                bad_value: id.to_string(),
            },
        )
        .await
    }

    /// Deletes a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub async fn delete_mnemonic(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<()> {
        self.driver
            .delete_data_for_key(SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: *id,
            })
            .await
    }

    /// Checks if a MnemonicWithPassphrase exists for the given `DeviceFactorSource`
    pub async fn contains_device_mnemonic(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<bool> {
        self.driver
            .contains_data_for_key(
                SecureStorageKey::DeviceFactorSourceMnemonic {
                    factor_source_id: device_factor_source.id,
                },
            )
            .await
    }

    pub async fn delete_profile(&self, id: ProfileID) -> Result<()> {
        warn!("Deleting profile with id: {}", id);
        self.driver
            .delete_data_for_key(SecureStorageKey::ProfileSnapshot {
                profile_id: id,
            })
            .await
    }
}

impl SecureStorageClient {
    pub fn ephemeral() -> (SecureStorageClient, Arc<EphemeralSecureStorage>) {
        let storage = EphemeralSecureStorage::new();
        (SecureStorageClient::new(storage.clone()), storage)
    }

    pub fn always_fail() -> Self {
        SecureStorageClient::new(Arc::new(AlwaysFailSecureStorage {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sut() -> SecureStorageClient {
        SecureStorageClient::ephemeral().0
    }

    #[actix_rt::test]
    async fn load_ok_when_none() {
        let sut = make_sut();
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::load_profile_snapshot())
                .await,
            Ok(None)
        );
    }

    #[actix_rt::test]
    async fn load_successful() {
        let sut = make_sut();

        let profile = Profile::sample();
        assert!(sut
            .save(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id()
                },
                &profile
            )
            .await
            .is_ok());
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ProfileSnapshot {
                profile_id: profile.id()
            })
            .await,
            Ok(Some(profile))
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_some_default_not_used() {
        let sut = make_sut();

        let profile = Profile::sample();
        assert!(sut
            .save(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id()
                },
                &profile
            )
            .await
            .is_ok());
        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile.id()
                },
                profile.clone()
            )
            .await,
            profile
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_none_default_is_used() {
        let sut = make_sut();

        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::load_profile_snapshot(),
                Profile::sample_other()
            )
            .await,
            Profile::sample_other()
        );
    }

    #[actix_rt::test]
    async fn save_mnemonic_with_passphrase() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let factor_source_id = private.factor_source.id;
        let (sut, storage) = SecureStorageClient::ephemeral();
        let key =
            SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id };
        assert_eq!(storage.load_data(key.clone()).await, Ok(None)); // not yet saved
        assert!(sut
            .save_mnemonic_with_passphrase(
                &private.mnemonic_with_passphrase,
                &factor_source_id.clone()
            )
            .await
            .is_ok());

        // Assert indeed was saved.
        assert!(storage
            .load_data(key)
            .await
            .map(|b| String::from_utf8(b.unwrap().to_vec()).unwrap())
            .unwrap()
            .contains("device"));
    }

    #[actix_rt::test]
    async fn contains_device_mnemonic() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let factor_source_id = private.factor_source.id;
        let (sut, _) = SecureStorageClient::ephemeral();

        // It doesn't contain it yet
        assert!(!sut
            .contains_device_mnemonic(private.factor_source.clone())
            .await
            .unwrap());

        // Save the mnemonic
        assert!(sut
            .save_mnemonic_with_passphrase(
                &private.mnemonic_with_passphrase,
                &factor_source_id.clone()
            )
            .await
            .is_ok());

        // Assert it contains it now
        assert!(sut
            .contains_device_mnemonic(private.factor_source)
            .await
            .unwrap());
    }

    #[actix_rt::test]
    async fn delete_mnemonic() {
        // ARRANGE
        let private =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let factor_source_id = private.factor_source.id;
        let (sut, storage) = SecureStorageClient::ephemeral();
        let key =
            SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id };
        assert!(storage
            .save_data(key.clone(), BagOfBytes::from(vec![0xde, 0xad]))
            .await
            .is_ok());
        assert_eq!(
            storage.load_data(key.clone()).await,
            Ok(Some(BagOfBytes::from(vec![0xde, 0xad])))
        ); // assert save worked

        // ACT
        assert!(sut.delete_mnemonic(&factor_source_id).await.is_ok());

        // ASSERT
        assert_eq!(storage.load_data(key).await, Ok(None));
    }

    #[actix_rt::test]
    async fn save_fail_to_serialize() {
        use serde::Serialize;
        struct AlwaysFailSerialize {}
        impl Serialize for AlwaysFailSerialize {
            fn serialize<S>(
                &self,
                _serializer: S,
            ) -> core::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                Err(serde::ser::Error::custom(CommonError::Unknown {
                    error_message: "Failed to serialize".to_string(),
                }))
            }
        }

        let (sut, _) = SecureStorageClient::ephemeral();
        assert_eq!(
            sut.save(
                SecureStorageKey::load_profile_snapshot(),
                &AlwaysFailSerialize {}
            )
            .await,
            Err(CommonError::FailedToSerializeToJSON)
        );
    }

    #[actix_rt::test]
    async fn save_fail_save_host_id() {
        let sut = SecureStorageClient::always_fail();
        assert_eq!(
            sut.save_host_id(&HostId::sample()).await,
            Err(CommonError::UnableToSaveHostIdToSecureStorage)
        );
    }
}
