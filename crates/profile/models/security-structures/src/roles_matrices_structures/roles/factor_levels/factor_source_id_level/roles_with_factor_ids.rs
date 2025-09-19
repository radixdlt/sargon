use crate::prelude::*;

pub type RoleWithFactorSourceIds<const ROLE: u8> =
    AbstractBuiltRoleWithFactor<ROLE, FactorSourceID>;

impl<const ROLE: u8> From<RoleWithFactorInstances<ROLE>>
    for RoleWithFactorSourceIds<ROLE>
{
    fn from(value: RoleWithFactorInstances<ROLE>) -> Self {
        RoleWithFactorSourceIds::with_factors_and_threshold(
            value.get_threshold(),
            value
                .get_threshold_factors()
                .iter()
                .map(|f| f.factor_source_id),
            value
                .get_override_factors()
                .iter()
                .map(|f| f.factor_source_id),
        )
    }
}
