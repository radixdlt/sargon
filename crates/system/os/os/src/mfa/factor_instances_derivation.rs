use crate::prelude::*;

impl SargonOS {
    /// Derives HD factor instances for the provided matrix factors and optional
    /// authentication (ROLA) factor, returning the fresh instances grouped by
    /// factor source.
    ///
    /// The method analyses the profile to find the next available derivation
    /// index per factor, reuses index `0` when a factor has not been seen before,
    /// and guarantees that matrix and ROLA paths are produced in securified key
    /// space. When a factor appears in both the matrix set and the ROLA slot, the
    /// returned map contains a single entry whose `FactorInstances` include both
    /// transaction- and authentication-signing paths, preventing duplicate map
    /// keys. Existing usage of a factor (either on securified entities or cached
    /// provisional configurations) increases the derived index accordingly.
    ///
    /// * `entity` - account or persona whose kind determines which derivation
    ///   preset is used.
    /// * `matrix_factors` - the factor sources that require transaction-signing
    ///   instances for matrix roles.
    /// * `rola_factor` - optional factor source that should yield an
    ///   authentication-signing instance.
    ///
    /// Returns an `IndexMap` keyed by `FactorSourceIDFromHash` with the freshly
    /// derived `FactorInstances` for every requested factor source.
    pub async fn derive_factor_instances(
        &self,
        entity: AccountOrPersona,
        matrix_factors: HashSet<&FactorSource>,
        rola_factor: Option<FactorSource>,
    ) -> Result<IndexMap<FactorSourceIDFromHash, FactorInstances>> {
        let profile_snapshot = self.profile()?;
        let network_id = profile_snapshot.current_network_id();
        let key_derivation_interactors = self.keys_derivation_interactor();

        // Used to determine the next derivation index for every factor source.
        let index_assigner =
            NextDerivationEntityIndexProfileAnalyzingAssigner::new(
                network_id,
                Some(Arc::new(profile_snapshot)),
            );

        // Collect all factor sources that will participate in derivation.
        let mut all_factor_sources: IndexSet<FactorSource> = matrix_factors
            .iter()
            .map(|factor| (*factor).clone())
            .collect();

        // Collect derivation paths per factor source. Factors typically produce a
        // single derivation path; the only case where two paths appear is when the
        // same factor is used for both matrix and ROLA roles.
        let mut per_factor_paths =
            IndexMap::<FactorSourceIDFromHash, IndexSet<DerivationPath>>::new();

        if let Some(rola_factor) = rola_factor {
            all_factor_sources.insert(rola_factor.clone());

            let rola_index_agnostic_path =
                DerivationPreset::rola_entity_kind(entity.get_entity_kind())
                    .index_agnostic_path_on_network(network_id);
            let default_index = HDPathComponent::from_local_key_space(
                0u32,
                rola_index_agnostic_path.key_space,
            )?;
            let next_component = index_assigner
                .next(rola_factor.id_from_hash(), rola_index_agnostic_path)
                .map(|index| index.unwrap_or(default_index))?;
            let rola_derivation_path =
                DerivationPath::from_index_agnostic_path_and_component(
                    rola_index_agnostic_path,
                    next_component,
                );

            per_factor_paths.append_or_insert_element_to(
                rola_factor.id_from_hash(),
                rola_derivation_path,
            );
        }

        let mfa_preset = DerivationPreset::mfa_entity_kind(entity.get_entity_kind());
        for factor in matrix_factors {
            let index_agnostic_path =
                mfa_preset.index_agnostic_path_on_network(network_id);
            let default_index = HDPathComponent::from_local_key_space(
                0u32,
                index_agnostic_path.key_space,
            )?;
            let next_component = index_assigner
                .next(factor.id_from_hash(), index_agnostic_path)
                .map(|index| index.unwrap_or(default_index))?;
            let derivation_path = DerivationPath::from_index_agnostic_path_and_component(
                index_agnostic_path,
                next_component,
            );

            per_factor_paths.append_or_insert_element_to(
                factor.id_from_hash(),
                derivation_path,
            );
        }

        // Derive all requested keys and translate them into FactorInstances.
        let collector = KeysCollector::new(
            all_factor_sources,
            per_factor_paths.clone(),
            key_derivation_interactors,
            DerivationPurpose::SecurifyingAccount,
        )?;

        let keys_output = collector.collect_keys().await?;

        Ok(keys_output
            .factors_by_source
            .into_iter()
            .map(|(id, factors)| {
                let instances = FactorInstances::from(factors);
                (id, instances)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[actix_rt::test]
    async fn derive_matrix_only_new_factor_produces_tx_key() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let profile = os.profile().unwrap();
        let matrix_factor = profile.factor_sources.iter().next().unwrap().clone();
        let factors = vec![matrix_factor.clone()];
        let matrix_factors: HashSet<&FactorSource> =
            factors.iter().collect();

        let entity = AccountOrPersona::sample_mainnet();

        let result = os
            .derive_factor_instances(entity, matrix_factors, None)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let factor_id = matrix_factor.id_from_hash();
        let instances = result.get(&factor_id).unwrap();
        assert_eq!(instances.len(), 1);

        let mut derived_iter = instances.clone().into_iter();
        let derived = derived_iter.next().unwrap();
        assert!(derived_iter.next().is_none());

        assert_eq!(derived.get_key_kind(), CAP26KeyKind::TransactionSigning);
        assert_eq!(derived.get_entity_kind(), CAP26EntityKind::Account);
        assert_eq!(derived.key_space(), KeySpace::Securified);

        let expected_index =
            HDPathComponent::from_local_key_space(0u32, KeySpace::Securified)
                .unwrap();
        assert_eq!(derived.derivation_path().index(), expected_index);
    }

    #[actix_rt::test]
    async fn derive_only_rola_factor_produces_auth_key() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let matrix_factors: HashSet<&FactorSource> = HashSet::new();
        let rola_factor = FactorSource::sample_ledger();
        let entity = AccountOrPersona::sample_mainnet();

        let result = os
            .derive_factor_instances(entity, matrix_factors, Some(rola_factor.clone()))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let factor_id = rola_factor.id_from_hash();
        let instances = result.get(&factor_id).unwrap();
        assert_eq!(instances.len(), 1);

        let mut derived_iter = instances.clone().into_iter();
        let derived = derived_iter.next().unwrap();
        assert!(derived_iter.next().is_none());

        assert_eq!(derived.get_key_kind(), CAP26KeyKind::AuthenticationSigning);
        assert_eq!(derived.get_entity_kind(), CAP26EntityKind::Account);
        assert_eq!(derived.key_space(), KeySpace::Securified);

        let expected_index =
            HDPathComponent::from_local_key_space(0u32, KeySpace::Securified)
                .unwrap();
        assert_eq!(derived.derivation_path().index(), expected_index);
    }

    #[actix_rt::test]
    async fn derive_multiple_matrix_factors_with_distinct_rola() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let ledger_factor = FactorSource::sample_ledger_other();
        os.with_timeout(|s| s.add_factor_source(ledger_factor.clone()))
            .await
            .unwrap();

        let rola_factor = FactorSource::sample_off_device();
        os.with_timeout(|s| s.add_factor_source(rola_factor.clone()))
            .await
            .unwrap();

        let bdfs_factor: FactorSource = os.bdfs().into();
        let matrix_sources = vec![bdfs_factor.clone(), ledger_factor.clone()];
        let matrix_factors: HashSet<&FactorSource> =
            matrix_sources.iter().collect();

        let entity = AccountOrPersona::sample_mainnet();

        let result = os
            .derive_factor_instances(
                entity,
                matrix_factors,
                Some(rola_factor.clone()),
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 3);

        for factor in &[bdfs_factor.clone(), ledger_factor.clone()] {
            let instances = result.get(&factor.id_from_hash()).unwrap();
            assert_eq!(instances.len(), 1);
            let instance = instances.clone().into_iter().next().unwrap();
            assert_eq!(instance.get_key_kind(), CAP26KeyKind::TransactionSigning);
            assert_eq!(instance.get_entity_kind(), CAP26EntityKind::Account);
            assert_eq!(instance.key_space(), KeySpace::Securified);
        }

        let rola_instances = result.get(&rola_factor.id_from_hash()).unwrap();
        assert_eq!(rola_instances.len(), 1);
        let rola_instance = rola_instances.clone().into_iter().next().unwrap();
        assert_eq!(rola_instance.get_key_kind(), CAP26KeyKind::AuthenticationSigning);
        assert_eq!(rola_instance.get_entity_kind(), CAP26EntityKind::Account);
        assert_eq!(rola_instance.key_space(), KeySpace::Securified);
    }

    #[actix_rt::test]
    async fn derive_shared_matrix_and_rola_factor_merges_instances() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let ledger_factor = FactorSource::sample_ledger();
        os.with_timeout(|s| s.add_factor_source(ledger_factor.clone()))
            .await
            .unwrap();

        let extra_matrix_factor = FactorSource::sample_off_device();
        os.with_timeout(|s| s.add_factor_source(extra_matrix_factor.clone()))
            .await
            .unwrap();

        let bdfs_factor: FactorSource = os.bdfs().into();
        let matrix_sources = vec![bdfs_factor.clone(), ledger_factor.clone(), extra_matrix_factor.clone()];
        let matrix_factors: HashSet<&FactorSource> =
            matrix_sources.iter().collect();

        let entity = AccountOrPersona::sample_mainnet();

        let result = os
            .derive_factor_instances(
                entity,
                matrix_factors,
                Some(ledger_factor.clone()),
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 3);

        // BDFS still derives a single transaction signing key
        let bdfs_instances = result.get(&bdfs_factor.id_from_hash()).unwrap();
        assert_eq!(bdfs_instances.len(), 1);
        let bdfs_instance = bdfs_instances.clone().into_iter().next().unwrap();
        assert_eq!(bdfs_instance.get_key_kind(), CAP26KeyKind::TransactionSigning);

        // Extra matrix factor also only yields transaction signing key
        let extra_instances = result
            .get(&extra_matrix_factor.id_from_hash())
            .unwrap();
        assert_eq!(extra_instances.len(), 1);
        let extra_instance = extra_instances.clone().into_iter().next().unwrap();
        assert_eq!(extra_instance.get_key_kind(), CAP26KeyKind::TransactionSigning);

        // Shared factor contains both authentication and transaction signing keys
        let shared_instances = result.get(&ledger_factor.id_from_hash()).unwrap();
        assert_eq!(shared_instances.len(), 2);
        let key_kinds = shared_instances
            .clone()
            .into_iter()
            .map(|instance| instance.get_key_kind())
            .collect::<HashSet<_>>();
        assert!(key_kinds.contains(&CAP26KeyKind::TransactionSigning));
        assert!(key_kinds.contains(&CAP26KeyKind::AuthenticationSigning));
        assert_eq!(key_kinds.len(), 2);
    }

