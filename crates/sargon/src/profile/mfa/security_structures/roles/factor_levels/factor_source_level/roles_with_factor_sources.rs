use crate::prelude::*;

pub(crate) type RoleWithFactorSources<const R: u8> =
    AbstractBuiltRoleWithFactor<R, FactorSource>;

impl<const R: u8> RoleWithFactorSources<R> {
    pub fn new(
        role_with_factor_source_ids: RoleWithFactorSourceIds<R>,
        factor_sources: &FactorSources,
    ) -> Result<Self, CommonError> {
        let lookup_f =
            |id: &FactorSourceID| -> Result<FactorSource, CommonError> {
                factor_sources
                    .get_id(id)
                    .ok_or(CommonError::FactorSourceDiscrepancy)
                    .cloned()
            };

        let lookup = |ids: &Vec<FactorSourceID>| -> Result<Vec<FactorSource>, CommonError> {
            ids.iter()
                .map(lookup_f)
                .collect::<Result<Vec<_>, CommonError>>()
        };

        let threshold_factors =
            lookup(role_with_factor_source_ids.get_threshold_factors())?;
        let override_factors =
            lookup(role_with_factor_source_ids.get_override_factors())?;

        Ok(Self::with_factors(
            role_with_factor_source_ids.get_threshold(),
            threshold_factors,
            override_factors,
        ))
    }
}
