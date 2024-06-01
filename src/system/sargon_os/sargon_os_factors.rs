use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns the "main Babylon" `DeviceFactorSource` of the current account as
    /// a `DeviceFactorSource`.
    pub fn bdfs(&self) -> DeviceFactorSource {
        self.profile_holder.access_profile_with(|p| p.bdfs())
    }

    /// Returns all the factor sources
    pub fn factor_sources(&self) -> FactorSources {
        self.profile_holder
            .access_profile_with(|p| p.factor_sources.clone())
    }

    /// Returns `Ok(false)` if the Profile already contained a factor source with the
    /// same id (Profile unchanged occurred).
    ///    
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        let id = factor_source.factor_source_id();
        let inserted = self
            .update_profile_with(|mut p| {
                Ok(p.factor_sources.append(factor_source.clone()).0)
            })
            .await?;

        if inserted {
            self.event_bus
                .emit(EventNotification::profile_modified(
                    EventProfileModified::FactorSourceAdded { id },
                ))
                .await;
        }

        Ok(inserted)
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

    #[actix_rt::test]
    async fn test_add_ledger_factor_source() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let inserted = os
            .with_timeout(|x| x.add_factor_source(FactorSource::sample_other()))
            .await
            .unwrap();

        // ASSERT
        assert!(inserted);
        assert!(os
            .profile()
            .factor_sources
            .contains_by_id(&FactorSource::sample_other()));
    }

    #[actix_rt::test]
    async fn test_add_existing_factor_source_is_noop() {
        // ARRANGE
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        let bdfs = os.bdfs();

        // ACT
        let inserted = os
            .with_timeout(|x| {
                x.add_factor_source(
                    DeviceFactorSource::babylon(
                        false,
                        &mwp,
                        &DeviceInfo::sample_other(),
                    )
                    .into(),
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!inserted); // already exists
        assert_eq!(
            os.profile().factor_sources,
            FactorSources::just(bdfs.into())
        );
    }
}
