use crate::prelude::*;

impl SargonOS {
    pub async fn make_setup_security_shield_manifest(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;

        let mut instances = self
            .derive_factor_instances(
                entity.clone(),
                security_structure.matrix_of_factors.all_factors(),
                Some(security_structure.authentication_signing_factor.clone()),
            )
            .await?;

        // 3. Populate the security structure with the instances
        let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
            &mut instances,
            None,
            &security_structure
        )?;

        // 4. Set the security structure as provisional, this will be extracted on transaction analysis
        let mut entity = entity;
        entity.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: security_structure_of_factor_instances.clone(),
            },
        );

        self.update_entities_erased(vec![entity.clone()].into())
            .await?;
        for factor_source_id in instances.keys() {
            self.update_last_used_of_factor_source(*factor_source_id)
                .await?
        }

        // 5. Create manifest
        TransactionManifest::apply_security_shield_for_unsecurified_entity(
            AnyUnsecurifiedEntity::with_unsecured_entity_control(
                entity.clone(),
                entity.entity_security_state().into_unsecured().unwrap(),
            ),
            security_structure_of_factor_instances,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[actix_rt::test]
    async fn setup_security_shield_populates_provisional_and_updates_factors() {
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            None::<Arc<dyn KeyDerivationInteractor>>,
            false,
        )
        .await;

        let bdfs_device = os.bdfs();
        let bdfs_hash = bdfs_device.id;
        let bdfs_id: FactorSourceID = bdfs_hash.into();

        let ledger_factor = FactorSource::sample_ledger();
        os.with_timeout(|sut: &SargonOS| {
            sut.add_factor_source(ledger_factor.clone())
        })
        .await
        .unwrap();

        let rola_factor = FactorSource::sample_off_device();
        os.with_timeout(|sut: &SargonOS| {
            sut.add_factor_source(rola_factor.clone())
        })
        .await
        .unwrap();

        let address = Account::sample_mainnet().address();

        os.update_profile_with(|profile| {
            profile.networks.update_with(NetworkID::Mainnet, |network| {
                network.accounts = Accounts::just(Account::sample_mainnet());
            });
            Ok(())
        })
        .await
        .unwrap();

        let ledger_id = ledger_factor.factor_source_id();
        let rola_id = rola_factor.factor_source_id();
        let rola_hash = *rola_id.as_hash().unwrap();

        let mut matrix_builder = MatrixBuilder::new();
        matrix_builder
            .add_factor_source_to_primary_threshold(bdfs_id)
            .unwrap();
        matrix_builder
            .add_factor_source_to_primary_threshold(ledger_id)
            .unwrap();
        matrix_builder
            .add_factor_source_to_recovery_override(bdfs_id)
            .unwrap();
        matrix_builder
            .add_factor_source_to_confirmation_override(bdfs_id)
            .unwrap();

        assert_eq!(
            matrix_builder.get_primary_threshold_factors(),
            &vec![bdfs_id, ledger_id]
        );
        assert!(matrix_builder.get_primary_override_factors().is_empty());
        assert_eq!(
            matrix_builder.get_recovery_factors(),
            &vec![bdfs_id]
        );
        assert_eq!(
            matrix_builder.get_confirmation_factors(),
            &vec![bdfs_id]
        );

        let matrix_ids = matrix_builder.build().unwrap();

        let profile_after_additions = os.profile().unwrap();
        let matrix = MatrixOfFactorSources::new(
            matrix_ids,
            &profile_after_additions.factor_sources,
        )
        .unwrap();

        let rola_from_profile = profile_after_additions
            .factor_sources
            .iter()
            .find(|factor| factor.factor_source_id() == rola_id)
            .unwrap()
            .clone();

        let security_structure = SecurityStructureOfFactorSources::new(
            DisplayName::new("Main Shield").unwrap(),
            matrix,
            rola_from_profile,
        );

        let manifest = os
            .make_setup_security_shield_manifest(
                security_structure.clone(),
                AddressOfAccountOrPersona::from(address),
            )
            .await
            .unwrap();

        assert!(!manifest.instructions.instructions().is_empty());

        let updated_entity = os
            .profile()
            .unwrap()
            .entity_by_address(AddressOfAccountOrPersona::from(address))
            .unwrap();
        let provisional = updated_entity.get_provisional();
        assert!(provisional.is_some());
        let provisional = provisional.unwrap();
        let derived = provisional.as_factor_instances_derived().unwrap();

        let expected_tx_ids: HashSet<FactorSourceID> =
            HashSet::from([bdfs_id, ledger_id]);

        assert!(derived
            .unique_tx_signing_factor_instances()
            .into_iter()
            .all(
                |instance| expected_tx_ids.contains(&instance.factor_source_id)
            ));

        assert_eq!(
            derived
                .authentication_signing_factor_instance
                .factor_source_id,
            rola_hash
        );

        let updated_factor_sources = os.profile().unwrap().factor_sources;
        for expected_id in [bdfs_id, ledger_id, rola_id] {
            assert!(updated_factor_sources
                .iter()
                .any(|factor| factor.id() == expected_id));
        }
    }

    #[actix_rt::test]
    async fn setup_security_shield_propagates_derivation_failure() {
        let failing_interactor: Arc<dyn KeyDerivationInteractor> =
            Arc::new(TestDerivationInteractor::fail());
        let os = SargonOS::fast_boot_bdfs_and_interactor(
            None::<MnemonicWithPassphrase>,
            Some(failing_interactor),
            false,
        )
        .await;

        let bdfs_factor: FactorSource = os.bdfs().into();
        let address = Account::sample_mainnet().address();

        os.update_profile_with(|profile| {
            profile.networks.update_with(NetworkID::Mainnet, |network| {
                network.accounts = Accounts::just(Account::sample_mainnet());
            });
            Ok(())
        })
        .await
        .unwrap();

        let mut matrix_builder = MatrixBuilder::new();
        matrix_builder
            .add_factor_source_to_primary_threshold(
                bdfs_factor.factor_source_id(),
            )
            .unwrap();
        matrix_builder
            .add_factor_source_to_recovery_override(
                bdfs_factor.factor_source_id(),
            )
            .unwrap();
        matrix_builder
            .add_factor_source_to_confirmation_override(
                bdfs_factor.factor_source_id(),
            )
            .unwrap();
        let matrix_ids = matrix_builder.build().unwrap();

        let security_structure = SecurityStructureOfFactorSources::new(
            DisplayName::new("Failing Shield").unwrap(),
            MatrixOfFactorSources::new(
                matrix_ids,
                &os.profile().unwrap().factor_sources,
            )
            .unwrap(),
            bdfs_factor.clone(),
        );

        let result = os
            .make_setup_security_shield_manifest(
                security_structure,
                AddressOfAccountOrPersona::from(address),
            )
            .await;

        assert!(result.is_err());

        let profile_after = os.profile().unwrap();
        let entity_after = profile_after
            .entity_by_address(AddressOfAccountOrPersona::from(address))
            .unwrap();
        assert!(entity_after.get_provisional().is_none());
    }
}
