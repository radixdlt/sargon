use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    remaining_available_factors: IndexSet<FactorSource>,

    proto_matrix: ProtoMatrix,
}

use serde_json::value::Index;
use FactorSourceCategory::*;
use RoleKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumAsInner)]
enum Quantity {
    All,
    Fixed(usize),
}
impl Quantity {
    fn one() -> Self {
        Self::Fixed(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FactorSelector {
    Category(FactorSourceCategory),
    Kind(FactorSourceKind),
}

impl SecurityShieldBuilder {
    pub fn preselect_using_currently_selected_primary_factors(
        &self,
        all_factors_in_profile: IndexSet<FactorSource>,
    ) -> Result<()> {
        if !self.get_primary_override_factors().is_empty() {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Primary override factors not allowed when preselecting factors for Recovery and Confirmation".to_string(),
            });
        }

        let primary_factors = self
            .get_primary_threshold_factors()
            .into_iter()
            .collect::<IndexSet<_>>();

        if primary_factors
            .intersection(
                &all_factors_in_profile
                    .iter()
                    .map(|f| f.id())
                    .collect::<IndexSet<_>>(),
            )
            .cloned()
            .collect::<IndexSet<FactorSourceID>>()
            != primary_factors
        {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Primary factors not in profile".to_string(),
            });
        }

