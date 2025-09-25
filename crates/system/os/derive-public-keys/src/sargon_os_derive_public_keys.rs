use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsDerivePublicKeys {
    async fn derive_public_keys(
        &self,
        derivation_paths: Vec<DerivationPath>,
        source: DerivePublicKeysSource,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>>;
}

#[async_trait::async_trait]
impl OsDerivePublicKeys for SargonOS {
    async fn derive_public_keys(
        &self,
        derivation_paths: Vec<DerivationPath>,
        source: DerivePublicKeysSource,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>> {
        match source {
            DerivePublicKeysSource::Mnemonic(mnemonic_with_passphrase) => {
                let public_keys = mnemonic_with_passphrase
                    .derive_public_keys_vec(derivation_paths);
                Ok(public_keys)
            }
            DerivePublicKeysSource::FactorSource(
                factor_source_id_from_hash,
            ) => {
                let id = FactorSourceID::from(factor_source_id_from_hash);
                let factor_source =
                    self.factor_sources()?.get_id(id).cloned().ok_or(
                        CommonError::ProfileDoesNotContainFactorSourceWithID {
                            bad_value: id.to_string(),
                        },
                    )?;

                let collector = KeysCollector::new(
                    vec![factor_source],
                    IndexMap::just((
                        factor_source_id_from_hash,
                        IndexSet::from_iter(derivation_paths),
                    )),
                    self.keys_derivation_interactor(),
                    DerivationPurpose::AccountRecovery,
                )?;

                let pf_derived =
                    collector.collect_keys().await?.factors_by_source;
                let result: Vec<HierarchicalDeterministicPublicKey> =
                    pf_derived
                        .get(&factor_source_id_from_hash)
                        .map(|set| {
                            set.iter().cloned().map(|a| a.public_key).collect()
                        })
                        .unwrap_or(Vec::new());
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn mnemonic__success() {
        let sut = boot(None).await;

        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();
        let source =
            DerivePublicKeysSource::Mnemonic(mnemonic_with_passphrase.clone());
        let result = sut
            .derive_public_keys(sample_derivation_paths(), source)
            .await
            .unwrap();

        assert_eq!(
            result,
            mnemonic_with_passphrase
                .derive_public_keys_vec(sample_derivation_paths())
        );
    }

    #[actix_rt::test]
    async fn factor_source__without_profile() {
        let sut = boot(None).await;

        let source = DerivePublicKeysSource::FactorSource(
            FactorSourceIDFromHash::sample(),
        );
        let result = sut
            .derive_public_keys(sample_derivation_paths(), source)
            .await
            .expect_err("Expected error");

        assert_eq!(
            result,
            CommonError::ProfileStateNotLoaded {
                current_state: "None".to_string()
            }
        );
    }

    #[actix_rt::test]
    async fn factor_source__factor_source_missing() {
        let sut = boot(Profile::sample()).await;

        let factor_source_not_in_profile =
            FactorSourceIDFromHash::sample_password();
        let source =
            DerivePublicKeysSource::FactorSource(factor_source_not_in_profile);
        let result = sut
            .derive_public_keys(sample_derivation_paths(), source)
            .await
            .expect_err("Expected error");

        assert_eq!(
            result,
            CommonError::ProfileDoesNotContainFactorSourceWithID {
                bad_value: factor_source_not_in_profile.to_string(),
            }
        );
    }

    #[actix_rt::test]
    async fn factor_source__device_success() {
        let sut = boot(Profile::sample()).await;

        let source = DerivePublicKeysSource::FactorSource(
            FactorSourceIDFromHash::sample_device(),
        );
        let result = sut
            .derive_public_keys(vec![DerivationPath::sample()], source)
            .await
            .unwrap();

        assert_eq!(result, vec![HierarchicalDeterministicPublicKey::sample(),]);
    }

    fn sample_derivation_paths() -> Vec<DerivationPath> {
        vec![DerivationPath::sample(), DerivationPath::sample_other()]
    }

    async fn boot(profile: impl Into<Option<Profile>>) -> Arc<SUT> {
        let secure_storage_driver = EphemeralSecureStorage::new();

        if let Some(profile) = profile.into() {
            let secure_storage_client =
                SecureStorageClient::new(secure_storage_driver.clone());
            secure_storage_client.save_profile(&profile).await.unwrap();
        }

        let test_drivers = Drivers::with_secure_storage(secure_storage_driver);
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();

        let interactors = Interactors::new_from_clients(&clients);
        SUT::boot_with_clients_and_interactor(clients, interactors).await
    }
}
