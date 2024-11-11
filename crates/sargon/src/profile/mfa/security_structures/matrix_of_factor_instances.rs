use crate::prelude::*;

/* impl RoleWithFactors {
    pub fn fulfilling_matrix_of_factor_sources_with_instances(
    role: RoleKind,
        instances: &mut IndexMap<FactorSourceIDFromHash, FactorInstances>,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self> {
        let mut get_factors =
            |required: Vec<FactorSource>| -> Result<Vec<HierarchicalDeterministicFactorInstance>> {
                required
                    .iter()
                    .map(|f| {
                        if let Some(existing) = instances
                        .get_mut(&f.id_from_hash()) {
                            assert!(!existing.is_empty());
                            let instance = existing.shift_remove_index(0);
                            Ok(instance)
                        } else {
                            Err(CommonError::MissingFactorMappingInstancesIntoRole)
                        }
                        })
                    .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>>>()
            };

        let threshold_factors = get_factors(matrix_of_factor_sources.threshold_factors)?;
        let override_factors = get_factors(matrix_of_factor_sources.override_factors)?;

        Self::new(
            threshold_factors,
            matrix_of_factor_sources.threshold,
            override_factors,
        )
    }
}
    */
