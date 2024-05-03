use crate::prelude::*;

/// An abstraction of an implementing WalletClients's secure storage, used by `Wallet` to
/// save and load models, most prominently `Profile` and `MnemonicWithPassphrase`.
///
/// It uses the lower level CRUD trait `SecureStorageDriver` which works on bytes (Vec<u8>),
/// by instead working with JSON.
///
/// The typical usage is that `Wallet` uses this to build even higher level API's that work
/// with application level types such as `PrivateHierarchicalDeterministicFactorSource`, which
/// apart from `MnemonicWithPassphrase` read from SecureStorageDriver using this `AppSecureStorageClient`,
/// also has to load the DeviceFactorSource from Profile, given a FactorSourceID only.
#[derive(Debug)]
pub struct AppSecureStorageClient {
    /// Low level CRUD traits injected from implementing Wallet Client, that works on bytes.
    driver: Arc<dyn SecureStorageDriver>,
}

impl AppSecureStorageClient {
    /// Creates a new AppSecureStorageClient using an implementation of
    /// `SecureStorageDriver`.
    pub(crate) fn new(driver: Arc<dyn SecureStorageDriver>) -> Self {
        Self { driver }
    }
}

impl AppSecureStorageClient {
    //======
    // Save T
    //======
    pub async fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)?;
        self.driver.save_data(key, json).await
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
            Some(j) => serde_json::from_slice(j.as_slice()).map_err(|_| {
                let type_name = std::any::type_name::<T>().to_string();
                error!(
                    "Deserialize json to type: {}\nJSON (utf8):\n{:?}",
                    &type_name,
                    String::from_utf8(j.clone())
                );
                CommonError::FailedToDeserializeJSONToValue {
                    json_byte_count: j.len() as u64,
                    type_name,
                }
            }),
        })
    }

    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns an error if `None` bytes were found.
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
        let Some(id) = self.load_active_profile_id().await? else {
            return Ok(None);
        };
        self.load_or(
            SecureStorageKey::ProfileSnapshot { profile_id: id },
            CommonError::UnableToLoadProfileFromSecureStorage {
                profile_id: id,
            },
        )
        .await
    }

    /// Loads the active ProfileID if any
    pub async fn load_active_profile_id(&self) -> Result<Option<ProfileID>> {
        self.load(SecureStorageKey::ActiveProfileID).await
    }

    /// Save `profile` and saves its id as active profile id
    pub async fn save_profile_and_active_profile_id(
        &self,
        profile: &Profile,
    ) -> Result<()> {
        self.save_profile(profile).await?;
        self.save_active_profile_id(profile.id()).await
    }

    /// Save `profile`
    pub async fn save_profile(&self, profile: &Profile) -> Result<()> {
        let profile_id = profile.id();
        self.save(SecureStorageKey::ProfileSnapshot { profile_id }, profile)
            .await
    }

    /// Save `profile_id` as the active profile id
    pub async fn save_active_profile_id(
        &self,
        profile_id: ProfileID,
    ) -> Result<()> {
        self.save(SecureStorageKey::ActiveProfileID, &profile_id)
            .await
    }

    //======
    // DeviceInfo CR(U)D
    //======

    /// Loads the  DeviceInfo if any
    pub async fn load_device_info(&self) -> Result<Option<DeviceInfo>> {
        self.load(SecureStorageKey::DeviceInfo).await
    }

    /// Saves [`DeviceInfo`]
    pub async fn save_device_info(
        &self,
        device_info: &DeviceInfo,
    ) -> Result<()> {
        self.save(SecureStorageKey::DeviceInfo, device_info)
            .await
            .map_err(|_| CommonError::UnableToSaveDeviceInfoToSecureStorage)
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
}

#[cfg(test)]
impl AppSecureStorageClient {
    pub(crate) fn ephemeral(
    ) -> (AppSecureStorageClient, Arc<EphemeralSecureStorage>) {
        let storage = EphemeralSecureStorage::new();
        (AppSecureStorageClient::new(storage.clone()), storage)
    }

    pub(crate) fn always_fail() -> Self {
        AppSecureStorageClient::new(Arc::new(AlwaysFailStorage {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sut() -> AppSecureStorageClient {
        AppSecureStorageClient::ephemeral().0
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
                type_name: "sargon::profile::v100::profile::Profile"
                    .to_string()
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
        let private =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let factor_source_id = private.factor_source.id;
        let (sut, storage) = AppSecureStorageClient::ephemeral();
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
            .map(|b| String::from_utf8(b.unwrap()).unwrap())
            .unwrap()
            .contains("zoo"));
    }

    #[actix_rt::test]
    async fn save_mnemonic_with_passphrase_failure() {
        let sut = AppSecureStorageClient::always_fail();
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
        let (sut, storage) = AppSecureStorageClient::ephemeral();
        let key =
            SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id };
        assert!(storage
            .save_data(key.clone(), vec![0xde, 0xad])
            .await
            .is_ok());
        assert_eq!(
            storage.load_data(key.clone()).await,
            Ok(Some(vec![0xde, 0xad]))
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

        let (sut, _) = AppSecureStorageClient::ephemeral();
        assert_eq!(
            sut.save(
                SecureStorageKey::ActiveProfileID,
                &AlwaysFailSerialize {}
            )
            .await,
            Err(CommonError::FailedToSerializeToJSON)
        );
    }
}
