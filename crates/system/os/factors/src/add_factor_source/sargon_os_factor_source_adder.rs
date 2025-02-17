use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsFactorSourceAdder {
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool>;

    async fn add_new_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsFactorSourceAdder for SargonOS {
    /// Accesses the active profile and checks if it already contains a factor source
    /// with the same `FactorSourceID`.
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool> {
        self.profile_contains_factor_source(factor_source_id).await
    }

    /// Returns `Err(CommonError::FactorSourceAlreadyExists)` if the Profile already contained a
    /// factor source with the same id.
    ///
    /// Pre-derives and fill cache with instances for the `factor_source`. If failed to pre-derive,
    /// removes the factor source from the profile and returns the error. If successful,
    /// saves the mnemonic to secure storage.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_new_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<()> {
        let host_info = self.resolve_host_info().await;
        let factor_source = FactorSource::with_details(
            factor_source_kind,
            mnemonic_with_passphrase.clone(),
            name,
            host_info,
        )?;
        let id = factor_source.factor_source_id();

        let contains = self.profile_contains_factor_source(id).await?;

        if contains {
            return Err(CommonError::FactorSourceAlreadyExists);
        }

        self.update_profile_with(|p| {
            p.factor_sources.append(factor_source.clone());
            Ok(())
        })
        .await?;

        if factor_source_kind == FactorSourceKind::Device {
            self.secure_storage
                .save_mnemonic_with_passphrase(
                    &mnemonic_with_passphrase,
                    &factor_source.id_from_hash(),
                )
                .await?;
        }

        if let Err(e) = self
            .pre_derive_and_fill_cache_with_instances_for_factor_source(
                factor_source.clone(),
            )
            .await
        {
            self.update_profile_with(|p| {
                p.factor_sources.remove_id(&id);
                Ok(())
            })
            .await?;
            self.secure_storage
                .delete_mnemonic(&factor_source.id_from_hash())
                .await?;

            return Err(e);
        }

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceAdded { id },
            ))
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn is_factor_source_already_in_use() {
        let test = async |fsid: FactorSourceID,
                          expected_result: Result<bool>| {
            let mwp = MnemonicWithPassphrase::sample();
            let os = SUT::fast_boot_bdfs(mwp.clone()).await;

            let result = os
                .with_timeout(|x| x.is_factor_source_already_in_use(fsid))
                .await;

            pretty_assertions::assert_eq!(result, expected_result);
        };

        test(FactorSourceID::sample(), Ok(true)).await;
        test(FactorSourceID::sample_other(), Ok(false)).await;
    }

    #[actix_rt::test]
    async fn is_factor_source_already_in_use_error() {
        let bios = Bios::new(Drivers::test());
        let clients = Clients::new(bios);
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let result = os
            .with_timeout(|x| {
                x.is_factor_source_already_in_use(FactorSourceID::sample())
            })
            .await;

        assert!(matches!(
            result,
            Err(CommonError::ProfileStateNotLoaded { .. })
        ));
    }

    #[actix_rt::test]
    async fn add_new_factor_source_empty_name_error() {
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        let result = os
            .with_timeout(|x| {
                x.add_new_factor_source(
                    FactorSourceKind::Device,
                    mwp.clone(),
                    "".to_owned(),
                )
            })
            .await;

        assert!(matches!(result, Err(CommonError::InvalidDisplayNameEmpty)))
    }

    #[actix_rt::test]
    async fn add_new_factor_source_access_profile_error() {
        let bios = Bios::new(Drivers::test());
        let clients = Clients::new(bios);
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;
        let mwp = MnemonicWithPassphrase::sample();

        let result = os
            .with_timeout(|x| {
                x.add_new_factor_source(
                    FactorSourceKind::Device,
                    mwp.clone(),
                    "New device".to_owned(),
                )
            })
            .await;

        assert!(matches!(
            result,
            Err(CommonError::ProfileStateNotLoaded { .. })
        ))
    }

    #[actix_rt::test]
    async fn add_new_factor_source_already_exists_error() {
        let mwp = MnemonicWithPassphrase::sample();
        let os = SUT::fast_boot_bdfs(mwp.clone()).await;

        let result = os
            .with_timeout(|x| {
                x.add_new_factor_source(
                    FactorSourceKind::Device,
                    mwp.clone(),
                    "New device".to_owned(),
                )
            })
            .await;

        assert!(matches!(
            result,
            Err(CommonError::FactorSourceAlreadyExists)
        ))
    }

    #[actix_rt::test]
    async fn add_new_factor_source_pre_derive_instances_error() {
        let clients = Clients::new(Bios::new(Drivers::test()));
        let derivation_interactor = Arc::new(TestDerivationInteractor::fail());
        let interactors =
            Interactors::new_with_derivation_interactor(derivation_interactor);

        let mwp = MnemonicWithPassphrase::sample();
        let mwp_to_add = MnemonicWithPassphrase::sample_other();
        let fsid_from_hash =
            FactorSourceIDFromHash::new_for_device(&mwp_to_add);

        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;
        os.new_wallet_with_mnemonic(Some(mwp.clone()), false)
            .await
            .unwrap();

        let result = os
            .with_timeout(|x| {
                x.add_new_factor_source(
                    FactorSourceKind::Device,
                    mwp_to_add.clone(),
                    "New device".to_owned(),
                )
            })
            .await;

        assert!(matches!(
            result,
            Err(CommonError::TooFewFactorInstancesDerived)
        ));
        // Verify that the mnemonic is not saved to secure storage
        assert!(matches!(
            os.secure_storage.load_mnemonic(fsid_from_hash).await,
            Err(CommonError::UnableToLoadMnemonicFromSecureStorage { .. })
        ));
        // Verify that the factor source is not added to the profile
        assert!(!os
            .profile()
            .unwrap()
            .factor_sources
            .iter()
            .any(|fs| fs.factor_source_id()
                == FactorSourceID::from(fsid_from_hash)));
    }

    #[actix_rt::test]
    async fn add_new_device_factor_source_success() {
        let test = async |factor_source_kind: FactorSourceKind,
                          mwp: MnemonicWithPassphrase,
                          load_mnemonic_from_storage_result: Result<
            MnemonicWithPassphrase,
        >| {
            let event_bus_driver = RustEventBusDriver::new();
            let clients = Clients::new(Bios::new(Drivers::with_event_bus(
                event_bus_driver.clone(),
            )));
            let interactors = Interactors::new_from_clients(&clients);

            let fsid_from_hash =
                FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                    factor_source_kind,
                    &mwp,
                );
            let fsid = FactorSourceID::from(fsid_from_hash);

            let os =
                SUT::boot_with_clients_and_interactor(clients, interactors)
                    .await;
            os.new_wallet_with_mnemonic(
                Some(MnemonicWithPassphrase::sample_device()),
                false,
            )
            .await
            .unwrap();

            // Clear recorded events
            event_bus_driver.clear_recorded();

            os.with_timeout(|x| {
                x.add_new_factor_source(
                    factor_source_kind,
                    mwp.clone(),
                    "New".to_owned(),
                )
            })
            .await
            .unwrap();

            // Verify that the mnemonic is saved to secure storage
            pretty_assertions::assert_eq!(
                os.secure_storage.load_mnemonic(fsid_from_hash).await,
                load_mnemonic_from_storage_result
            );
            // Verify that the factor source is added to the profile
            assert!(os
                .profile()
                .unwrap()
                .factor_sources
                .iter()
                .any(|fs| fs.factor_source_id() == fsid));

            // Verify 2 events are emitted: `ProfileSaved` and `FactorSourceAdded`
            let events = event_bus_driver.recorded();
            assert_eq!(events.len(), 2);
            assert!(events.iter().any(|e| e.event == Event::ProfileSaved));
            assert!(events.iter().any(|e| e.event
                == Event::ProfileModified {
                    change: EventProfileModified::FactorSourceAdded {
                        id: fsid
                    }
                }));
        };

        test(
            FactorSourceKind::Device,
            MnemonicWithPassphrase::sample_device_other(),
            Ok(MnemonicWithPassphrase::sample_device_other()),
        )
        .await;
        test(
            FactorSourceKind::Password,
            MnemonicWithPassphrase::sample_password(),
            Err(CommonError::UnableToLoadMnemonicFromSecureStorage {
                bad_value: "password:181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9".to_owned(),
            }),
        ).await;
        test(
            FactorSourceKind::OffDeviceMnemonic,
            MnemonicWithPassphrase::sample_off_device(),
            Err(CommonError::UnableToLoadMnemonicFromSecureStorage {
                bad_value: "offDeviceMnemonic:820122c9573768ab572b0c9fa492a45b7b451a2740291b3da908ad423d10e410".to_owned(),
            }),
        ).await
    }
}
