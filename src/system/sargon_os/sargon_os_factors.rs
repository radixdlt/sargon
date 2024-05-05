use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub async fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let device_factor_source = self
            .profile_holder
            .access_profile_with(|p| p.device_factor_source_by_id(id))?;
        self.load_private_device_factor_source(&device_factor_source)
            .await
    }

    pub fn bdfs(&self) -> DeviceFactorSource {
        self.profile_holder.access_profile_with(|p| p.bdfs())
    }

    /// Deletes the profile and the active profile id and all references Device
    /// factor sources from secure storage.
    pub async fn delete_profile_and_mnemonics(&self) -> Result<()> {
        let secure_storage = &self.clients.secure_storage;
        let device_factor_sources = self
            .profile_holder
            .access_profile_with(|p| p.device_factor_sources());

        for dfs in device_factor_sources.iter() {
            secure_storage.delete_mnemonic(&dfs.id).await?
        }

        secure_storage.delete_profile(self.profile().id()).await?;
        secure_storage.delete_active_profile_id().await?;
        Ok(())
    }
}

impl SargonOS {
    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `id` of type `FactorSourceIDFromHash`.
    pub async fn mnemonic_with_passphrase_of_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.clients
            .secure_storage
            .load_mnemonic_with_passphrase(id)
            .await
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
    pub async fn load_private_device_factor_source(
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
        .await
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

    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `factor_source_id`.
    pub async fn mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Result<MnemonicWithPassphrase> {
        let id = factor_source_id
            .clone()
            .into_hash()
            .map_err(|_| CommonError::FactorSourceIDNotFromHash)?;
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(&id)
            .await
    }

    /// Tries to load the  `MnemonicWithPassphrase` for the main "Babylon"
    /// `DeviceFactorSource` from secure storage.
    pub async fn main_bdfs_mnemonic_with_passphrase(
        &self,
    ) -> Result<MnemonicWithPassphrase> {
        let bdfs = self.profile_holder.access_profile_with(|p| p.bdfs());
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(&bdfs.id)
            .await
    }
}
