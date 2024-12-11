use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    #[allow(dead_code)]
    #[doc(hidden)]
    hidden: HiddenConstructor,
    remaining_available_factors: IndexSet<FactorSource>,
    primary: IndexSet<FactorSourceID>,
    recovery: IndexSet<FactorSourceID>,
    confirmation: IndexSet<FactorSourceID>,
    shield_builder: SecurityShieldBuilder,
}

use FactorSourceCategory::*;
use RoleKind::*;
impl AutomaticShieldBuilder {
    fn new(
        available_factors: IndexSet<FactorSource>,
        user_selected_primary: IndexSet<FactorSourceID>,
    ) -> Self {
        assert!(
            user_selected_primary
                .difference(
                    &available_factors
                        .iter()
                        .map(|f| f.id())
                        .collect::<IndexSet<_>>()
                )
                .collect::<IndexSet<_>>()
                .is_empty(),
            "All user_selected_primary must be in available_factors"
        );
        Self {
            hidden: HiddenConstructor,
            remaining_available_factors: available_factors,
            primary: user_selected_primary,
            recovery: IndexSet::new(),
            confirmation: IndexSet::new(),
            shield_builder: SecurityShieldBuilder::new(),
        }
    }

    fn find_primary_role_candidates(
        all: &IndexSet<FactorSource>,
    ) -> IndexSet<FactorSource> {
        let ephemeral = SecurityShieldBuilder::new();
        let factor_source_ids =
            all.iter().map(|f| f.id()).collect::<IndexSet<_>>();
        ephemeral.validation_for_addition_of_factor_source_to_primary_threshold_for_each(factor_source_ids.into_iter().collect_vec()).into_iter().filter(|vs| match vs.validation {
            Ok(_) => true,
            Err(RoleBuilderValidation::NotYetValid(_)) => true,
            Err(RoleBuilderValidation::BasicViolation(_)) |  Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
        }).filter_map(|vs| all.iter().find(|f| f.id() == vs.factor_source_id))
        .cloned()
        .collect::<IndexSet<_>>()
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

        let target_factors = match target_role {
            Primary => &mut self.primary,
            Recovery => &mut self.recovery,
            Confirmation => &mut self.confirmation,
        };

        self.remaining_available_factors
            .retain(|f| !factors_to_add.contains(&f.id()));

        target_factors.extend(factors_to_add);
    }

    fn factor_for_role(&self, role: RoleKind) -> &IndexSet<FactorSourceID> {
        match role {
            Primary => &self.primary,
            Recovery => &self.recovery,
            Confirmation => &self.confirmation,
        }
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
        self.factor_for_role(role_kind).len() as u8
    }

    fn assign_factors_of_category_to_primary(
        &mut self,
        category: FactorSourceCategory,
        quantity_to_add: Quantity,
    ) {
        self.assign_factors_of_category(Primary, category, quantity_to_add);
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
}

impl AutomaticShieldBuilder {
    fn _do_build_shield(&self) -> Result<SecurityStructureOfFactorSourceIds> {
        let builder = &self.shield_builder;
        builder.set_threshold(self.count_factors_for_role(Primary));
        self.primary.iter().for_each(|&f| {
            builder.add_factor_source_to_primary_threshold(f);
        });
        self.recovery.iter().for_each(|&f| {
            builder.add_factor_source_to_recovery_override(f);
        });
        self.confirmation.iter().for_each(|&f| {
            builder.add_factor_source_to_confirmation_override(f);
        });

        builder.build().map_err(|e| {
            CommonError::AutomaticShieldBuildingFailure {
                underlying: format!("{:?}", e),
            }
        })
    }

    fn _build_shield(&mut self) -> Result<SecurityStructureOfFactorSourceIds> {
        // 📒 "If the user only chose 1 factor for PRIMARY, remove that factor from the list (it cannot be used elsewhere - otherwise it can)."
        {
            if self.count_factors_for_role(Primary) == 1
                && let Some(only_primary_factor) = self.primary.iter().next()
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
            // until there is at least 1 factor source in RECOVERY."
            // ❓ SHOULD WE ALWAYS DO THIS? OR ONLY IF count_factors_for_role(Recovery) < 1?
            self.assign_one_hardware_factor(Recovery);
            // ❓ WHAT TO DO IF THERE WAS NONE?

            // 📒 "Add any Hardware (Ledger, Arculus, Yubikey) factors in the list, starting with the most recently used, to CONFIRMATION until there is at least 1 factor factors in CONFIRMATION."
            // ❓ SHOULD WE ALWAYS DO THIS? OR ONLY IF count_factors_for_role(Confirmation) < 1?
            self.assign_one_hardware_factor(Confirmation);
            // ❓ WHAT TO DO IF THERE WAS NONE?
        };

        // 📒 "Distribute to try to get at least 1 RECOVERY and then 1 CONFIRMATION"
        distribute_custodian_and_hardware_to_non_primary();

        // 📒 "Distribute to try to get up to 2 RECOVERY and then 2 CONFIRMATION factors if possible"
        distribute_custodian_and_hardware_to_non_primary();

        // 📒 "Fill in any remaining other factors to increase reliability of being able to recover"
        {
            // 📒 "Add any (and all) remaining Hardware or Custodian factors in the list to RECOVERY."
            self.assign_factors_of_category_to_primary(Hardware, Quantity::All);

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

        self._do_build_shield()
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

pub struct AutoShieldBuilderValidatorOfPickedPrimaryFactors {
    #[allow(dead_code)]
    #[doc(hidden)]
    hidden: HiddenConstructor,
}
impl AutoShieldBuilderValidatorOfPickedPrimaryFactors {
    fn new() -> Self {
        Self {
            hidden: HiddenConstructor,
        }
    }

    pub fn validate_picked(
        &self,
        picked: IndexSet<FactorSourceID>,
    ) -> Result<ValidatedPrimary> {
        let ephemeral = SecurityShieldBuilder::new();
        ephemeral.set_threshold(picked.len() as u8);
        picked.iter().for_each(|f| {
            ephemeral.add_factor_source_to_primary_threshold(*f);
        });
        if let Some(invalid_reason) = ephemeral.validate_primary_role() {
            Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: format!(
                    "Invalid picked for primary: {:?}",
                    invalid_reason
                ),
            })
        } else {
            // valid!
            let valid = unsafe { ValidatedPrimary::new(picked) };
            Ok(valid)
        }
    }
}

pub struct ValidatedPrimary {
    validated_picked: IndexSet<FactorSourceID>,
}
impl ValidatedPrimary {
    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for specification
    /// of unsafe - as in application **unsecure** - factors for PrimaryRole, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn new(validated_picked: IndexSet<FactorSourceID>) -> Self {
        Self { validated_picked }
    }

