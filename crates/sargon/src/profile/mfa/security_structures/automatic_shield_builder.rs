use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    remaining_available_factors: IndexSet<FactorSource>,
    picked_primary_role_factors: IndexSet<FactorSourceID>,
    shield_builder: SecurityShieldBuilder,
}

impl AutomaticShieldBuilder {
    fn find_primary_role_candidates(
        all: &IndexSet<FactorSource>,
        shield_builder: &SecurityShieldBuilder,
    ) -> IndexSet<FactorSource> {
        let factor_source_ids =
            all.iter().map(|f| f.id()).collect::<IndexSet<_>>();
        shield_builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(factor_source_ids.into_iter().collect_vec()).into_iter().filter(|vs| match vs.validation {
            Ok(_) => true,
            Err(RoleBuilderValidation::NotYetValid(_)) => true,
            Err(RoleBuilderValidation::BasicViolation(_)) |  Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
        }).filter_map(|vs| all.iter().find(|f| f.id() == vs.factor_source_id))
        .cloned()
        .collect::<IndexSet<_>>()
    }

    fn factors_of_category(
        &self,
        category: FactorSourceCategory,
    ) -> IndexSet<FactorSource> {
        self.remaining_available_factors
            .iter()
            .filter(|f| f.category() == category)
            .sorted_by_key(|&f| f.common_properties().last_used_on)
            .cloned()
            .collect::<IndexSet<_>>()
    }

    fn consume(&mut self, factor: FactorSourceID) {
        self.remaining_available_factors
            .retain(|f| f.id() != factor);
    }

    fn consume_factor_and_add_to(
        &mut self,
        factor: FactorSourceID,
        add_to: &mut IndexSet<FactorSourceID>,
    ) {
        let was_inserted = add_to.insert(factor);
        assert!(was_inserted);
        self.consume(factor);
    }

    fn add_quantified_factors_of_categories_to_set_if_able(
        &mut self,
        categories: &[FactorSourceCategory],
        quantity_limit_per_category: Option<usize>,
        to: &mut IndexSet<FactorSourceID>,
    ) -> Result<()> {
        for category in categories.into_iter() {
            let factors_of_category = self.factors_of_category(*category);

            let quantified_factors = if let Some(quantity_limit_per_category) =
                quantity_limit_per_category
            {
                if factors_of_category.len() < quantity_limit_per_category {
                    return Err(CommonError::AutomaticShieldBuildingFailure {
                        underlying: format!(
                            "Not enough factors of category {:?}",
                            category
                        ),
                    });
                }

                Ok(factors_of_category
                    .iter()
                    .take(quantity_limit_per_category)
                    .cloned()
                    .collect::<IndexSet<_>>())
            } else {
                Ok(factors_of_category)
            }?;

            quantified_factors.into_iter().for_each(|factor| {
                self.consume_factor_and_add_to(factor.id(), to);
            });
        }

        Ok(())
    }

    fn add_quantified_custodian_and_hardware_factors_to_set_if_able(
        &mut self,
        quantity_limit_per_category: Option<usize>,
        to: &mut IndexSet<FactorSourceID>,
    ) -> Result<()> {
        self.add_quantified_factors_of_categories_to_set_if_able(
            &[
                FactorSourceCategory::Custodian,
                FactorSourceCategory::Hardware,
            ],
            quantity_limit_per_category,
            to,
        )
    }

    fn add_one_custodian_and_hardware_factor_to_set_if_able(
        &mut self,
        to: &mut IndexSet<FactorSourceID>,
    ) -> Result<()> {
        self.add_quantified_custodian_and_hardware_factors_to_set_if_able(
            Some(1),
            to,
        )
    }

    fn assign_recovery_factors_to(
        &mut self,
        factors: &mut IndexSet<FactorSourceID>,
    ) -> Result<()> {
        self.add_one_custodian_and_hardware_factor_to_set_if_able(factors)
    }

    fn assign_confirmation_factors_to(
        &mut self,
        factors: &mut IndexSet<FactorSourceID>,
    ) -> Result<()> {
        self.add_one_custodian_and_hardware_factor_to_set_if_able(factors)
    }

    fn add_factors_to_role(
        &self,
        factors: &IndexSet<FactorSourceID>,
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

        let mut recovery_factors = self
            .factors_of_category(FactorSourceCategory::Contact)
            .iter()
            .map(|f| f.id())
            .collect::<IndexSet<_>>();

        let mut confirmation_factors = self
            .factors_of_category(FactorSourceCategory::Information)
            .iter()
            .map(|f| f.id())
            .collect::<IndexSet<_>>();

        self.assign_recovery_factors_to(&mut recovery_factors)?;
        self.assign_confirmation_factors_to(&mut confirmation_factors)?;
        if recovery_factors.len() < 1 {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "No recovery factors available".to_string(),
            });
        }
        if confirmation_factors.len() < 1 {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "No confirmation factors available".to_string(),
            });
        }
        self.assign_recovery_factors_to(&mut recovery_factors)?;
        self.assign_confirmation_factors_to(&mut confirmation_factors)?;
        if recovery_factors.len() < 2 {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Not enough recovery factors available".to_string(),
            });
        }
        if confirmation_factors.len() < 2 {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Not enough confirmation factors available"
                    .to_string(),
            });
        }

        self.add_quantified_custodian_and_hardware_factors_to_set_if_able(
            None,
            &mut recovery_factors,
        )?;

        self.add_factors_to_role(&recovery_factors, RoleKind::Recovery);
        self.add_factors_to_role(&confirmation_factors, RoleKind::Confirmation);

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
        factor_source_ids: &IndexSet<FactorSourceID>,
    ) -> SecurityShieldPrerequisitesStatus {
        let count_excluding_identity = factor_source_ids
            .iter()
            .filter(|f| f.category() != FactorSourceCategory::Identity)
            .count();
        let count_hardware = factor_source_ids
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
        all_factors: IndexSet<FactorSource>,
        pick_primary_role_factors: impl Fn(IndexSet<FactorSource>) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = IndexSet<FactorSourceID>>,
    {
        if !SecurityShieldBuilder::prerequisites_status(
            &all_factors.iter().map(|f| f.id()).collect(),
        )
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
