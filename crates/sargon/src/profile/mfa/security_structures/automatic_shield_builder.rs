use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    remaining_available_factors: Vec<FactorSource>,
    picked_primary_role_factors: Vec<FactorSourceID>,
    shield_builder: SecurityShieldBuilder,
}

impl AutomaticShieldBuilder {
    fn find_primary_role_candidates(
        all: &[FactorSource],
        shield_builder: &SecurityShieldBuilder,
    ) -> Vec<FactorSource> {
        let factor_source_ids = all.iter().map(|f| f.id()).collect_vec();
        shield_builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(factor_source_ids).into_iter().filter(|vs| match vs.validation {
            Ok(_) => true,
            Err(RoleBuilderValidation::NotYetValid(_)) => true,
            Err(RoleBuilderValidation::BasicViolation(_)) |  Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
        }).filter_map(|vs| all.iter().find(|f| f.id() == vs.factor_source_id))
        .cloned()
        .collect_vec()
    }

    fn factors_of_category(
        &self,
        category: FactorSourceCategory,
    ) -> Vec<FactorSource> {
        self.remaining_available_factors
            .iter()
            .filter(|f| f.category() == category)
            .sorted_by_key(|&f| f.common_properties().last_used_on)
            .cloned()
            .collect_vec()
    }

    fn recovery_role_factors(&mut self) -> Result<Vec<FactorSourceID>> {
        let contact_factors =
            self.factors_of_category(FactorSourceCategory::Contact);
        let hardware_factors =
            self.factors_of_category(FactorSourceCategory::Hardware);

        todo!()
    }

    fn confirmation_role_factors(&mut self) -> Result<Vec<FactorSourceID>> {
        let information_factors =
            self.factors_of_category(FactorSourceCategory::Information);

        Ok(vec![])
    }

    fn add_factors_to_role(
        &self,
        factors: &Vec<FactorSourceID>,
        role: RoleKind,
    ) {
        factors.into_iter().for_each(|&f| match role {
            RoleKind::Primary => {
                self.shield_builder
                    .add_factor_source_to_primary_threshold(f);
            }
            RoleKind::Recovery => {
                self.shield_builder
                    .add_factor_source_to_recovery_override(f);
            }
            RoleKind::Confirmation => {
                self.shield_builder
                    .add_factor_source_to_confirmation_override(f);
            }
        });
    }

    fn _build_shield(&mut self) -> Result<SecurityStructureOfFactorSourceIDs> {
        if self.picked_primary_role_factors.len() == 1 {
            // if the user chose only 1 that factor cannot be used in the recovery or confirmation roles
            self.remaining_available_factors
                .retain(|f| f.id() != self.picked_primary_role_factors[0]);
        }
        self.add_factors_to_role(
            &self.picked_primary_role_factors,
            RoleKind::Primary,
        );
        self.shield_builder
            .set_threshold(self.picked_primary_role_factors.len() as u8);

        let recovery_factors = &self.recovery_role_factors()?;
        self.add_factors_to_role(recovery_factors, RoleKind::Recovery);

        let confirmation_factors = &self.confirmation_role_factors()?;
        self.add_factors_to_role(confirmation_factors, RoleKind::Confirmation);

        self.shield_builder.build().map_err(|e| {
            CommonError::AutomaticShieldBuildingFailure {
                underlying: format!("{:?}", e),
            }
        })
    }

    fn build_shield(self) -> Result<SecurityStructureOfFactorSourceIDs> {
        let mut _self = self;
        _self._build_shield()
    }
}

impl SecurityShieldBuilder {
    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    pub fn prerequisites_status(
        factor_sources: &[FactorSource],
    ) -> SecurityShieldPrerequisitesStatus {
        let count_excluding_identity = factor_sources
            .iter()
            .filter(|f| f.category() != FactorSourceCategory::Identity)
            .count();
        let count_hardware = factor_sources
            .iter()
            .filter(|f| f.category() == FactorSourceCategory::Hardware)
            .count();
        if count_hardware < 1 {
            SecurityShieldPrerequisitesStatus::HardwareRequired
        } else if count_excluding_identity < 2 {
            SecurityShieldPrerequisitesStatus::AnyRequired
        } else {
            SecurityShieldPrerequisitesStatus::Sufficient
        }
    }
}

impl AutomaticShieldBuilder {
    pub async fn build<Fut>(
        all_factors: Vec<FactorSource>,
        pick_primary_role_factors: impl Fn(Vec<FactorSource>) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = Vec<FactorSourceID>>,
    {
        if !SecurityShieldBuilder::prerequisites_status(&all_factors)
            .is_sufficient()
        {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Prerequisites not met".to_string(),
            });
        }
        let security_shield_builder = SecurityShieldBuilder::new();
        let candidates = Self::find_primary_role_candidates(
            &all_factors,
            &security_shield_builder,
        );
        let picked = pick_primary_role_factors(candidates).await;
        let auto_builder = Self {
            remaining_available_factors: all_factors.into_iter().collect(),
            picked_primary_role_factors: picked,
            shield_builder: security_shield_builder,
        };
        auto_builder.build_shield()
    }
}
