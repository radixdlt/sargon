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

    /// Returns the "main Babylon" `DeviceFactorSource` of the current account as
    /// a `DeviceFactorSource`.
    pub fn bdfs(&self) -> DeviceFactorSource {
        self.profile_holder.access_profile_with(|p| p.bdfs())
    }
}

impl SargonOS {
    /// Tries to load a `MnemonicWithPassphrase` from secure storage
    /// by `id` of type `FactorSourceIDFromHash`.
    pub async fn mnemonic_with_passphrase_of_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.secure_storage.load_mnemonic_with_passphrase(id).await
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
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    /// Tries to load the  `MnemonicWithPassphrase` for the main "Babylon"
    /// `DeviceFactorSource` from secure storage.
    pub async fn main_bdfs_mnemonic_with_passphrase(
        &self,
    ) -> Result<MnemonicWithPassphrase> {
        let bdfs = self.profile_holder.access_profile_with(|p| p.bdfs());
        self.mnemonic_with_passphrase_of_device_factor_source_by_id(&bdfs.id)
            // tarpaulin will incorrectly flag next line is missed
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_load_private_device_factor_source_by_id() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let factor_source_id = FactorSourceIDFromHash::new_for_device(&mwp);
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let private = os
            .with_timeout(|x| {
                x.load_private_device_factor_source_by_id(&factor_source_id)
            })
            .await
            .unwrap();

        // ASSERT
        assert_eq!(private.mnemonic_with_passphrase, mwp);
    }

    #[actix_rt::test]
    async fn test_bdfs() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let loaded = os.bdfs();

        // ASSERT
        assert_eq!(
            loaded.factor_source_id(),
            FactorSourceIDFromHash::new_for_device(&mwp).into()
        );
    }

    #[actix_rt::test]
    async fn test_mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(
    ) {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let factor_source_id = FactorSourceIDFromHash::new_for_device(&mwp);
        let id = FactorSourceID::from(factor_source_id);
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        // ACT
        let loaded = os
      .with_timeout(|x| {
          x.mnemonic_with_passphrase_of_device_factor_source_by_factor_source_id(&id)
      })
      .await
      .unwrap();

        // ASSERT
        assert_eq!(loaded, mwp);
    }
}
