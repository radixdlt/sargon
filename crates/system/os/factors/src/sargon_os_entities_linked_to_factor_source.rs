use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsEntitiesLinkedToFactorSource {
    async fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToFactorSource>;

    async fn integrity(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorSourceIntegrity>;

    async fn device_integrity(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<FactorSourceIntegrity>;
}

#[async_trait::async_trait]
impl OsEntitiesLinkedToFactorSource for SargonOS {
    /// Returns the entities linked to a given `FactorSource`, either on the current `Profile` or a specific one.
    async fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToFactorSource> {
        let integrity = self.integrity(factor_source.clone()).await?;
        let entities_linked = match profile_to_check {
            ProfileToCheck::Current => {
                self.profile()?.current_network().ok().map(|network| {
                    network.entities_linked_to_factor_source(
                        factor_source,
                        integrity.clone(),
                    )
                })
            }
            ProfileToCheck::Specific(specific_profile) => specific_profile
                .networks
                .get_id(NetworkID::Mainnet)
                .map(|network| {
                    network.entities_linked_to_factor_source(
                        factor_source,
                        integrity.clone(),
                    )
                }),
        };

        entities_linked.unwrap_or(Ok(EntitiesLinkedToFactorSource::new(
            integrity,
            Accounts::new(),
            Accounts::new(),
            Personas::new(),
            Personas::new(),
        )))
    }

    async fn integrity(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorSourceIntegrity> {
        match factor_source {
            FactorSource::Device { value } => {
                self.device_integrity(value).await
            }
            FactorSource::Ledger { value } => Ok(value.into()),
            FactorSource::OffDeviceMnemonic { value } => Ok(value.into()),
            FactorSource::ArculusCard { value } => Ok(value.into()),
            FactorSource::Password { value } => Ok(value.into()),
            _ => Err(CommonError::Unknown {
                error_message: "Failed checking factor source integrity"
                    .to_string(),
            }),
        }
    }

    async fn device_integrity(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<FactorSourceIntegrity> {
        let is_mnemonic_present_in_secure_storage = self
            .secure_storage
            .contains_device_mnemonic(device_factor_source.clone())
            .await?;
        let is_mnemonic_marked_as_backed_up = self
            .unsafe_storage
            .check_if_mnemonic_is_backed_up(device_factor_source.clone())
            .await?;
        let result = DeviceFactorSourceIntegrity::new(
            device_factor_source,
            is_mnemonic_present_in_secure_storage,
            is_mnemonic_marked_as_backed_up,
        );
        Ok(result.into())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn current_profile__device_factor_source_present_in_secure_storage_and_backed_up(
    ) {
        // Verify the integrity and entities when the device mnemonic is present in the secure storage and marked as backed up.
        let os = boot_with_entities(true, true).await;
        let factor_source = FactorSource::sample_device();

        let result = os
            .entities_linked_to_factor_source(
                factor_source,
                ProfileToCheck::Current,
            )
            .await
            .unwrap();
        verify_entities(result.clone());
        verify_device_integrity(result.integrity, true, true);
    }

    #[actix_rt::test]
    async fn current_profile__device_factor_source_present_in_secure_storage_but_not_backed_up(
    ) {
        // Verify the integrity and entities when the device mnemonic is present in secure storage but not marked as backed up.
        let os = boot_with_entities(true, false).await;
        let factor_source = FactorSource::sample_device();

        let result = os
            .entities_linked_to_factor_source(
                factor_source,
                ProfileToCheck::Current,
            )
            .await
            .unwrap();
        verify_entities(result.clone());
        verify_device_integrity(result.integrity, true, false);
    }

    #[actix_rt::test]
    async fn current_profile__device_factor_source_missing_in_secure_storage_and_not_backed_up(
    ) {
        // Verify the integrity and entities when the device mnemonic is not present in secure storage and not marked as backed up.
        let os = boot_with_entities(false, false).await;
        let factor_source = FactorSource::sample_device();

        let result = os
            .entities_linked_to_factor_source(
                factor_source,
                ProfileToCheck::Current,
            )
            .await
            .unwrap();
        verify_entities(result.clone());
        verify_device_integrity(result.integrity, false, false);
    }

    #[actix_rt::test]
    async fn current_profile__ledger_factor_source() {
        // Verify the integrity and entities for ledger factor source.
        let os = boot_with_entities(false, false).await;
        let factor_source = FactorSource::sample_ledger();

        let result = os
            .entities_linked_to_factor_source(
                factor_source.clone(),
                ProfileToCheck::Current,
            )
            .await
            .unwrap();
        assert!(result.accounts.is_empty());
        assert!(result.hidden_accounts.is_empty());
        assert!(result.personas.is_empty());
        assert!(result.hidden_personas.is_empty());
        match result.integrity {
            FactorSourceIntegrity::Ledger(integrity) => {
                assert_eq!(integrity, *factor_source.as_ledger().unwrap());
            }
            _ => panic!("Expected Ledger integrity"),
        }
    }

    #[actix_rt::test]
    async fn specific_profile__checks_on_mainnet() {
        // Verify the entities when checking for a specific Profile (which will check on Mainnet, regardless of the current network set on Profile)
        let mut profile = Profile::sample();
        profile
            .app_preferences
            .gateways
            .change_current(Gateway::stokenet());
        let os = boot_with_entities(true, true).await;
        let factor_source = FactorSource::sample_device();

        let result = os
            .entities_linked_to_factor_source(
                factor_source,
                ProfileToCheck::Specific(profile),
            )
            .await
            .unwrap();
        verify_device_integrity(result.integrity, true, true);

        assert_eq!(result.accounts, Accounts::sample_mainnet()); // Alice and Bob, which are both visible
        assert!(result.hidden_accounts.is_empty());
        assert_eq!(result.personas, Personas::sample_mainnet()); // Satoshi and Batman, which are both visible
        assert!(result.hidden_personas.is_empty());
    }

    #[actix_rt::test]
    async fn specific_profile__mainnet_missing() {
        // Test the failure case when checking entities for a specific Profile that doesn't have Mainnet in its networks
        let profile = Profile::sample_other();
        let os = boot_with_entities(true, true).await;
        let factor_source = FactorSource::sample_device();

        let result = os
            .entities_linked_to_factor_source(
                factor_source,
                ProfileToCheck::Specific(profile),
            )
            .await
            .unwrap();
        verify_device_integrity(result.integrity, true, true);
        assert!(result.accounts.is_empty());
        assert!(result.hidden_accounts.is_empty());
        assert!(result.personas.is_empty());
        assert!(result.hidden_personas.is_empty());
    }

    #[actix_rt::test]
    async fn integrity() {
        // Verify the integrity for all types of factor sources not covered on above tests
        let os = boot_with_entities(true, true).await;

        let factor_source = FactorSource::sample_off_device();
        let result = os.integrity(factor_source.clone()).await.unwrap();
        match result {
            FactorSourceIntegrity::OffDeviceMnemonic(integrity) => {
                assert_eq!(
                    integrity,
                    *factor_source.as_off_device_mnemonic().unwrap()
                );
            }
            _ => panic!("Expected OffDeviceMnemonic integrity"),
        }

        let factor_source = FactorSource::sample_arculus();
        let result = os.integrity(factor_source.clone()).await.unwrap();
        match result {
            FactorSourceIntegrity::ArculusCard(integrity) => {
                assert_eq!(
                    integrity,
                    *factor_source.as_arculus_card().unwrap()
                );
            }
            _ => panic!("Expected ArculusCard integrity"),
        }

        let factor_source = FactorSource::sample_password();
        let result = os.integrity(factor_source.clone()).await.unwrap();
        match result {
            FactorSourceIntegrity::Password(integrity) => {
                assert_eq!(integrity, *factor_source.as_password().unwrap());
            }
            _ => panic!("Expected Password integrity"),
        }
    }

    /// Verifies the integrity corresponds to a DeviceFactorSourceIntegrity with the expected values
    fn verify_device_integrity(
        result: FactorSourceIntegrity,
        is_mnemonic_present_in_secure_storage: bool,
        is_mnemonic_marked_as_backed_up: bool,
    ) {
        match result {
            FactorSourceIntegrity::Device(integrity) => {
                assert_eq!(
                    integrity.is_mnemonic_present_in_secure_storage,
                    is_mnemonic_present_in_secure_storage
                );
                assert_eq!(
                    integrity.is_mnemonic_marked_as_backed_up,
                    is_mnemonic_marked_as_backed_up
                );
            }
            _ => panic!("Expected Device integrity"),
        }
    }

    /// Verifies the entities linked to a factor source are the expected ones.
    fn verify_entities(result: EntitiesLinkedToFactorSource) {
        assert_eq!(
            result.accounts,
            Accounts::just(Account::sample_stokenet_nadia())
        );
        assert_eq!(
            result.hidden_accounts,
            Accounts::just(Account::sample_stokenet_olivia())
        );
        assert_eq!(
            result.personas,
            Personas::just(Persona::sample_stokenet_leia_skywalker())
        );
        assert!(result.hidden_personas.is_empty());
    }

    /// Will boot SargonOS with a profile that has the following entities on Stokenet (its current network):
    /// - 1 visible Account (sample_stokenet_nadia)
    /// - 1 hidden Account (sample_stokenet_olivia)
    /// - 1 visible Persona (sample_stokenet_leia_skywalker)
    ///
    ///  And the corresponding mocked secure/unsafe storages.
    async fn boot_with_entities(
        device_mnemonic_in_secure_storage: bool,
        device_mnemonic_backed_up: bool,
    ) -> Arc<SargonOS> {
        let secure_storage =
            build_secure_storage(device_mnemonic_in_secure_storage).await;
        let unsafe_storage =
            build_unsafe_storage(device_mnemonic_backed_up).await;
        let drivers = Drivers::with_storages(secure_storage, unsafe_storage);
        let bios = Bios::new(drivers);
        let mut clients = Clients::new(bios);
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);

        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let (mut profile, _) = os.create_new_profile_with_bdfs(None).unwrap();

        let new_network = ProfileNetwork::new(
            NetworkID::Stokenet,
            Accounts::from_iter([
                Account::sample_stokenet_nadia(),
                Account::sample_stokenet_olivia(),
            ]),
            Personas::just(Persona::sample_stokenet_leia_skywalker()),
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
        );

        profile.networks.append(new_network);
        profile
            .app_preferences
            .gateways
            .change_current(Gateway::stokenet());

        os.with_timeout(|x| x.set_profile(profile.clone()))
            .await
            .unwrap();

        os
    }

    fn device_secure_key() -> SecureStorageKey {
        SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: FactorSource::sample_device().id_from_hash(),
        }
    }

    async fn build_secure_storage(
        device_mnemonic_in_secure_storage: bool,
    ) -> Arc<EphemeralSecureStorage> {
        let secure_storage = EphemeralSecureStorage::new();
        if device_mnemonic_in_secure_storage {
            secure_storage
                .save_data(device_secure_key(), BagOfBytes::from(vec![0x01]))
                .await
                .unwrap();
        }
        secure_storage
    }

    async fn build_unsafe_storage(
        device_mnemonic_backed_up: bool,
    ) -> Arc<EphemeralUnsafeStorage> {
        let unsafe_storage = EphemeralUnsafeStorage::new();
        let backed_up = if device_mnemonic_backed_up {
            vec![FactorSource::sample_device().id_from_hash()]
        } else {
            vec![]
        };
        let json = serde_json::to_vec(&backed_up).unwrap();
        unsafe_storage
            .save_data(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                BagOfBytes::from(json),
            )
            .await
            .unwrap();
        unsafe_storage
    }
}
