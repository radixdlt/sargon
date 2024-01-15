use crate::prelude::*;

#[derive(Debug)]
pub struct WalletClientStorage {
    interface: Arc<dyn SecureStorage>,
}
impl WalletClientStorage {
    pub(crate) fn new(interface: Arc<dyn SecureStorage>) -> Self {
        Self { interface }
    }
}
impl WalletClientStorage {
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

    /// Like `load` but returns `Result<T>` instead of `Result<Option<T>>` and throws the provided error if
    /// the value was `None`.
    pub fn load_or<T>(&self, key: SecureStorageKey, err: CommonError) -> Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.load(key).and_then(|o| o.ok_or(err))
    }

    /// Like `load` but returns `T` instead of `Result<Option<T>>` and defaults to `default`, if `load` returned `Ok(None)` or `Err`.
    pub fn load_unwrap_or<T>(&self, key: SecureStorageKey, default: T) -> T
    where
        T: for<'a> serde::Deserialize<'a> + Clone,
    {
        self.load(key)
            .map(|o| o.unwrap_or(default.clone()))
            .unwrap_or(default)
    }

    pub fn load_headers_list_or_empty(&self) -> HeadersList {
        self.load_unwrap_or(SecureStorageKey::SnapshotHeadersList, HeadersList::new())
    }

    pub fn load_mnemonic_with_passphrase(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_or(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id.clone(),
            },
            CommonError::UnableToLoadDeviceFactorSourceFromSecureStorage,
        )
    }

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

    pub fn delete_mnemonic(&self, id: &FactorSourceIDFromHash) -> Result<()> {
        self.interface
            .delete_data_for_key(SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id.clone(),
            })
    }

    pub fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
            .and_then(|j| self.interface.save_data(key, j))
    }
}
