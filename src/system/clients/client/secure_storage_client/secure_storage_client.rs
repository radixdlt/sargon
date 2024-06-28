use crate::prelude::*;

/// An abstraction of an implementing host's secure storage, used to
/// save and load models, most prominently `Profile` and `MnemonicWithPassphrase`.
///
/// It uses the lower level CRUD trait `SecureStorageDriver` which works on bytes (Vec<u8>),
/// by instead working with JSON.
#[derive(Debug)]
pub struct SecureStorageClient {
    /// Low level CRUD traits that works on bytes, passed from host via BIOS when
    /// booting the SargonOS
    driver: Arc<dyn SecureStorageDriver>,
}

impl SecureStorageClient {
    /// Creates a new SecureStorageClient using an implementation of
    /// `SecureStorageDriver`.
    pub(crate) fn new(driver: Arc<dyn SecureStorageDriver>) -> Self {
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

    /// Loads the active Profile if any, by first loading the active
    /// profile id.
    pub async fn load_active_profile(&self) -> Result<Option<Profile>> {
        debug!("Loading active profile");
        let Some(id) = self.load_active_profile_id().await? else {
            trace!("Found no active profile id");
            return Ok(None);
        };
        self.load_profile_with_id(id).await.map(Some)
    }

    /// Loads the Profile with the given `profile_id`.
    pub async fn load_profile_with_id(
        &self,
        profile_id: ProfileID,
    ) -> Result<Profile> {
        debug!("Loading profile profile with id: {}", profile_id);
        self.load_or(
            SecureStorageKey::ProfileSnapshot { profile_id },
            CommonError::UnableToLoadProfileFromSecureStorage { profile_id },
        )
        .await
        .inspect(|_| debug!("Loaded profile"))
        .inspect_err(|e| error!("Failed to load profile, error {e}"))
    }

    /// Loads the active ProfileID if any
    pub async fn load_active_profile_id(&self) -> Result<Option<ProfileID>> {
        trace!("Loading active profile id");
        self.load(SecureStorageKey::ActiveProfileID).await
    }

    /// Save `profile` and saves its id as active profile id
    pub async fn save_profile_and_active_profile_id(
        &self,
        profile: &Profile,
    ) -> Result<()> {
        debug!(
            "Saving profile, id: {}, and setting it as active",
            &profile.id()
        );
        self.save_profile(profile).await?;
        self.save_active_profile_id(profile.id()).await
    }

    /// Save `profile`
    pub async fn save_profile(&self, profile: &Profile) -> Result<()> {
        let profile_id = profile.id();
        debug!("Saving profile with id: {}", profile_id);
        self.save(SecureStorageKey::ProfileSnapshot { profile_id }, profile)
            .await
            .inspect(|_| debug!("Saved profile with id {}", profile_id))
            .inspect_err(|e| error!("Failed to save profile, error {e}"))
    }

    /// Save `profile_id` as the active profile id
    pub async fn save_active_profile_id(
        &self,
        profile_id: ProfileID,
    ) -> Result<()> {
        debug!("Saving active profile id: {}", profile_id);
        self.save(SecureStorageKey::ActiveProfileID, &profile_id)
            .await
            .inspect(|_| debug!("Saved active profile id"))
            .inspect_err(|e| {
                error!("Failed to save active profile id, error {e}")
            })
    }

    //======
    // DeviceInfo CR(U)D
    //======

    /// Loads the  DeviceInfo if any
    pub async fn load_device_info(&self) -> Result<Option<DeviceInfo>> {
        trace!("Loading device info");
        self.load(SecureStorageKey::DeviceInfo).await
    }

    /// Saves [`DeviceInfo`]
    pub async fn save_device_info(
        &self,
        device_info: &DeviceInfo,
    ) -> Result<()> {
        debug!("Saving new device info: {:?}", device_info);
        self.save(SecureStorageKey::DeviceInfo, device_info)
            .await
            .inspect(|_| debug!("Saved new device info."))
            .map_err(|e| {
                error!(
                    "Failed to save device info to secure storage - error {e}",
                );
                CommonError::UnableToSaveDeviceInfoToSecureStorage
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
        .map_err(|_| {
            CommonError::UnableToSaveMnemonicToSecureStorage { bad_value: *id }
        })
    }

    /// Loads a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub async fn load_mnemonic_with_passphrase(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_or(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: *id,
            },
            CommonError::UnableToLoadMnemonicFromSecureStorage {
                bad_value: *id,
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

    pub async fn delete_profile(&self, id: ProfileID) -> Result<()> {
        warn!("Deleting profile with id: {}", id);
        self.driver
            .delete_data_for_key(SecureStorageKey::ProfileSnapshot {
                profile_id: id,
            })
            .await
    }

    pub async fn delete_active_profile_id(&self) -> Result<()> {
        warn!("Deleting active profile id");
        self.driver
            .delete_data_for_key(SecureStorageKey::ActiveProfileID)
            .await
    }
}

#[cfg(test)]
impl SecureStorageClient {
    pub(crate) fn ephemeral(
    ) -> (SecureStorageClient, Arc<EphemeralSecureStorage>) {
        let storage = EphemeralSecureStorage::new();
        (SecureStorageClient::new(storage.clone()), storage)
    }

    pub(crate) fn always_fail() -> Self {
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
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID).await,
            Ok(None)
        );
    }

    #[actix_rt::test]
    async fn load_fail_to_deserialize_json() {
        let sut = make_sut();

        assert!(sut
            .save(
                SecureStorageKey::ActiveProfileID,
                &0u8, // obviously a u8 is not a Profile
            )
            .await
            .is_ok());
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID).await,
            Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: 1,
                type_name: "Profile".to_owned(),
                serde_message: "invalid type: integer `0`, expected struct Profile at line 1 column 1".to_owned(),
            })
        );
    }

    #[actix_rt::test]
    async fn load_successful() {
        let sut = make_sut();

        assert!(sut
            .save(SecureStorageKey::ActiveProfileID, &Profile::sample())
            .await
            .is_ok());
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID).await,
            Ok(Some(Profile::sample()))
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_some_default_not_used() {
        let sut = make_sut();

        assert!(sut
            .save(SecureStorageKey::ActiveProfileID, &Profile::sample())
            .await
            .is_ok());
        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::ActiveProfileID,
                Profile::sample_other()
            )
            .await,
            Profile::sample()
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_none_default_is_used() {
        let sut = make_sut();

        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::ActiveProfileID,
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
    async fn save_mnemonic_with_passphrase_failure() {
        let sut = SecureStorageClient::always_fail();
        let id = FactorSourceIDFromHash::sample();
        assert_eq!(
            sut.save_mnemonic_with_passphrase(
                &MnemonicWithPassphrase::sample(),
                &id
            )
            .await,
            Err(CommonError::UnableToSaveMnemonicToSecureStorage {
                bad_value: id
            })
        );
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
                Err(serde::ser::Error::custom(CommonError::Unknown))
            }
        }

        let (sut, _) = SecureStorageClient::ephemeral();
        assert_eq!(
            sut.save(
                SecureStorageKey::ActiveProfileID,
                &AlwaysFailSerialize {}
            )
            .await,
            Err(CommonError::FailedToSerializeToJSON)
        );
    }

    #[actix_rt::test]
    async fn save_fail_save_device_info() {
        let sut = SecureStorageClient::always_fail();
        assert_eq!(
            sut.save_device_info(&DeviceInfo::sample()).await,
            Err(CommonError::UnableToSaveDeviceInfoToSecureStorage)
        );
    }
}
