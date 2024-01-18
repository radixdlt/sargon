use crate::prelude::*;

/// An abstraction of an implementing WalletClients's secure storage, used by `Wallet` to
/// save and load models, most prominently `Profile` and `MnemonicWithPassphrase`.
///
/// It uses the lower level CRUD trait `SecureStorage` which works on bytes (Vec<u8>),
/// by instead working with JSON.
///
/// The typical usage is that `Wallet` uses this to build even higher level API's that work
/// with application level types such as `PrivateHierarchicalDeterministicFactorSource`, which
/// apart from `MnemonicWithPassphrase` read from SecureStorage using this `WalletClientStorage`,
/// also has to load the DeviceFactorSource from Profile, given a FactorSourceID only.
#[derive(Debug)]
pub struct WalletClientStorage {
    /// Low level CRUD traits injected from implementing Wallet Client, that works on bytes.
    interface: Arc<dyn SecureStorage>,
}

impl WalletClientStorage {
    /// Creates a new WalletClientStorage using an implementation of
    /// `SecureStorage`.
    pub(crate) fn new(interface: Arc<dyn SecureStorage>) -> Self {
        Self { interface }
    }
}

//======
// Save T
//======
impl WalletClientStorage {
    pub fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
            .and_then(|j| self.interface.save_data(key, j))
    }
}

//======
// Load T
//======
impl WalletClientStorage {
    /// Loads bytes from SecureStorage and deserializes them into `T`.
    ///
    /// Returns `Ok(None)` if no bytes were found, returns Err if failed
    /// to load bytes or failed to deserialize the JSON into a `T`.
    pub fn load<T>(&self, key: SecureStorageKey) -> Result<Option<T>>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.interface.load_data(key).and_then(|o| match o {
            None => Ok(None),
            Some(j) => serde_json::from_slice(j.as_slice()).map_err(|_| {
                CommonError::FailedToDeserializeJSONToValue {
                    json_byte_count: j.len(),
                    type_name: std::any::type_name::<T>().to_string(),
                }
            }),
        })
    }

    /// Loads bytes from SecureStorage and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns an error if `None` bytes were found.
    pub fn load_or<T>(&self, key: SecureStorageKey, err: CommonError) -> Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.load(key).and_then(|o| o.ok_or(err))
    }

    /// Loads bytes from SecureStorage and deserializes them into `T`.
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
impl WalletClientStorage {
    /// Saves a MnemonicWithPassphrase under a given `FactorSourceIDFromHash`
    pub fn save_mnemonic_with_passphrase(
        &self,
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
        id: &FactorSourceIDFromHash,
    ) -> Result<()> {
        self.save(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id.clone(),
            },
            mnemonic_with_passphrase,
        )
        .map_err(|_| CommonError::UnableToSaveMnemonicToSecureStorage(id.clone()))
    }

    /// Loads a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub fn load_mnemonic_with_passphrase(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_or(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id.clone(),
            },
            CommonError::UnableToLoadMnemonicFromSecureStorage(id.clone()),
        )
    }

    /// Deletes a MnemonicWithPassphrase with a `FactorSourceIDFromHash`
    pub fn delete_mnemonic(&self, id: &FactorSourceIDFromHash) -> Result<()> {
        self.interface
            .delete_data_for_key(SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id.clone(),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn make_sut() -> WalletClientStorage {
        WalletClientStorage::new(EphemeralSecureStorage::new())
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
                type_name: "profile::v100::profile::Profile".to_string()
            })
        );
    }
}
