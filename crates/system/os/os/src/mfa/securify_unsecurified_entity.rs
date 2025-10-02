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
