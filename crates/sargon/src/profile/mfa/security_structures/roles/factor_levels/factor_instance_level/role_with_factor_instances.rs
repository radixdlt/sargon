use crate::prelude::*;

pub(crate) type RoleWithFactorInstances<const R: u8> =
    AbstractBuiltRoleWithFactor<R, FactorInstance>;

impl<const R: u8> RoleWithFactorSources<R> {
    fn from<const F: u8>(other: &RoleWithFactorSources<F>) -> Self {
        Self::with_factors(
            other.get_threshold(),
            other.get_threshold_factors().clone(),
            other.get_override_factors().clone(),
        )
    }
}

impl MatrixOfFactorSources {
    pub(crate) fn get_role<const R: u8>(&self) -> RoleWithFactorSources<R> {
        match R {
            ROLE_PRIMARY => RoleWithFactorSources::from(&self.primary_role),
            ROLE_RECOVERY => RoleWithFactorSources::from(&self.recovery_role),
            ROLE_CONFIRMATION => {
                RoleWithFactorSources::from(&self.confirmation_role)
            }
            _ => panic!("unknown"),
        }
    }
}

impl<const R: u8> RoleWithFactorInstances<R> {
    pub(crate) fn fulfilling_role_of_factor_sources_with_factor_instances(
        consuming_instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
        matrix_of_factor_sources: &MatrixOfFactorSources,
    ) -> Result<Self, CommonError> {
        let role_kind = RoleKind::from_u8(R).unwrap();

        let role_of_sources = matrix_of_factor_sources.get_role::<R>();
        assert_eq!(role_of_sources.role(), role_kind);
        let threshold: u8 = role_of_sources.get_threshold();

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

        let role_with_instances =
            Self::with_factors(threshold, threshold_factors, override_factors);

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
                    let hd_instance = existing.first().ok_or(
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
