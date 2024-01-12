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
            Some(j) => serde_json::from_slice(j.as_slice())
                .map_err(|_| CommonError::FailedToDeserializeToJSON),
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

    pub(crate) fn assert_not_contains_profile_with_id(&self, profile_id: ProfileID) {
        if self.load_headers_list_or_empty().contains_id(&profile_id) {
            fatal_error(format!("Profile with id {profile_id}"))
        }
    }

    pub fn save<T>(&self, key: SecureStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
            .and_then(|j| self.interface.save_data(key, j))
    }

    pub fn load_private_device_factor_source(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_or(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: id,
            },
            CommonError::UnableToLoadDeviceFactorSourceFromSecureStorage,
        )
    }
}
