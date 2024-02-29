use crate::prelude::*;

impl Wallet {
    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `id` of type `FactorSourceIDFromHash`.
    pub fn mnemonic_with_passphrase_of_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.wallet_client_storage.load_mnemonic_with_passphrase(id)
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub fn load_private_device_factor_source(
        &self,
        device_factor_source: &DeviceFactorSource,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        info!(
            "Load Private DeviceFactorSource from SecureStorage, factor source id: {}",
            &device_factor_source.id
        );
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(
            &device_factor_source.id,
        )
        .map(|mwp| {
            PrivateHierarchicalDeterministicFactorSource::new(
                mwp,
                device_factor_source.clone(),
            )
        })
        .log_info(
            "Successfully loaded Private DeviceFactorSource from SecureStorage",
        )
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let device_factor_source =
            self.profile().device_factor_source_by_id(id)?;
        self.load_private_device_factor_source(&device_factor_source)
    }
}

#[uniffi::export]
impl Wallet {
    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `factor_source_id`.
    pub fn mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Result<MnemonicWithPassphrase> {
        factor_source_id
            .clone()
            .into_hash()
            .map_err(|_| CommonError::FactorSourceIDNotFromHash)
            .and_then(|id| {
                self.mnemonic_with_passphrase_of_device_factor_source_by_id(&id)
            })
    }

    /// Tries to load the  `MnemonicWithPassphrase` for the main "Babylon"
    /// `DeviceFactorSource` from secure storage.
    pub fn main_bdfs_mnemonic_with_passphrase(
        &self,
    ) -> Result<MnemonicWithPassphrase> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(&bdfs.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn main_bdfs_mnemonic_with_passphrase() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let dfs = private.factor_source;
        let profile = Profile::sample();
        let (wallet, storage) = Wallet::ephemeral(profile.clone());
        let data =
            serde_json::to_vec(&private.mnemonic_with_passphrase).unwrap();
        let key = SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: dfs.id.clone(),
        };
        storage.save_data(key.clone(), data.clone()).unwrap();
        assert_eq!(
            wallet.main_bdfs_mnemonic_with_passphrase().unwrap(),
            MnemonicWithPassphrase::sample()
        );
        assert_eq!(
            wallet.mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(&dfs.factor_source_id()).unwrap(),
            MnemonicWithPassphrase::sample()
        );
    }

    #[test]
    fn mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id_fail_not_factor_source_id_from_hash(
    ) {
        let (wallet, _) = Wallet::ephemeral(Profile::sample());
        assert_eq!(
            wallet.mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(&FactorSourceIDFromAddress::sample().into()),
          Err(CommonError::FactorSourceIDNotFromHash)
        );
    }
}