        if !Self::prerequisites_status(
            &all_factors_in_profile.iter().map(|f| f.id()).collect(),
        )
        .is_sufficient()
        {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Prerequisites not met".to_string(),
            });
        }

        let mut auto_builder = AutomaticShieldBuilder::new(
            all_factors_in_profile,
            primary_factors,
        );

        let proto_matrix = auto_builder.assign()?;

        assert_eq!(
            proto_matrix.primary.clone().into_iter().collect_vec(),
            self.get_primary_threshold_factors(),
            "Auto assignment should not have changed the primary factors"
        );
        self.set_state(proto_matrix);

        if let Some(invalid_reason) = self.validate() {
            Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: invalid_reason.to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn set_state(&self, proto_matrix: ProtoMatrix) {
        self.reset_factors_in_roles();
        self.set_threshold(proto_matrix.primary.len() as u8);
        proto_matrix.primary.into_iter().for_each(|f| {
            self.add_factor_source_to_primary_threshold(f);
        });
        proto_matrix.recovery.into_iter().for_each(|f| {
            self.add_factor_source_to_recovery_override(f);
        });
        proto_matrix.confirmation.into_iter().for_each(|f| {
            self.add_factor_source_to_confirmation_override(f);
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProtoMatrix {
    primary: IndexSet<FactorSourceID>,
    recovery: IndexSet<FactorSourceID>,
    confirmation: IndexSet<FactorSourceID>,
}
impl ProtoMatrix {
    fn new(primary: IndexSet<FactorSourceID>) -> Self {
        Self {
            primary,
            recovery: IndexSet::new(),
            confirmation: IndexSet::new(),
        }
    }

    fn factors_for_role(&self, role: RoleKind) -> &IndexSet<FactorSourceID> {
        match role {
            Primary => &self.primary,
            Recovery => &self.recovery,
            Confirmation => &self.confirmation,
        }
    }

    fn add_factors_for_role(
        &mut self,
        role: RoleKind,
        factors: IndexSet<FactorSourceID>,
    ) {
        match role {
            Primary => self.primary.extend(factors),
            Recovery => self.recovery.extend(factors),
            Confirmation => self.confirmation.extend(factors),
        }
    }
}

impl AutomaticShieldBuilder {
    fn new(
        available_factors: IndexSet<FactorSource>,
        primary: IndexSet<FactorSourceID>,
    ) -> Self {
        Self {
            remaining_available_factors: available_factors,
            proto_matrix: ProtoMatrix::new(primary),
        }
    }

    fn assign_factors_matching_selector(
        &mut self,
        to: RoleKind,
        selector: FactorSelector,
        quantity_to_add: Quantity,
    ) {
        let target_role = to;

        let mut factors_to_add = self
            .remaining_available_factors
            .iter()
            .filter(|&f| match selector {
                FactorSelector::Category(category) => f.category() == category,
                FactorSelector::Kind(kind) => f.factor_source_kind() == kind,
            })
            .map(|f| f.id())
            .collect::<IndexSet<_>>();

        if let Some(quantity) = quantity_to_add.as_fixed() {
            factors_to_add = factors_to_add
                .into_iter()
                .take(*quantity)
                .collect::<IndexSet<_>>();
        }

        self.remaining_available_factors
            .retain(|f| !factors_to_add.contains(&f.id()));

        self.proto_matrix
            .add_factors_for_role(target_role, factors_to_add);
    }

    fn factors_for_role(&self, role: RoleKind) -> &IndexSet<FactorSourceID> {
        self.proto_matrix.factors_for_role(role)
    }

    fn assign_factors_of_category(
        &mut self,
        to: RoleKind,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) {
        self.assign_factors_matching_selector(
            to,
            FactorSelector::Category(category),
            quantity_to_add,
        )
    }

    fn assign_factors_of_kind(
        &mut self,
        to: RoleKind,
        kind: FactorSourceKind,
        quantity_to_add: Quantity,
    ) {
        self.assign_factors_matching_selector(
            to,
            FactorSelector::Kind(kind),
            quantity_to_add,
        )
    }

    fn assign_one_hardware_factor(&mut self, to: RoleKind) {
        self.assign_factors_of_category(
            to,
            FactorSourceCategory::Hardware,
            Quantity::one(),
        )
    }

    fn count_factors_for_role(&self, role_kind: RoleKind) -> u8 {
        self.factors_for_role(role_kind).len() as u8
    }

    fn assign_factors_of_category_to_recovery(
        &mut self,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) {
        self.assign_factors_of_category(Recovery, category, quantity_to_add)
    }

    fn assign_factors_of_category_to_confirmation(
        &mut self,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) {
        self.assign_factors_of_category(Confirmation, category, quantity_to_add)
    }

    /// Automatic assignment of factors to roles according to [this heuristics][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Automatic-Security-Shield-Construction
    fn assign(&mut self) -> Result<ProtoMatrix> {
        // 📒 "If the user only chose 1 factor for PRIMARY, remove that factor from the list (it cannot be used elsewhere - otherwise it can)."
        {
            if self.count_factors_for_role(Primary) == 1
                && let Some(only_primary_factor) =
                    self.proto_matrix.primary.iter().next()
            {
                self.remaining_available_factors
                    .retain(|f| f.id() != *only_primary_factor);
            }
        }

        // 📒 "Drop in the somewhat “special-use” factors first"
        {
            // 📒	"Add all Contact factors in the list to RECOVERY."
            self.assign_factors_of_category_to_recovery(Contact, Quantity::All);

            // 📒	"Add all Information factors in the list to CONFIRMATION."
            self.assign_factors_of_category_to_confirmation(
                Information,
                Quantity::All,
            );
        }

        let mut distribute_custodian_and_hardware_to_non_primary = || {
            // 📒 "Add any Custodian.." 🙅‍♀️  Custodian FactorSources does not exist yet...

            // 📒 "Add any Hardware factors in the list, starting with the most recently used, to RECOVERY
            // until there is at least _ factor source in RECOVERY."
            self.assign_one_hardware_factor(Recovery);

            // 📒 "Add any Hardware (Ledger, Arculus, Yubikey) factors in the list, starting with the most recently used, to CONFIRMATION until there is at least _ factor factors in CONFIRMATION."
            self.assign_one_hardware_factor(Confirmation);
        };

        // 📒 "Distribute to try to get at least 1 RECOVERY and then 1 CONFIRMATION" [if possible]
        distribute_custodian_and_hardware_to_non_primary();

        // 📒 "Distribute to try to get up to 2 RECOVERY and then 2 CONFIRMATION factors if possible"
        distribute_custodian_and_hardware_to_non_primary();

        // 📒 "Fill in any remaining other factors to increase reliability of being able to recover"
        {
            // 📒 "Add any (and all) remaining Hardware or Custodian factors in the list to RECOVERY."
            self.assign_factors_of_category_to_recovery(
                Hardware,
                Quantity::All,
            );

            // 📒 "Set all Biometrics/PIN factors to a role (they must be all in one role because they are unlocked by the same Biometrics/PIN check):"
            {
                self.assign_factors_of_kind(
                    if self.count_factors_for_role(Recovery)
                        > self.count_factors_for_role(Confirmation)
                    {
                        // 📒 "If there are more RECOVERY factors than CONFIRMATION factors, add any (and all) Biometrics/PIN factors to CONFIRMATION"
                        Confirmation
                    } else {
                        // 📒 "Else, add any (and all) Biometrics/PIN factors to RECOVERY."
                        Recovery
                    },
                    FactorSourceKind::Device,
                    Quantity::All,
                );
            }
        }

        Ok(self.proto_matrix.clone())
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_std::future::ready;
    use indexmap::IndexSet;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AutomaticShieldBuilder;

    impl SUT {
        fn test(
            all_factors_in_profile: IndexSet<FactorSource>,
            pick_primary_role_factors: IndexSet<FactorSourceID>,
        ) -> Result<SecurityStructureOfFactorSourceIDs> {
            let shield_builder = SecurityShieldBuilder::new();
            shield_builder.set_threshold(pick_primary_role_factors.len() as u8);
            pick_primary_role_factors.into_iter().for_each(|f| {
                shield_builder.add_factor_source_to_primary_threshold(f);
            });
            shield_builder.preselect_using_currently_selected_primary_factors(
                all_factors_in_profile,
            )?;

            shield_builder.build().map_err(|e| {
                CommonError::AutomaticShieldBuildingFailure {
                    underlying: format!("{:?}", e),
                }
            })
        }
    }

    #[test]
    fn empty_factors_is_err() {
        let res = SUT::test(IndexSet::new(), IndexSet::new());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[test]
    fn one_factors_is_not_enough_is_err() {
        let res = SUT::test(
            IndexSet::from_iter([FactorSource::sample_device()]),
            IndexSet::just(FactorSourceID::sample_device()),
        );

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[test]
    fn two_factors_is_not_enough_is_err() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_device(),
                FactorSource::sample_ledger(),
            ]),
            IndexSet::just(FactorSourceID::sample_device()),
        );

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[test]
    fn two_device_factor_source_and_one_ledger_is_not_sufficient() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_ledger(),
            ]),
            IndexSet::just(FactorSourceID::sample_device()),
        );

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[test]
    fn one_device_factor_source_and_two_ledger_is_ok_when_primary_uses_one_ledger(
    ) {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            IndexSet::just(FactorSource::sample_ledger().id()),
        );
        let matrix = res.unwrap().matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_ledger()],
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger_other()
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_device()
            ],)
        );
    }

    #[test]
    fn one_device_factor_source_and_two_ledger_is_ok_when_primary_uses_all() {
        let factors = IndexSet::from_iter([
            FactorSource::sample_device_babylon(),
            FactorSource::sample_ledger(),
            FactorSource::sample_ledger_other(),
        ]);

        let res = SUT::test(
            factors.clone(),
            factors.clone().into_iter().map(|f| f.id()).collect(),
        );

        let matrix = res.unwrap().matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                3,
                factors.clone().into_iter().map(|f| f.id()),
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_device(),
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger_other(),
            ],)
        );
    }
}
