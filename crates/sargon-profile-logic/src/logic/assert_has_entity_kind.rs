pub trait AssertHasEntityKind: HasFactorInstances {
    fn assert_has_entity_kind(
        &self,
        entity_kind_of_entity: CAP26EntityKind,
    ) -> Result<()> {
        let entity_kind_of_factor_instances =
            self.entity_kind_of_all_factors()?;

        if entity_kind_of_entity != entity_kind_of_factor_instances {
            return Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: entity_kind_of_entity.to_string(), entity_kind_of_factor_instances: entity_kind_of_factor_instances.to_string() });
        }

        Ok(())
    }

    fn entity_kind_of_all_factors(&self) -> Result<CAP26EntityKind> {
        let index_agnostic_path =
            self.index_agnostic_path_of_all_tx_signing_factor_instances()?;
        Ok(index_agnostic_path.entity_kind)
    }

    fn index_agnostic_path_of_all_tx_signing_factor_instances(
        &self,
    ) -> Result<IndexAgnosticPath> {
        let factors = self
            .unique_tx_signing_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
            .collect_vec();

        if factors.is_empty() {
            return Err(CommonError::NoTransactionSigningFactorInstance);
        }

        let index_agnostic_path =
            factors.first().unwrap().derivation_path().agnostic();

        if factors
            .iter()
            .any(|f| f.get_entity_kind() != index_agnostic_path.entity_kind)
        {
            return Err(CommonError::WrongEntityKindOfInFactorInstancesPath);
        }

        if factors
            .iter()
            .any(|f| f.get_key_kind() != CAP26KeyKind::TransactionSigning)
        {
            return Err(
                CommonError::WrongKeyKindOfTransactionSigningFactorInstance,
            );
        }

        Ok(index_agnostic_path)
    }

    /// Returns whether the entity is linked to the given factor source.
    fn is_linked_to_factor_source(&self, factor_source: FactorSource) -> bool {
        self.unique_all_factor_instances().iter().any(|factor| {
            factor.factor_source_id == factor_source.factor_source_id()
        })
    }
}