    #[actix_rt::test]
    async fn derive_for_persona_uses_identity_presets() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let ledger_factor = FactorSource::sample_ledger_other();
        os.with_timeout(|s| s.add_factor_source(ledger_factor.clone()))
            .await
            .unwrap();

        let rola_factor = FactorSource::sample_off_device_other();
        os.with_timeout(|s| s.add_factor_source(rola_factor.clone()))
            .await
            .unwrap();

        let bdfs_factor: FactorSource = os.bdfs().into();
        let matrix_sources = vec![bdfs_factor.clone(), ledger_factor.clone()];
        let matrix_factors: HashSet<&FactorSource> =
            matrix_sources.iter().collect();

        let entity = AccountOrPersona::sample_mainnet_other();

        let result = os
            .derive_factor_instances(
                entity,
                matrix_factors,
                Some(rola_factor.clone()),
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 3);

        for factor in &[bdfs_factor.clone(), ledger_factor.clone()] {
            let instances = result.get(&factor.id_from_hash()).unwrap();
            assert_eq!(instances.len(), 1);
            let instance = instances.clone().into_iter().next().unwrap();
            assert_eq!(instance.get_key_kind(), CAP26KeyKind::TransactionSigning);
            assert_eq!(instance.get_entity_kind(), CAP26EntityKind::Identity);
            assert_eq!(instance.key_space(), KeySpace::Securified);
        }

