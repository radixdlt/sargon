use crate::prelude::*;

use super::{
    proto_matrix::ProtoMatrix, quantity::Quantity,
    CallsToAssignUnsupportedFactor,
};

use FactorSourceCategory::*;
use RoleKind::*;

impl FactorSourceCategory {
    fn is_supported(&self) -> bool {
        match self {
            Identity | Hardware | Contact | Information => true,
            Custodian => false,
        }
    }
}

/// A crate internal helper builder which assigns factors to
/// Recovery and Confirmation roles based on the heuristic
/// laid out in ["Automatic Security Shield Construction" document][doc]
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Automatic-Security-Shield-Construction
pub(crate) struct AutomaticShieldBuilder {
    /// Only used for testing purposes, feel free to remove.
    stats_for_testing: AutoBuildOutcomeForTesting,

    /// While we assign factors to Recovery and Confirmation roles, we remove
    /// them from this set.
    remaining_available_factors: IndexSet<FactorSource>,

    /// The factors assigned to each role, including the factors originally
    /// set for the primary role.
    proto_matrix: ProtoMatrix,
}

impl SecurityShieldBuilder {
    /// Assigns the factors to the Recovery and Confirmation roles according to the heuristic
    /// laid out in ["Automatic Security Shield Construction" document][doc].
    ///
    /// The `all_factors_in_profile` should contain all factors that are available for the user,
    /// i.e. from Profile, and SHOULD contain the factors that the user has preselected for the
    /// Primary role.
    ///
    /// # Throws
    /// Throws if the primary role is invalid.
    /// Throws if the primary override factors are not empty.
    /// Throws if the primary factors are not in the profile.
    /// Throws if the prerequisites are not met.
    /// Throws if the shield is invalid after auto assignment - which should not happen, can be considered programmer error!
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Automatic-Security-Shield-Construction
    pub fn auto_assign_factors_to_recovery_and_confirmation_based_on_primary(
        &self,
        all_factors_in_profile: IndexSet<FactorSource>,
    ) -> Result<AutoBuildOutcomeForTesting>
/* Feel free to replace `AutoBuildOutcomeForTesting` return type if you need anything else, I had Unit, so might as well make testing easier by returning this type. */
    {
        if let Some(invalid_reason) = self.validate_primary_role() {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: format!(
                    "Primary role is not valid: {:?}",
                    invalid_reason
                ),
            });
        }

        if !self.get_primary_override_factors().is_empty() {
            // Can we update this auto assign heuristics to allow primary override factors?
            // If we would allow it, we would need to remove all those factors in override
            // from `all_factors_in_profile`.
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
            Ok(auto_builder.stats_for_testing)
        }
    }

    /// Updates the Primary, Recovery and Confirmation roles with the factors of the given `ProtoMatrix`.
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

impl AutomaticShieldBuilder {
    fn new(
        available_factors: IndexSet<FactorSource>,
        primary: IndexSet<FactorSourceID>,
    ) -> Self {
        Self {
            stats_for_testing: AutoBuildOutcomeForTesting::default(),
            remaining_available_factors: available_factors,
            proto_matrix: ProtoMatrix::new(primary),
        }
    }

    /// Returns `Some(n)` if any factor matching the selector was found where `n`
    /// is `<= quantity_to_add` and `None` if no factors matching the selector was.
    /// found. Guaranteed to never return `Some(0)`.
    fn assign_factors_matching_selector(
        &mut self,
        to: RoleKind,
        selector: FactorSelector,
        quantity_to_add: Quantity,
    ) -> Option<usize> {
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
                .take(quantity)
                .collect::<IndexSet<_>>();
        }

        let number_of_factors_added = factors_to_add.len();
        if number_of_factors_added == 0 {
            return None;
        }

        self.remaining_available_factors
            .retain(|f| !factors_to_add.contains(&f.id()));

        self.proto_matrix
            .add_factors_for_role(target_role, factors_to_add);

