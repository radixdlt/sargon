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
            SecurityStructureOfFactorSourceIds::from(entity_sec_structure.clone());

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
            .derive_factor_instances(entity.clone(), new_factors, new_rola_factor)
            .await?;

        // Merge kept factor instances with new factor instances.

        let new_factor_source_ids = SecurityStructureOfFactorSourceIds::from(security_structure.clone());
        let kept_instances = entity_sec_structure
        .unique_all_factor_instances()
        .into_iter()
        .filter( |instance| {
            new_factor_source_ids.all_factors().contains(&instance.factor_source_id)
        })
        .collect::<IndexSet<FactorInstance>>();

        for instance in kept_instances {
            factor_instances.append_or_insert_element_to(
                // For now we do work only with factor source which are derived from factor as a hash
                instance.factor_source_id.into_hash().unwrap(),
                // For now we do work only with hd factor instances
                instance.try_as_hd_factor_instances().unwrap()
            );
        }

        // Create the updated security structure of instances
        let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
            &mut factor_instances,
            None,
            &security_structure
        )?;

        // Create the transaction manifest next for R+T.
        // Hardcoded for now to unblock the work for implementing Timed Recovery flows.
        Ok(
            TransactionManifest::apply_security_shield_for_securified_entity(
                AnySecurifiedEntity::with_securified_entity_control(entity.clone(), entity.security_state().into_securified().unwrap()),
                security_structure_of_factor_instances,
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion
            )
        )
    }
}