        let rola_instances = result.get(&rola_factor.id_from_hash()).unwrap();
        assert_eq!(rola_instances.len(), 1);
        let rola_instance = rola_instances.clone().into_iter().next().unwrap();
        assert_eq!(rola_instance.get_key_kind(), CAP26KeyKind::AuthenticationSigning);
        assert_eq!(rola_instance.get_entity_kind(), CAP26EntityKind::Identity);
        assert_eq!(rola_instance.key_space(), KeySpace::Securified);
    }

    #[actix_rt::test]
    async fn derive_increments_indices_after_existing_usage() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let bdfs_factor: FactorSource = os.bdfs().into();
        let matrix_sources = vec![bdfs_factor.clone()];
        let matrix_factors: HashSet<&FactorSource> =
            matrix_sources.iter().collect();

        let first_entity = AccountOrPersona::sample_mainnet();

        let first_result = os
            .derive_factor_instances(
                first_entity,
                matrix_factors,
                Some(bdfs_factor.clone()),
            )
            .await
            .unwrap();

        let factor_id = bdfs_factor.id_from_hash();
        let first_instances = first_result
            .get(&factor_id)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();

        let initial_tx = first_instances
            .iter()
            .find(|instance| instance.get_key_kind() == CAP26KeyKind::TransactionSigning)
            .unwrap()
            .clone();
        let initial_auth = first_instances
            .iter()
            .find(|instance| instance.get_key_kind() == CAP26KeyKind::AuthenticationSigning)
            .unwrap()
            .clone();

        let primary_role = PrimaryRoleWithFactorSources::with_factors_and_threshold(
            Threshold::Specific(1),
            [bdfs_factor.clone()],
            Vec::<FactorSource>::new(),
        );
        let recovery_role = RecoveryRoleWithFactorSources::with_factors_and_threshold(
            Threshold::All,
            Vec::<FactorSource>::new(),
            Vec::<FactorSource>::new(),
        );
        let confirmation_role = ConfirmationRoleWithFactorSources::with_factors_and_threshold(
            Threshold::All,
            Vec::<FactorSource>::new(),
            Vec::<FactorSource>::new(),
        );

        let matrix_of_sources = unsafe {
            MatrixOfFactorSources::unbuilt_with_roles_and_days(
                primary_role,
                recovery_role,
                confirmation_role,
                TimePeriod::with_days(14),
            )
        };

        let security_structure_sources = SecurityStructureOfFactorSources::new(
            DisplayName::new("Existing Shield").unwrap(),
            matrix_of_sources,
            bdfs_factor.clone(),
        );

        let mut consuming_instances = first_result.clone();
        let security_structure_instances =
            SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
                &mut consuming_instances,
                None,
                &security_structure_sources,
            )
            .unwrap();

        let secured_control = SecuredEntityControl::new(
            Option::<HierarchicalDeterministicFactorInstance>::None,
            AccessControllerAddress::sample(),
            security_structure_instances,
        )
        .unwrap();

        let account_address = AccountAddress::new_from_public_key(
            initial_tx.public_key(),
            NetworkID::Mainnet,
        );

        let existing_account = Account::with(
            NetworkID::Mainnet,
            account_address,
            DisplayName::new("Existing Account").unwrap(),
            EntitySecurityState::Securified {
                value: secured_control,
            },
            Vec::<EntityFlag>::new(),
            AppearanceID::default(),
            OnLedgerSettings::default(),
        );

        os.update_profile_with(|profile| {
            profile.networks.update_with(NetworkID::Mainnet, |network| {
                network.accounts.append(existing_account.clone());
            });
            Ok(())
        })
        .await
        .unwrap();

        let matrix_sources_again = vec![bdfs_factor.clone()];
        let matrix_factors_again: HashSet<&FactorSource> =
            matrix_sources_again.iter().collect();

        let second_entity = AccountOrPersona::sample_mainnet();
        let second_result = os
            .derive_factor_instances(
                second_entity,
                matrix_factors_again,
                Some(bdfs_factor.clone()),
            )
            .await
            .unwrap();

        assert_eq!(second_result.len(), 1);

        let second_instances = second_result
            .get(&factor_id)
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(second_instances.len(), 2);

        let new_tx = second_instances
            .iter()
            .find(|instance| instance.get_key_kind() == CAP26KeyKind::TransactionSigning)
            .unwrap();
        let new_auth = second_instances
            .iter()
            .find(|instance| instance.get_key_kind() == CAP26KeyKind::AuthenticationSigning)
            .unwrap();

        let initial_tx_index = initial_tx.derivation_path().index().map_to_global_key_space();
        let new_tx_index = new_tx.derivation_path().index().map_to_global_key_space();
        assert_eq!(new_tx_index, initial_tx_index + 1);
        assert_ne!(new_tx.public_key(), initial_tx.public_key());

        let initial_auth_index = initial_auth
            .derivation_path()
            .index()
            .map_to_global_key_space();
        let new_auth_index = new_auth.derivation_path().index().map_to_global_key_space();
        assert_eq!(new_auth_index, initial_auth_index + 1);
        assert_ne!(new_auth.public_key(), initial_auth.public_key());
    }
}
