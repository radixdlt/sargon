use crate::prelude::*;

impl SargonOS {
    pub async fn make_transaction_manifest_for_securified_entity(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        entity: AccountOrPersona,
        entity_sec_structure: SecurityStructureOfFactorInstances,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;

        // Read the current configuration
        let matrix_of_factor_source_ids =
            SecurityStructureOfFactorSourceIds::from(
                entity_sec_structure.clone(),
            );

        let current_factor_sources: SecurityStructureOfFactorSources = (
            &matrix_of_factor_source_ids,
            &profile_snapshot.factor_sources,
        )
            .try_into()?;

        // Derive new instance for rola only if it changed
        let new_rola_factor = if security_structure
            .authentication_signing_factor
            != current_factor_sources.authentication_signing_factor
        {
            Some(security_structure.authentication_signing_factor.clone())
        } else {
            None
        };

        // Get the newly added factors
        let new_factors: HashSet<&FactorSource> = security_structure
            .matrix_of_factors
            .all_factors()
            .difference(&current_factor_sources.matrix_of_factors.all_factors())
            .copied()
            .collect();

        // Create instances for new factors
        let mut factor_instances = self
            .derive_factor_instances(
                entity.clone(),
                new_factors,
                new_rola_factor,
            )
            .await?;

        // Merge kept factor instances with new factor instances.

        let new_factor_source_ids = SecurityStructureOfFactorSourceIds::from(
            security_structure.clone(),
        );
        let kept_instances = entity_sec_structure
            .unique_all_factor_instances()
            .into_iter()
            .filter(|instance| {
                new_factor_source_ids
                    .all_factors()
                    .contains(&instance.factor_source_id)
            })
            .collect::<IndexSet<FactorInstance>>();

        for instance in kept_instances {
            factor_instances.append_or_insert_element_to(
                // For now we do work only with factor source which are derived from factor as a hash
                instance.factor_source_id.into_hash().unwrap(),
                // For now we do work only with hd factor instances
                instance.try_as_hd_factor_instances().unwrap(),
            );
        }

        // Create the updated security structure of instances
        let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
            &mut factor_instances,
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
        for factor_source_id in factor_instances.keys() {
            self.update_last_used_of_factor_source(*factor_source_id)
                .await?
        }

        // Create the transaction manifest next for R+T.
        // Hardcoded for now to unblock the work for implementing Timed Recovery flows.
        let role_combination = RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion;

        let securified_entity_control =
            entity.security_state().into_securified().unwrap();
        let ac_address =
            securified_entity_control.access_controller_address;

        // Fetch the current Access Controller state to check for existing recovery attempts
        let ac_state_details = self
            .access_controller_state_repository_client
            .get_cached_access_controller_details(&ac_address)
            .await?;

        let mut manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                AnySecurifiedEntity::with_securified_entity_control(
                    entity.clone(),
                    securified_entity_control,
                ),
                security_structure_of_factor_instances,
                role_combination,
            );

        // Cancel any existing recovery proposal before initiating a new one
        manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &ac_state_details,
            role_combination,
        );

        Ok(manifest)
    }
}
