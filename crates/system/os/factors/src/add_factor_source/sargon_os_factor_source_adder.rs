use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsFactorSourceAdder {
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool>;

    async fn add_new_mnemonic_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<FactorSourceID>;
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
    async fn add_new_mnemonic_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<FactorSourceID> {
        let host_info = self.resolve_host_info().await;
        let factor_source = FactorSource::with_mwp(
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

        if factor_source_kind == FactorSourceKind::Device {
            self.secure_storage
                .save_mnemonic_with_passphrase(
                    &mnemonic_with_passphrase,
                    &factor_source.id_from_hash(),
                )
                .await?
        }

        let save_result = self.update_profile_with(|p| {
                p.factor_sources.append(factor_source.clone());
                Ok(())
            })
            .await;

        if let Err(e) = save_result {
            if factor_source_kind == FactorSourceKind::Device {
                self.secure_storage
                    .delete_mnemonic(&factor_source.id_from_hash())
                    .await?;
            }

            return Err(e);
        }

        Ok(id)
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
                x.add_new_mnemonic_factor_source(
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
                x.add_new_mnemonic_factor_source(
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
                x.add_new_mnemonic_factor_source(
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
    async fn add_new_factor_source_not_supported_kind_error() {
        let test = async |factor_source_kind: FactorSourceKind,
                          mwp: MnemonicWithPassphrase| {
            let clients = Clients::new(Bios::new(Drivers::test()));
            let interactors = Interactors::new_from_clients(&clients);

            let os =
                SUT::boot_with_clients_and_interactor(clients, interactors)
                    .await;
            os.new_wallet_with_mnemonic(
                Some(MnemonicWithPassphrase::sample_device()),
                false,
            )
            .await
            .unwrap();

            let result = os
                .with_timeout(|x| {
                    x.add_new_mnemonic_factor_source(
                        factor_source_kind,
                        mwp.clone(),
                        "Not supported fs".to_owned(),
                    )
                })
                .await;

            assert!(matches!(
                result,
                Err(CommonError::InvalidFactorSourceKind { .. })
            ));
        };

        test(
            FactorSourceKind::LedgerHQHardwareWallet,
            MnemonicWithPassphrase::sample_ledger(),
        )
        .await;
        test(
            FactorSourceKind::ArculusCard,
            MnemonicWithPassphrase::sample_arculus(),
        )
        .await;
        test(
            FactorSourceKind::SecurityQuestions,
            MnemonicWithPassphrase::sample_security_questions(),
        )
        .await;
    }

    #[actix_rt::test]
    async fn add_new_factor_source_success() {
        let test = async |factor_source_kind: FactorSourceKind,
                          mwp: MnemonicWithPassphrase,
                          load_mnemonic_from_storage_result: Result<
            MnemonicWithPassphrase,
        >| {
            let clients = Clients::new(Bios::new(Drivers::test()));
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

            let result_id = os
                .with_timeout(|x| {
                    x.add_new_mnemonic_factor_source(
                        factor_source_kind,
                        mwp.clone(),
                        "New".to_owned(),
                    )
                })
                .await
                .unwrap();

            pretty_assertions::assert_eq!(result_id, fsid);

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
