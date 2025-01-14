use crate::prelude::*;

pub(crate) type RoleWithFactorInstances<const ROLE: u8> =
    AbstractBuiltRoleWithFactor<ROLE, FactorInstance>;

impl<const ROLE: u8> RoleWithFactorInstances<ROLE> {
    pub(crate) fn fulfilling_role_of_factor_sources_with_factor_instances(
        consuming_instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
        matrix_of_factor_sources: &MatrixOfFactorSources,
    ) -> Result<Self, CommonError> {
        let role_kind = RoleKind::from_u8(ROLE).unwrap();

        let role_of_sources = matrix_of_factor_sources.get_role::<ROLE>();
        assert_eq!(role_of_sources.role(), role_kind);
        let threshold: Threshold = role_of_sources.get_threshold();

        // Threshold factors
        let threshold_factors =
            Self::try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
                consuming_instances,
                role_of_sources.get_threshold_factors(),
            )?;

        // Override factors
        let override_factors =
            Self::try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
                consuming_instances,
                role_of_sources.get_override_factors(),
            )?;

        let role_with_instances = Self::with_factors_and_threshold(
            threshold,
            threshold_factors,
            override_factors,
        );

        assert_eq!(role_with_instances.role(), role_kind);
        Ok(role_with_instances)
    }

    fn try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
        instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
        from: &[FactorSource],
    ) -> Result<Vec<FactorInstance>, CommonError> {
        from.iter()
            .map(|f| {
                if let Some(existing) = instances.get(&f.id_from_hash()) {
                    let hd_instance =
                        existing.first_transaction_signing().ok_or(
                            CommonError::MissingFactorMappingInstancesIntoRole,
                        )?;
                    let instance = FactorInstance::from(hd_instance);
                    Ok(instance)
                } else {
                    Err(CommonError::MissingFactorMappingInstancesIntoRole)
                }
            })
            .collect::<Result<Vec<FactorInstance>, CommonError>>()
    }
}
