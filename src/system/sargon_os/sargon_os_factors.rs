use crate::prelude::*;

/// If we wanna create an Olympia DeviceFactorSource or
/// a Babylon one, either main or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum DeviceFactorSourceType {
    Babylon { is_main: bool },
    Olympia,
}

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
    /// And also emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if the newly added FactorSource is a new **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        let id = factor_source.factor_source_id();

        let is_new_main_bdfs = factor_source.is_main_bdfs();
        let id_of_old_bdfs = self.bdfs().factor_source_id();

        let inserted = self
            .update_profile_with(|mut p| {
                Ok(p.factor_sources.append(factor_source.clone()).0)
            })
            .await?;

        if inserted {
            if is_new_main_bdfs {
                self.update_factor_source_remove_flag_main(id_of_old_bdfs)
                    .await?;
                assert_eq!(self.bdfs().factor_source_id(), id);
            }

            self.event_bus
                .emit(EventNotification::profile_modified(
                    EventProfileModified::FactorSourceAdded { id },
                ))
                .await;
        }

        Ok(inserted)
    }

    pub async fn create_device_factor_source(
        &self,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        factor_type: DeviceFactorSourceType,
    ) -> Result<DeviceFactorSource> {
        let device_info = self.device_info().await?;
        let factor_source = match factor_type {
            DeviceFactorSourceType::Olympia => DeviceFactorSource::olympia(
                &mnemonic_with_passphrase,
                &device_info,
            ),
            DeviceFactorSourceType::Babylon { is_main } => {
                DeviceFactorSource::babylon(
                    is_main,
                    &mnemonic_with_passphrase,
                    &device_info,
                )
            }
        };
        Ok(factor_source)
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
    /// Returns IDs of all the factor sources.
    pub fn factor_source_ids(&self) -> HashSet<FactorSourceID> {
        self.profile_holder.access_profile_with(|p| {
            HashSet::from_iter(p.factor_sources.iter().map(|s| s.id()))
        })
    }

    /// Updates the `last_used_on` for the factor source and emits events.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_last_used_of_factor_source(
        &self,
        factor_source_id: impl Into<FactorSourceID>,
    ) -> Result<()> {
        let id = factor_source_id.into();

        debug!(
            "Updating 'last_used_on' date for FactorSource with ID: {}",
            &id
        );

        self.update_profile_with(|mut p| {
            p.update_last_used_of_factor_source(&id)
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceUpdated { id },
            ))
            .await;

        Ok(())
    }

    pub async fn update_factor_source_remove_flag_main(
        &self,
        factor_source_id: impl Into<FactorSourceID>,
    ) -> Result<()> {
        let id = factor_source_id.into();

        debug!(
            "Updating 'flags', removing main, for FactorSource with ID: {}",
            &id
        );

        self.update_profile_with(|mut p| {
            p.update_factor_source_remove_flag_main(&id)
        })
        .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceUpdated { id },
            ))
            .await;

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
    async fn test_add_ledger_factor_source_new_bdfs_removes_main_from_existing_bdfs(
    ) {
        // ARRANGE
        let os = SUT::fast_boot().await;

        let old_bdfs_id = os.bdfs().factor_source_id();
        let new_bdfs = DeviceFactorSource::babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample(),
        );
        assert_ne!(old_bdfs_id, new_bdfs.factor_source_id());

        // ACT
        let inserted = os
            .with_timeout(|x| {
                x.add_factor_source(FactorSource::from(new_bdfs.clone()))
            })
            .await
            .unwrap();

        // ASSERT
        assert!(inserted);
        assert_eq!(os.bdfs(), new_bdfs);
        let old_bdfs = os
            .profile()
            .factor_sources
            .get_id(old_bdfs_id)
            .unwrap()
            .clone()
            .into_device()
            .unwrap();
        assert!(!old_bdfs.is_main_bdfs());
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

    #[actix_rt::test]
    async fn test_create_device_factor_source_babylon_main() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let bdfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample(),
                    DeviceFactorSourceType::Babylon { is_main: true },
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(bdfs.is_main_bdfs());
    }

    #[actix_rt::test]
    async fn test_create_device_factor_source_babylon_not_main() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let bdfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample(),
                    DeviceFactorSourceType::Babylon { is_main: false },
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!bdfs.common.is_main_bdfs());
        assert!(bdfs.common.supports_babylon());
    }

    #[actix_rt::test]
    async fn test_create_device_factor_source_olympia() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        let dfs = os
            .with_timeout(|x| {
                x.create_device_factor_source(
                    MnemonicWithPassphrase::sample_device_12_words(),
                    DeviceFactorSourceType::Olympia,
                )
            })
            .await
            .unwrap();

        // ASSERT
        assert!(!dfs.common.is_main_bdfs());
        assert!(!dfs.common.supports_babylon());
        assert!(dfs.common.supports_olympia());
        assert_eq!(
            dfs.factor_source_id(),
            DeviceFactorSource::sample_other().factor_source_id()
        );
    }
}
