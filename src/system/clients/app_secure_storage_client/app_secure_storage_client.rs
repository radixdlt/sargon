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

//======
// Save T
//======
impl AppSecureStorageClient {
    pub fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
            .and_then(|j| self.driver.save_data(key, j))
    }
}

//======
// Load T
//======
impl AppSecureStorageClient {
    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns `Ok(None)` if no bytes were found, returns Err if failed
    /// to load bytes or failed to deserialize the JSON into a `T`.
    #[cfg(not(tarpaulin_include))] // false negative
    pub fn load<T>(&self, key: SecureStorageKey) -> Result<Option<T>>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.driver.load_data(key).and_then(|o| match o {
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
    pub fn load_or<T>(
        &self,
        key: SecureStorageKey,
        err: CommonError,
    ) -> Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.load(key).and_then(|o| o.ok_or(err))
    }

    /// Loads bytes from SecureStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns `default` if `None` bytes were found.
    pub fn load_unwrap_or<T>(&self, key: SecureStorageKey, default: T) -> T
    where
        T: for<'a> serde::Deserialize<'a> + Clone,
    {
        self.load(key)
            .map(|o| o.unwrap_or(default.clone()))
            .unwrap_or(default)
    }
}

//======
// Mnemonic CR(U)D
//======
impl AppSecureStorageClient {
    /// Saves a MnemonicWithPassphrase under a given `FactorSourceIDFromHash`
    pub fn save_mnemonic_with_passphrase(
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
        .map_err(|_| {
            CommonError::UnableToSaveMnemonicToSecureStorage { bad_value: *id }
        })
    }

    /// Loads a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub fn load_mnemonic_with_passphrase(
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
    }

    /// Deletes a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub fn delete_mnemonic(&self, id: &FactorSourceIDFromHash) -> Result<()> {
        self.driver.delete_data_for_key(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: *id,
            },
        )
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
    use ::hex::FromHex;

    use crate::{prelude::*, system::secure_storage::ephemeral_secure_storage};
    use std::{fmt::Write, sync::RwLock};

    fn make_sut() -> AppSecureStorageClient {
        AppSecureStorageClient::ephemeral().0
    }

    #[test]
    fn load_ok_when_none() {
        let sut = make_sut();
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID),
            Ok(None)
        );
    }

    #[test]
    fn load_fail_to_deserialize_json() {
        let sut = make_sut();

        assert!(sut
            .save(
                SecureStorageKey::ActiveProfileID,
                &0u8, // obviously a u8 is not a Profile
            )
            .is_ok());
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID),
            Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: 1,
                type_name: "sargon::profile::v100::profile::Profile"
                    .to_string()
            })
        );
    }

    #[test]
    fn load_successful() {
        let sut = make_sut();

        assert!(sut
            .save(SecureStorageKey::ActiveProfileID, &Profile::sample())
            .is_ok());
        assert_eq!(
            sut.load::<Profile>(SecureStorageKey::ActiveProfileID),
            Ok(Some(Profile::sample()))
        );
    }

    #[test]
    fn load_unwrap_or_some_default_not_used() {
        let sut = make_sut();

        assert!(sut
            .save(SecureStorageKey::ActiveProfileID, &Profile::sample())
            .is_ok());
        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::ActiveProfileID,
                Profile::sample_other()
            ),
            Profile::sample()
        );
    }

    #[test]
    fn load_unwrap_or_none_default_is_used() {
        let sut = make_sut();

        assert_eq!(
            sut.load_unwrap_or::<Profile>(
                SecureStorageKey::ActiveProfileID,
                Profile::sample_other()
            ),
            Profile::sample_other()
        );
    }

    #[test]
    fn save_mnemonic_with_passphrase() {
        let private =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let factor_source_id = private.factor_source.id;
        let (sut, storage) = AppSecureStorageClient::ephemeral();
        let key =
            SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id };
        assert_eq!(storage.load_data(key.clone()), Ok(None)); // not yet saved
        assert!(sut
            .save_mnemonic_with_passphrase(
                &private.mnemonic_with_passphrase,
                &factor_source_id.clone()
            )
            .is_ok());

        // Assert indeed was saved.
        assert!(storage
            .load_data(key)
            .map(|b| String::from_utf8(b.unwrap()).unwrap())
            .unwrap()
            .contains("zoo"));
    }

    #[test]
    fn save_mnemonic_with_passphrase_failure() {
        let sut = AppSecureStorageClient::always_fail();
        let id = FactorSourceIDFromHash::sample();
        assert_eq!(
            sut.save_mnemonic_with_passphrase(
                &MnemonicWithPassphrase::sample(),
                &id
            ),
            Err(CommonError::UnableToSaveMnemonicToSecureStorage {
                bad_value: id
            })
        );
    }

    #[test]
    fn delete_mnemonic() {
        // ARRANGE
        let private =
            PrivateHierarchicalDeterministicFactorSource::sample_other();
        let factor_source_id = private.factor_source.id;
        let (sut, storage) = AppSecureStorageClient::ephemeral();
        let key =
            SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id };
        assert!(storage.save_data(key.clone(), vec![0xde, 0xad]).is_ok());
        assert_eq!(storage.load_data(key.clone()), Ok(Some(vec![0xde, 0xad]))); // assert save worked

        // ACT
        assert!(sut.delete_mnemonic(&factor_source_id).is_ok());

        // ASSERT
        assert_eq!(storage.load_data(key), Ok(None));
    }

    #[test]
    fn save_fail_to_serialize() {
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
            ),
            Err(CommonError::FailedToSerializeToJSON)
        );
    }
}