        Some(number_of_factors_added)
    }

    fn factors_for_role(&self, role: RoleKind) -> &IndexSet<FactorSourceID> {
        self.proto_matrix.factors_for_role(role)
    }

    /// Returns `true` if any factor was assigned, `false` otherwise.
    fn assign_factors_of_category(
        &mut self,
        to: RoleKind,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) -> bool {
        match self.assign_factors_matching_selector(
            to,
            FactorSelector::Category(category),
            quantity_to_add,
        ) {
            Some(0) | None => false,
            Some(_) => true,
        }
    }

    /// Returns `true` if any factor was assigned, `false` otherwise.
    fn assign_factors_of_kind(
        &mut self,
        to: RoleKind,
        kind: FactorSourceKind,
        quantity_to_add: Quantity,
    ) -> bool {
        match self.assign_factors_matching_selector(
            to,
            FactorSelector::Kind(kind),
            quantity_to_add,
        ) {
            Some(0) | None => false,
            Some(_) => true,
        }
    }

    fn count_factors_for_role(&self, role_kind: RoleKind) -> u8 {
        self.factors_for_role(role_kind).len() as u8
    }

    /// Returns `true` if any factor was assigned, `false` otherwise.
    fn assign_factors_of_category_to_recovery(
        &mut self,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) -> bool {
        self.assign_factors_of_category(Recovery, category, quantity_to_add)
    }

    /// Returns `true` if any factor was assigned, `false` otherwise.
    fn assign_factors_of_category_to_confirmation(
        &mut self,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) -> bool {
        self.assign_factors_of_category(Confirmation, category, quantity_to_add)
    }

    fn assign_factor_of_category_to_role_while_meaningful_and_less_than_limit(
        &mut self,
        category: FactorSourceCategory,
        limit: u8,
        to: RoleKind,
    ) {
        let role = to;

        loop {
            if self.count_factors_for_role(role) >= limit {
                // when `limit` reached, we stop.
                return;
            }

            if !category.is_supported() {
                self.stats_for_testing
                    .calls_to_assign_unsupported_factor
                    .push(CallsToAssignUnsupportedFactor {
                        role,
                        unsupported: FactorSelector::Category(category),
                        number_of_factors_for_role: self
                            .count_factors_for_role(role),
                        limit,
                    });
                return;
            }

            if !self.assign_factors_of_category(role, category, Quantity::One) {
                // We did not manage to assign any hardware factor, meaning we
                // it is meaningless to try to assign more factors of this category.
                return;
            }
        }
    }

    /// Calls `assign_factor_of_category_to_role_while_meaningful_and_less_than_limit`
    /// for both Custodian and Hardware categories.
    fn assign_custodian_and_hardware_to_role_while_meaningful_and_less_than_limit(
        &mut self,
        limit: u8,
        to: RoleKind,
    ) {
        self.assign_factor_of_category_to_role_while_meaningful_and_less_than_limit(
            Custodian,
            limit,
            to,
        );

        self.assign_factor_of_category_to_role_while_meaningful_and_less_than_limit(
            Hardware,
            limit,
            to,
        );
    }

    /// Calls `assign_custodian_and_hardware_to_role_while_meaningful_and_less_than_limit`
    /// for both Recovery and Confirmation roles.
    fn assign_custodian_and_hardware_to_non_primary_roles_while_less_than_limit_for_each_assignment(
        &mut self,
        limit: u8,
    ) {
        self.assign_custodian_and_hardware_to_role_while_meaningful_and_less_than_limit(
            limit, Recovery,
        );
        self.assign_custodian_and_hardware_to_role_while_meaningful_and_less_than_limit(
            limit,
            Confirmation,
        );
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
            // 📒 "Add all Contact factors in the list to RECOVERY."
            self.assign_factors_of_category_to_recovery(Contact, Quantity::All);

            // 📒	"Add all Information factors in the list to CONFIRMATION."
            self.assign_factors_of_category_to_confirmation(
                Information,
                Quantity::All,
            );
        }

        // 📒 Assign Custodian/Hardware factors to RECOVERY & CONFIRMATION
        // without exceeding limit of 1 factor in each role.
        self.assign_custodian_and_hardware_to_non_primary_roles_while_less_than_limit_for_each_assignment(1);

        // 📒 Assign Custodian/Hardware factors to RECOVERY & CONFIRMATION
        // without exceeding limit of 2 factor in each role.
        self.assign_custodian_and_hardware_to_non_primary_roles_while_less_than_limit_for_each_assignment(2);

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
        ) -> Result<(
            SecurityStructureOfFactorSourceIDs,
            AutoBuildOutcomeForTesting,
        )> {
            let shield_builder = SecurityShieldBuilder::new();
            shield_builder.set_threshold(pick_primary_role_factors.len() as u8);
            pick_primary_role_factors.into_iter().for_each(|f| {
                shield_builder.add_factor_source_to_primary_threshold(f);
            });

            let stats_for_testing = shield_builder.auto_assign_factors_to_recovery_and_confirmation_based_on_primary(
                all_factors_in_profile,
            )?;

            let built = shield_builder.build().map_err(|e| {
                CommonError::AutomaticShieldBuildingFailure {
                    underlying: format!("{:?}", e),
                }
            })?;

            Ok((built, stats_for_testing))
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

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 0,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

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

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

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

    #[test]
    fn two_contacts() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_trusted_contact_frank(),
                FactorSource::sample_trusted_contact_grace(),
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            IndexSet::just(FactorSource::sample_device_babylon().id()),
        );

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_device()],
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_trusted_contact(),
                FactorSourceID::sample_trusted_contact_other(),
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other(),
            ],)
        );
    }

    #[test]
    fn two_information() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_password(),
                FactorSource::sample_password_other(),
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            IndexSet::just(FactorSource::sample_device_babylon().id()),
        );

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 0,
                    limit: 1
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_device()],
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other(),
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_password(),
                FactorSourceID::sample_password_other(),
            ],)
        );
    }

    #[test]
    fn one_info_one_contact() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_password(),
                FactorSource::sample_trusted_contact_frank(),
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            IndexSet::just(FactorSource::sample_device_babylon().id()),
        );

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                1,
                [FactorSourceID::sample_device()],
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_trusted_contact(),
                FactorSourceID::sample_ledger(),
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_password(),
                FactorSourceID::sample_ledger_other(),
            ],)
        );
    }

    #[test]
    fn arculus_and_ledger_mixed_with_one_info_and_one_contact() {
        let res = SUT::test(
            IndexSet::from_iter([
                FactorSource::sample_password(),
                FactorSource::sample_trusted_contact_frank(),
                FactorSource::sample_device_babylon(),
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_ledger(),
                FactorSource::sample_arculus(),
                FactorSource::sample_arculus_other(),
                FactorSource::sample_ledger_other(),
            ]),
            IndexSet::from_iter([
                FactorSource::sample_device_babylon().id(),
                FactorSource::sample_ledger().id(),
            ]),
        );

        let (shield, stats) = res.unwrap();

        pretty_assertions::assert_eq!(
            stats.calls_to_assign_unsupported_factor,
            vec![
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Recovery,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
                CallsToAssignUnsupportedFactor {
                    unsupported: FactorSelector::Category(Custodian),
                    role: Confirmation,
                    number_of_factors_for_role: 1,
                    limit: 2
                },
            ]
        );

        let matrix = shield.matrix_of_factors;

        pretty_assertions::assert_eq!(
            matrix.primary(),
            &PrimaryRoleWithFactorSourceIds::with_factors(
                2,
                [
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_ledger()
                ],
                []
            )
        );

        pretty_assertions::assert_eq!(
            matrix.recovery(),
            &RecoveryRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_trusted_contact(),
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_arculus_other(),
                FactorSourceID::sample_ledger_other(),
            ],)
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation(),
            &ConfirmationRoleWithFactorSourceIds::override_only([
                FactorSourceID::sample_password(),
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other(),
            ],)
        );
    }
}