    pub fn validated_picked(&self) -> IndexSet<FactorSourceID> {
        self.validated_picked.clone()
    }
}

impl AutomaticShieldBuilder {
    pub async fn build<Fut>(
        all_factors: IndexSet<FactorSource>,
        pick_primary_role_factors: impl Fn(
            IndexSet<FactorSource>,
            AutoShieldBuilderValidatorOfPickedPrimaryFactors,
        ) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = ValidatedPrimary>,
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
        let candidates = Self::find_primary_role_candidates(&all_factors);
        let validated_picked: IndexSet<FactorSourceID> =
            pick_primary_role_factors(
                candidates,
                AutoShieldBuilderValidatorOfPickedPrimaryFactors::new(),
            )
            .await
            .validated_picked;

        let auto_builder = Self::new(all_factors, validated_picked);
        auto_builder.build_shield()
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
        #[allow(dead_code)]
        async fn test_non_validated<Fut>(
            pick_primary_role_factors: impl Fn(
                IndexSet<FactorSource>,
                AutoShieldBuilderValidatorOfPickedPrimaryFactors,
            ) -> Fut,
        ) -> Result<SecurityStructureOfFactorSourceIDs>
        where
            Fut: Future<Output = ValidatedPrimary>,
        {
            SUT::build(FactorSource::sample_all(), pick_primary_role_factors)
                .await
        }

        async fn test_valid(
            pick_primary_role_factors: impl Fn(
                IndexSet<FactorSource>,
            )
                -> IndexSet<FactorSourceID>,
        ) -> Result<SecurityStructureOfFactorSourceIDs> {
            SUT::build(
                FactorSource::sample_all(),
                async |candidates, validator| {
                    let picked = pick_primary_role_factors(candidates);
                    validator.validate_picked(picked).unwrap()
                },
            )
            .await
        }
    }

    #[actix_rt::test]
    async fn primary_role_candidates() {
        let shield_builder = SecurityShieldBuilder::new();

        let expected =  shield_builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                FactorSource::sample_all().into_iter().map(|f| f.id()).collect_vec()
            )
            .into_iter()
            .filter(|f| matches!(f.validation, Err(RoleBuilderValidation::NotYetValid(_))) || f.validation.is_ok())
            .map(|vs| vs.factor_source_id)
            .collect_vec();

        let called = Arc::new(Mutex::new(false));

        let _ = SUT::test_valid(|candidates| {
            *called.lock().unwrap() = true;
            pretty_assertions::assert_eq!(
                candidates.into_iter().map(|f| f.id()).collect_vec(),
                expected
            );

            IndexSet::just(FactorSourceID::sample_device())
        })
        .await
        .unwrap();

        assert!(*called.lock().unwrap());
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_first() {
        let built = SUT::test_valid(|xs| {
            IndexSet::just(xs.iter().map(|x| x.id()).next().unwrap())
        })
        .await
        .unwrap();
        assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &vec![FactorSource::sample_all().first().unwrap().id()]
        );
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_last() {
        let built = SUT::test_valid(|xs| {
            IndexSet::just(xs.iter().map(|x| x.id()).last().unwrap())
        })
        .await
        .unwrap();
        assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &vec![FactorSource::sample_all()
                .into_iter()
                .filter(|f| f.category() == FactorSourceCategory::Identity)
                .last()
                .unwrap()
                .id()]
        );
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_two() {
        let factors = IndexSet::from_iter([
            FactorSourceID::sample_ledger(),
            FactorSourceID::sample_device(),
        ]);
        let built = SUT::test_valid(|_| factors.clone()).await.unwrap();
        pretty_assertions::assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &factors.into_iter().collect_vec()
        );
    }

    #[actix_rt::test]
    #[should_panic]
    async fn selection_of_primary_invalid_only_one_password() {
        let _ = SUT::test_valid(
            |_| IndexSet::just(FactorSourceID::sample_password()), // invalid for primary
        )
        .await;
    }

    #[actix_rt::test]
    async fn empty_factors_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(IndexSet::new(), |_, _| {
            *called.lock().unwrap() = true;
            ready(unsafe {
                ValidatedPrimary::new(IndexSet::just(
                    FactorSourceID::sample_device(),
                ))
            })
        })
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn one_factors_is_not_enough_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([FactorSource::sample_device()]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn two_factors_is_not_enough_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device(),
                FactorSource::sample_ledger(),
            ]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn two_device_factor_source_and_one_ledger_is_not_sufficient() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_ledger(),
            ]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn one_device_factor_source_and_two_ledger_is_ok_when_primary_uses_one_ledger(
    ) {
        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            |_, _| {
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSource::sample_ledger().id(),
                    ))
                })
            },
        )
        .await;

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
}

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
