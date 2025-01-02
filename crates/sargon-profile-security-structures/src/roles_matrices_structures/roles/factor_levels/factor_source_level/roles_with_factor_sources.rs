use crate::prelude::*;

pub(crate) type RoleWithFactorSources<const ROLE: u8> =
    AbstractBuiltRoleWithFactor<ROLE, FactorSource>;

impl<const ROLE: u8> RoleWithFactorSources<ROLE> {
    fn from<const ROLE_FROM: u8>(
        other: &RoleWithFactorSources<ROLE_FROM>,
    ) -> Self {
        Self::with_factors(
            other.get_threshold(),
            other.get_threshold_factors().clone(),
            other.get_override_factors().clone(),
        )
    }
}

impl MatrixOfFactorSources {
    pub(crate) fn get_role<const ROLE: u8>(
        &self,
    ) -> RoleWithFactorSources<ROLE> {
        match ROLE {
            ROLE_PRIMARY => {
                RoleWithFactorSources::<ROLE>::from(&self.primary_role)
            }
            ROLE_RECOVERY => {
                RoleWithFactorSources::<ROLE>::from(&self.recovery_role)
            }
            ROLE_CONFIRMATION => {
                RoleWithFactorSources::<ROLE>::from(&self.confirmation_role)
            }
            _ => panic!("unknown"),
        }
    }
}

impl<const ROLE: u8> RoleWithFactorSources<ROLE> {
    pub fn new(
        role_with_factor_source_ids: RoleWithFactorSourceIds<ROLE>,
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
